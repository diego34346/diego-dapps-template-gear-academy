#![no_std]

use codec::{Decode, Encode};
use gmeta::{In, InOut, Metadata, Out};
use gstd::{
    exec::{self, block_height},
    prelude::*,
    ActorId,
};
use scale_info::TypeInfo;

#[derive(Default, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct Tamagotchi {
    pub name: String,
    pub age: u64,
    pub owner: ActorId,
    pub fed: u64,
    pub fed_block: u64,
    pub entertained: u64,
    pub entertained_block: u64,
    pub slept: u64,
    pub slept_block: u64,
}

const HUNGER_PER_BLOCK: u64 = 1;
const BOREDOM_PER_BLOCK: u64 = 2;
const ENERGY_PER_BLOCK: u64 = 2;
const FILL_PER_FEED: u64 = 1000;
const FILL_PER_ENTERTAINMENT: u64 = 1000;
const FILL_PER_SLEEP: u64 = 1000;

impl Tamagotchi {
    // ...

    // Función para actualizar los niveles y los bloques correspondientes
    fn update_levels(&mut self) {
        // Calculate the number of blocks since the last update
        let blocks_since_last_update = (block_height() as u64) - self.fed_block;

        // Update the feeding level
        self.fed = self
            .fed
            .saturating_sub(blocks_since_last_update * HUNGER_PER_BLOCK);

        // Update the entertainment level
        self.entertained = self
            .entertained
            .saturating_sub(blocks_since_last_update * BOREDOM_PER_BLOCK);

        // Update the sleep level
        self.slept = self
            .slept
            .saturating_sub(blocks_since_last_update * ENERGY_PER_BLOCK);

        // Update the corresponding blocks
        self.fed_block = block_height() as u64;
        self.entertained_block = block_height() as u64;
        self.slept_block = block_height() as u64;
    }

    // Functions to feed, entertain, and sleep the pet
    pub fn feed(&mut self) {
        self.update_levels();
        self.fed = (self.fed + FILL_PER_FEED).min(10000);
        self.fed_block = block_height() as u64;

        // Manually emit the Fed event
        // Assuming you have a mechanism to handle events
    }

    pub fn entertain(&mut self) {
        self.update_levels();
        self.entertained = (self.entertained + FILL_PER_ENTERTAINMENT).min(10000);
        self.entertained_block = block_height() as u64;

        // Manually emit the Entertained event
        // Assuming you have a mechanism to handle events
    }

    pub fn sleep(&mut self) {
        self.update_levels();
        self.slept = (self.slept + FILL_PER_SLEEP).min(10000);
        self.slept_block = exec::block_height() as u64;

        // Manually emit the Slept event
        // Assuming you have a mechanism to handle events
    }
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum TmgAction {
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
    Name(String),
    Age(u64),
    Fed,
    Entertained,
    Slept,
}

pub struct ProgramMetadata;

impl Metadata for ProgramMetadata {
    type Init = In<String>;
    type Handle = InOut<TmgAction, TmgEvent>;
    type State = Out<Tamagotchi>;
    type Reply = ();
    type Others = ();
    type Signal = ();
}
