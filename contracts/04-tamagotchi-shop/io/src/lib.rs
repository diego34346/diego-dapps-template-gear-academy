#![no_std]

use codec::{Decode, Encode};
use gmeta::{In, InOut, Metadata, Out};
#[allow(unused_imports)]
use gstd::{exec, msg, prelude::*, ActorId};
use scale_info::TypeInfo;
use sharded_fungible_token_io::*;
use store_io::*;

#[derive(Default, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct Tamagotchi {
    // TODO: 1️⃣ Add `name` and `age` fields
    pub name: String,
    pub date_of_birth: u64,
    pub owner: ActorId,
    pub fed: u64,
    pub fed_block: u64,
    pub entertained: u64,
    pub entertained_block: u64,
    pub slept: u64,
    pub slept_block: u64,
    pub approved_account: Option<ActorId>,
    pub ft_contract_id: ActorId,
    pub transaction_id: u64,
    pub approve_transaction: Option<(TransactionId, ActorId, u128)>,
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum TmgAction {
    // TODO: 2️⃣ Add `Name` and `Age` actions that set the name and age
    Name,
    Age,
    Feed,
    Entertain,
    Sleep,
    Transfer(ActorId),
    Approve(ActorId),
    RevokeApproval,
    SetFTokenContract(ActorId),
    ApproveTokens {
        account: ActorId,
        amount: u128,
    },
    BuyAttribute {
        store_id: ActorId,
        attribute_id: AttributeId,
    },
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum TmgEvent {
    // TODO: 3️⃣ Add `Name` and `Age` events that return the name and age
    Name(String),
    Age(u64),
    Fed,
    Entertained,
    Slept,
    Transferred(ActorId),
    Approved(ActorId),
    ApprovalRevoked,
    FTokenContractSet,
    TokensApproved { account: ActorId, amount: u128 },
    ApprovalError,
    AttributeBought(AttributeId),
    CompletePrevPurchase(AttributeId),
    ErrorDuringPurchase,
}

impl Tamagotchi {
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

pub struct ProgramMetadata;

// TODO: 4️⃣ Fill `Init`, `Handle`, and `State` types
impl Metadata for ProgramMetadata {
    type Init = In<String>;
    type Handle = InOut<TmgAction, TmgEvent>;
    type State = Out<Tamagotchi>;
    type Reply = ();
    type Others = ();
    type Signal = ();
}
