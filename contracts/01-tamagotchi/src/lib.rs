#![no_std]
use gstd::{exec, msg, prelude::*};
use tamagotchi_io::*;

static mut TAMAGOTCHI: Option<Tamagotchi> = None;

#[no_mangle]
extern fn init() {
    let name: String = msg::load().expect("Failed to decode Tamagotchi name");
    let age = exec::block_timestamp();

    let tmg = Tamagotchi { name, age };

    unsafe {
        TAMAGOTCHI = Some(tmg);
    }
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
            let age = exec::block_timestamp() - tmg.age;
            msg::reply(TmgEvent::Age(age), 0).expect("Error in a reply `TmgEvent::Age`");
        }
        TmgAction::Feed => {
            // Implementa la lógica para alimentar al Tamagotchi
            tmg.fed += FILL_PER_FEED;
            tmg.fed_block = exec::block_height();
            msg::reply(TmgEvent::Fed, 0).expect("Error al responder `TmgEvent::Fed`");
        }
        TmgAction::Entertain => {
            // Implementa la lógica para entretener al Tamagotchi
            tmg.entertained += FILL_PER_ENTERTAINMENT;
            tmg.entertained_block = exec::block_height();
            msg::reply(TmgEvent::Entertained, 0)
                .expect("Error al responder `TmgEvent::Entertained`");
        }
        TmgAction::Sleep => {
            // Implementa la lógica para hacer dormir al Tamagotchi
            tmg.slept += FILL_PER_SLEEP;
            tmg.slept_block = exec::block_height();
            msg::reply(TmgEvent::Slept, 0).expect("Error al responder `TmgEvent::Slept`");
        }
    };
}

const HUNGER_PER_BLOCK: u64 = 1;
const BOREDOM_PER_BLOCK: u64 = 2;
const ENERGY_PER_BLOCK: u64 = 2;
const FILL_PER_FEED: u64 = 1000;
const FILL_PER_ENTERTAINMENT: u64 = 1000;
const FILL_PER_SLEEP: u64 = 1000;

//logica por niveles

fn calculate_levels(tmg: &mut Tamagotchi) {
    let current_block = exec::block_height();

    //fed
    let hunger_blocks = current_block - tmg.fed_block;
    tmg.fed = tmg.fed.saturating_sub(hunger_blocks * HUNGER_PER_BLOCK);

    //entertained
    let boredom_blocks = current_block - tmg.entertained_block;
    tmg.entertained = tmg
        .entertained
        .saturating_sub(boredom_blocks * BOREDOM_PER_BLOCK);

    // slept
    let energy_blocks = current_block - tmg.slept_block;
    tmg.slept = tmg.slept.saturating_sub(energy_blocks * ENERGY_PER_BLOCK);

    // Actualiza
    tmg.fed_block = current_block;
    tmg.entertained_block = current_block;
    tmg.slept_block = current_block;
}

#[no_mangle]
extern fn state() {
    let tmg = unsafe { TAMAGOTCHI.take().expect("Error in taking current state") };
    msg::reply(tmg, 0).expect("Failed to reply state");
}
