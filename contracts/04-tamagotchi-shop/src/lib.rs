#![no_std]

#[allow(unused_imports)]
use codec::{Decode, Encode};
#[allow(unused_imports)]
use gstd::{exec, msg, prelude::*, ActorId};
use store_io::*;
use tamagotchi_shop_io::*;

pub const HUNGER_PER_BLOCK: u64 = 1;
pub const BOREDOM_PER_BLOCK: u64 = 2;
pub const ENERGY_PER_BLOCK: u64 = 2;
pub const FILL_PER_FEED: u64 = 1000;
pub const FILL_PER_ENTERTAINMENT: u64 = 1000;
pub const FILL_PER_SLEEP: u64 = 1000;

static mut TAMAGOTCHI: Option<Tamagotchi> = None;

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
        ft_contract_id: ActorId::from(1),
        transaction_id: 1,
        approve_transaction: None,
    };
    unsafe {
        TAMAGOTCHI = Some(tmg);
    };
}

#[no_mangle]
async fn handle() {
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
            msg::reply(TmgEvent::FTokenContractSet, 0).expect("Error in sending reply");
        }
        TmgAction::ApproveTokens { account, amount } => {
            tmg.approve_tokens(account, amount);
        }
        TmgAction::BuyAttribute {
            store_id,
            attribute_id,
        } => {
            let result = msg::send_for_reply_as::<_, StoreEvent>(
                store_id,
                StoreAction::BuyAttribute { attribute_id },
                0,
                0,
            )
            .expect("Error al enviar mensaje `StoreAction::BuyAttribute`")
            .await
            .expect("Error al decodificar 'StoreEvent'");

            match result {
                StoreEvent::AttributeSold { success } => {
                    if success {
                        msg::reply(TmgEvent::AttributeBought(attribute_id), 0)
                            .expect("Error al enviar respuesta `TmgEvent::AttributeBought`");
                    } else {
                        msg::reply(TmgEvent::ErrorDuringPurchase, 0)
                            .expect("Error al enviar respuesta `TmgEvent::ErrorDuringPurchase`");
                    }
                }
                _ => {
                    msg::reply(TmgEvent::ErrorDuringPurchase, 0)
                        .expect("Error al enviar respuesta `TmgEvent::ErrorDuringPurchase`");
                }
            }
        }
    }
}

#[no_mangle]
extern fn state() {
    let tmg = unsafe { TAMAGOTCHI.take().expect("Unexpected error in taking state") };
    msg::reply(tmg, 0).expect("Failed to share state");
}
