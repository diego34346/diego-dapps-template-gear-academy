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
