#![no_std]
use gstd::{msg, prelude::*, collections::BTreeMap};
use tamagotchi_battle_io::*;

static mut TAMAGOTCHI_BATTLE: Option<Battle> = None;

#[no_mangle]
extern "C" fn init() {
    let BattleInit { tmg_store_id } = msg::load()
        .expect("Unable to decode CodeId of the Escrow program");
    let tamagotchi_battle = Battle {
        tmg_store_id,
        weapons_data: BTreeMap::from([
            (SWORD_ID, SWORD_TMG),
            (SWORD_WOOD_ID, SWORD_TMG_WOOD),
            (SHOTGUN_ID, SHOTGUN_POWER),
            (RPG_ID, RPG_POWER),
        ]),
        shields_data: BTreeMap::from([
            (SHIELD_ID, SHIELD),
        ]),
        ..Default::default()
    };
    unsafe { TAMAGOTCHI_BATTLE = Some(tamagotchi_battle) };
}   

#[gstd::async_main]
async fn main() {
    let action: BattleAction = msg::load()
        .expect("Unable to decode `FactoryAction`");
    let tmg_battle = unsafe {
        TAMAGOTCHI_BATTLE.get_or_insert(Default::default())
    };
    
    match action {
        BattleAction::Register {
            tamagotchi_id,
            attributes
        } => {
            tmg_battle.initiate_registration(&tamagotchi_id, attributes).await;            
        },
        BattleAction::Move(direction) => {
            tmg_battle.execute_move(direction);
        },
        BattleAction::UpdateInfo => {
            tmg_battle.update_game_info().await;
        },
        BattleAction::SendNewAttributesToNextRound {
            new_attributes,
        } => {
            tmg_battle.update_tamagotchi_attributes(new_attributes);
        },
        BattleAction::StartNewGame => {
            tmg_battle.reset_game();
        },
        BattleAction::ReserveGas { 
            reservation_amount, 
            duration 
        } => {
            tmg_battle.initiate_reservation(reservation_amount, duration);
        }
    }
}

#[no_mangle]
extern fn state() {
    msg::reply(state_ref(), 0)
        .expect("Failed to share state");
}

fn state_ref() -> &'static Battle {
    let state = unsafe { TAMAGOTCHI_BATTLE.as_ref() };
    debug_assert!(state.is_some(), "State is not initialized");
    unsafe { state.unwrap_unchecked() }
}