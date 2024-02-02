#![no_std]

use codec::{Decode, Encode};
use gmeta::{In, InOut, Metadata, Out};
#[allow(unused_imports)]
use gstd::{exec, msg, prelude::*, ActorId};
use scale_info::TypeInfo;

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
    type Handle = InOut<TmgAction, TmgEvent>;
    type State = Out<Tamagotchi>;
}

pub fn blocks_height() -> u64 {
    exec::block_height() as u64
}

pub fn updated_field_value(
    field: u64,
    field_block: u64,
    value_per_block: u64,
    blocks_height: u64,
) -> u64 {
    let total_value_to_rest = (blocks_height - field_block) * value_per_block;
    if field >= total_value_to_rest {
        // If the given value of the tamagotchi is greater than the value to be
        // subtracted after a certain number of blocks, the update value is
        // returned
        field - total_value_to_rest
    } else {
        // If not, the given value is smaller, causing a negative result, one
        // is returned instead.
        1
    }
}

pub fn update_field(field: u64, increase_value: u64) -> u64 {
    let field = field + increase_value;
    field.min(10_000)
}
