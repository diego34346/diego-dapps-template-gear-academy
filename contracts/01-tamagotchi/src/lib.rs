#![no_std]

#[allow(unused_imports)]
use codec::{Decode, Encode};
#[allow(unused_imports)]
use gstd::{exec, msg, prelude::*, ActorId};
use tamagotchi_io::*;

static mut TAMAGOTCHI: Option<Tamagotchi> = None;

#[no_mangle]
extern fn init() {
    let initname = msg::load().expect("Failed to decode Tamagotchi name");
    let birthdate = exec::block_timestamp();

    let tmg = Tamagotchi {
        name: initname,
        date_of_birth: birthdate,
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
    }
}

#[no_mangle]
extern fn state() {
    let tmg = unsafe { TAMAGOTCHI.take().expect("Unexpected error in taking state") };
    msg::reply(tmg, 0).expect("Failed to share state");
}
