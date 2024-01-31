#![no_std]
use gstd::{msg, prelude::*, exec};
use gstd::exec::block_height;
use tamagotchi_interaction_io::*;



static mut TAMAGOTCHI: Option<Tamagotchi> = None;

#[no_mangle]
extern fn init() {
    let name: String = msg::load().expect("Failed to decode Tamagotchi name");
    let age = exec::block_timestamp();
    let block_height=block_height() as u64;

    let tmg = Tamagotchi {
        name, 
        age,
        owner: msg::source(),
        fed: 10000,  
        fed_block: block_height,
        entertained: 10000,  
        entertained_block:block_height,
        slept: 10000,  
        slept_block: block_height,
    };

    unsafe {
        TAMAGOTCHI = Some(tmg);
    }
    msg::reply("successful initialization!", 0)
    .expect("error in reply");
}

#[no_mangle]
extern "C" fn handle() {
    let action: TmgAction = msg::load().expect("Unable to decode `TmgAction`");
    let tmg = unsafe { TAMAGOTCHI.get_or_insert(Default::default()) };
    match action {
        TmgAction::Name => {
            msg::reply(TmgEvent::Name(tmg.name.clone()), 0)
                .expect("Error in a reply `TmgEvent::Name`");
        }
        TmgAction::Age => {
            let age = exec::block_timestamp() - tmg.age;
            msg::reply(TmgEvent::Age(age), 0)
                .expect("Error in a reply `TmgEvent::Age`");
        }
        TmgAction::Feed => {
            tmg.feed();

            
            msg::reply(TmgEvent::Fed, 0)
                .expect("Error sending tamagotchi variant 'Fed'");
        },
        TmgAction::Entertain => {
            tmg.entertain();


            msg::reply(TmgEvent::Entertained, 0)
                .expect("Error sending tamagotchi variant 'Entertained'");  
        },
        TmgAction::Sleep => {
            tmg.sleep();


            msg::reply(TmgEvent::Slept, 0)
                .expect("Error sending tamagotchi variant 'Slept'");  
        },
    };
}



//logica por niveles



#[no_mangle]
extern fn state() {
    let mut tmg = unsafe { TAMAGOTCHI.get_or_insert(Default::default()) };
    msg::reply(&tmg, 0).expect("Failed to reply state");
}