#![no_std]

#[allow(unused_imports)]
use codec::{Decode, Encode};
#[allow(unused_imports)]
use gstd::{exec, msg, prelude::*, ActorId};
use sharded_fungible_token_io::{FTokenAction, FTokenEvent, LogicAction};
use store_io::{AttributeId, StoreAction, StoreEvent, TransactionId};
use tamagotchi_nft_io::*;

pub const HUNGER_PER_BLOCK: u64 = 1;
pub const BOREDOM_PER_BLOCK: u64 = 2;
pub const ENERGY_PER_BLOCK: u64 = 2;
pub const FILL_PER_FEED: u64 = 1000;
pub const FILL_PER_ENTERTAINMENT: u64 = 1000;
pub const FILL_PER_SLEEP: u64 = 1000;

static mut TAMAGOTCHI: Option<Tamagotchi> = None;

impl Tamagotchi {
    pub fn sleep(&mut self) {
        let blocks_height = blocks_height();
        let updated_rested = updated_field_value(
            self.rested,
            self.rested_block,
            ENERGY_PER_BLOCK,
            blocks_height,
        );
        self.rested = update_field(updated_rested, FILL_PER_SLEEP);
        self.rested_block = blocks_height;
    }

    pub fn feed(&mut self) {
        let blocks_height = blocks_height();
        let updated_feed =
            updated_field_value(self.fed, self.fed_block, HUNGER_PER_BLOCK, blocks_height);
        self.fed = update_field(updated_feed, FILL_PER_FEED);
        self.fed_block = blocks_height;
    }

    pub fn play(&mut self) {
        let blocks_height = blocks_height();
        let updated_entertainer = updated_field_value(
            self.entertained,
            self.entertained_block,
            BOREDOM_PER_BLOCK,
            blocks_height,
        );
        self.entertained = update_field(updated_entertainer, FILL_PER_ENTERTAINMENT);
        self.entertained_block = blocks_height;
    }

    pub fn is_owner_or_approved(&self, user: &ActorId) -> bool {
        if self.owner == *user {
            return true;
        }
        if self.approved_account == Some(*user) {
            return true;
        }
        false
    }

    pub async fn buy_attribute(&mut self, store_id: ActorId, attribute_id: AttributeId) {
        let response = msg::send_for_reply_as::<_, StoreEvent>(
            store_id,
            StoreAction::BuyAttribute { attribute_id },
            0,
            0,
        )
        .expect("Error in sending a message `FTokenAction::Message`")
        .await
        .expect("Error in decoding 'FTokenEvent'");
        msg::reply(response, 0).expect("Error in sending reply 'StoreEvent' event");
    }

    pub async fn approve_tokens(&mut self, account: ActorId, amount: u128) {
        let (transaction_id, account, amount) = if let Some((
            prev_transaction_id,
            prev_account,
            prev_amount,
        )) = self.approve_transaction
        {
            if prev_account != account && prev_amount != amount {
                msg::reply(TmgEvent::ApprovalError, 0)
                    .expect("Error in sending a reply `TmgEvent::ApprovalError`");
                return;
            }
            (prev_transaction_id, prev_account, prev_amount)
        } else {
            let current_transaction_id = self.transaction_id;
            self.transaction_id = self.transaction_id.wrapping_add(1);
            self.approve_transaction = Some((current_transaction_id, account, amount));
            (current_transaction_id, account, amount)
        };

        let result_transaction = msg::send_for_reply_as::<_, FTokenEvent>(
            self.ft_contract_id,
            FTokenAction::Message {
                transaction_id,
                payload: LogicAction::Approve {
                    approved_account: account,
                    amount,
                },
            },
            0,
            0,
        )
        .expect("Error in sending a message `FTokenAction::Message`")
        .await
        .expect("Error in decoding 'FTokenEvent'");

        if result_transaction != FTokenEvent::Ok {
            msg::reply(TmgEvent::ApprovalError, 0)
                .expect("Error in sending a reply `TmgEvent::ApprovalError`");
            return;
        }

        let response = TmgEvent::TokensApproved { account, amount };
        msg::reply(response, 0).expect("Error in sending a reply `TmgEvent::ApprovalError`");
    }
}

#[no_mangle]
extern fn init() {
    let initname = msg::load().expect("Failed to decode Tamagotchi name");
    let birthdate = exec::block_timestamp();

    let tmg = Tamagotchi {
        name: initname,
        date_of_birth: birthdate,
        owner: Default::default(),
        fed: 1,
        fed_block: exec::block_height() as u64,
        entertained: 1,
        entertained_block: exec::block_height() as u64,
        slept: 1,
        slept_block: exec::block_height() as u64,
        approved_account: None,
    };
    unsafe {
        TAMAGOTCHI = Some(tmg);
    };
}

#[no_mangle]
extern fn handle() {
    let action: TmgAction = msg::load().expect("Unable to decode `TmgAction`");
    let tmg = unsafe { TAMAGOTCHI.get_or_insert(Default::default()) };
    match action {
        TmgAction::Name => {
            msg::reply(TmgEvent::Name(tmg.name.clone()), 0)
                .expect("Error in a reply `TmgEvent::Name`");
        }
        TmgAction::Age => {
            let age = exec::block_timestamp() - tmg.date_of_birth;
            msg::reply(TmgEvent::Age(age), 0).expect("Error in a reply `TmgEvent::Age`");
        }
        TmgAction::Feed => {
            tmg.fed_block = exec::block_height() as u64;
            if exec::block_height() as u64 * HUNGER_PER_BLOCK > tmg.fed {
                tmg.fed = 1;
            } else {
                tmg.fed -= exec::block_height() as u64 * HUNGER_PER_BLOCK;
            }
            tmg.fed += FILL_PER_FEED;
            msg::reply(TmgEvent::Fed, 0).expect("Not fed correctly");
        }
        TmgAction::Entertain => {
            tmg.entertained_block = exec::block_height() as u64;
            if exec::block_height() as u64 * BOREDOM_PER_BLOCK > tmg.entertained {
                tmg.entertained = 1;
            } else {
                tmg.entertained -= exec::block_height() as u64 * BOREDOM_PER_BLOCK;
            }
            tmg.entertained += FILL_PER_ENTERTAINMENT;
            msg::reply(TmgEvent::Entertained, 0).expect("Not entertained correctly");
        }
        TmgAction::Sleep => {
            tmg.slept_block = exec::block_height() as u64;
            if exec::block_height() as u64 * ENERGY_PER_BLOCK > tmg.slept {
                tmg.slept = 1;
            } else {
                tmg.slept -= exec::block_height() as u64 * ENERGY_PER_BLOCK;
            }
            tmg.slept_block += FILL_PER_SLEEP;
            msg::reply(TmgEvent::Slept, 0).expect("Not slept correctly");
        }
        TmgAction::Transfer(actor_id) => {
            let source_id = msg::source();
            let mut owner_transfered = false;
            if tmg.owner == source_id {
                tmg.owner = actor_id;
                owner_transfered = true;
            }
            if tmg.approved_account == Some(source_id) {
                tmg.owner = actor_id;
                owner_transfered = true;
            }
            if owner_transfered {
                msg::reply(TmgEvent::Transferred(actor_id), 0).expect("Error in sending reply");
            }
        }
        TmgAction::Approve(actor_id) => {
            let source_id = msg::source();
            if tmg.owner == source_id {
                tmg.approved_account = Some(actor_id);
                msg::reply(TmgEvent::Approved(actor_id), 0).expect("Error in sending reply");
            }
        }
        TmgAction::RevokeApproval => {
            let source_id = msg::source();
            if tmg.owner == source_id {
                tmg.approved_account = None;
                msg::reply(TmgEvent::ApprovalRevoked, 0).expect("Error in sending reply");
            }
        }
        TmgAction::SetFTokenContract(ft_contract_id) => {
            tmg.ft_contract_id = ft_contract_id;
            msg::reply(TmgEvent::FTokenContractSet, 0)
                .expect("Error setting Fungible Token contract");
        }
        TmgAction::ApproveTokensForStore { store_id, amount } => {
            tmg.approve_tokens(&store_id, amount).await;
        }
        TmgAction::BuyAttributeFromStore {
            store_id,
            attribute_id,
        } => {
            tmg.buy_attribute_from_store(store_id, attribute_id).await;
        }
    }
}

#[no_mangle]
extern fn state() {
    let tmg = unsafe { TAMAGOTCHI.take().expect("Unexpected error in taking state") };
    msg::reply(tmg, 0).expect("Failed to share state");
}
