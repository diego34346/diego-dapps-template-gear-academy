#![no_std]
#[allow(unused_imports)]
use gstd::{exec, msg, prelude::*, prog::ProgramGenerator, ActorId, CodeId};
use tamagotchi_army_io::*;

static mut TAMAGOTCHI_FACTORY: Option<TamagotchiFactory> = None;

#[no_mangle]
extern fn init() {
    let tamagotchi_code_id: CodeId =
        msg::load().expect("Unable to decode CodeId of the Escrow program");
    let tamagotchi_factory = TamagotchiFactory {
        tamagotchi_code_id,
        ..Default::default()
    };
    unsafe { TAMAGOTCHI_FACTORY = Some(tamagotchi_factory) };
}

#[gstd::async_main]
async fn main() {
    let action: TamagotchiFactoryAction = msg::load().expect("Unable to decode `FactoryAction`");
    let factory = unsafe { TAMAGOTCHI_FACTORY.get_or_insert(Default::default()) };
    match action {
        TamagotchiFactoryAction::CreateTamagotchi { name } => {
            factory.create_tamagotchi(&exec::program_id(), name).await;
        }
        TamagotchiFactoryAction::TamagotchiName(tamagotchi_id) => {
            factory.get_tamagotchi_name(tamagotchi_id).await;
        }
        TamagotchiFactoryAction::TamagotchiAge(tamagotchi_id) => {
            factory.get_tamagotchi_age(tamagotchi_id).await;
        }
        TamagotchiFactoryAction::FeedTamagotchi(tamagotchi_id) => {
            factory.feed_tamagotchi(tamagotchi_id).await;
        }
        TamagotchiFactoryAction::PlayWithTamagotchi(tamagotchi_id) => {
            factory.play_with_tamagotchi(tamagotchi_id).await;
        }
        TamagotchiFactoryAction::SleepTamagotchi(tamagotchi_id) => {
            factory.sleep_tamagotchi(tamagotchi_id).await;
        }
        TamagotchiFactoryAction::TransferTamagotchi {
            tamagotchi_id,
            new_owner,
        } => {
            factory.transfer_tamagotchi(tamagotchi_id, new_owner).await;
        }
        TamagotchiFactoryAction::ApproveUser {
            tamagotchi_id,
            user,
        } => {
            factory.approve_user(tamagotchi_id, user).await;
        }
        TamagotchiFactoryAction::RemoveUserApproval(tamagotchi_id) => {
            factory.revoke_approval(tamagotchi_id).await;
        }
        TamagotchiFactoryAction::SetFTTokenContract {
            tamagotchi_id,
            ft_token_contract,
        } => {
            factory
                .set_ft_token_contract_to_tamagotchi(tamagotchi_id, ft_token_contract)
                .await;
        }
        TamagotchiFactoryAction::ApproveTokens {
            tamagotchi_id,
            user,
            amount,
        } => {
            factory
                .approve_tokens_from(tamagotchi_id, user, amount)
                .await;
        }
        TamagotchiFactoryAction::BuyAttributeToTamagotchi {
            tamagotchi_id,
            store_id,
            attribute_id,
        } => {
            factory
                .buy_attribute_to_tamagotchi(tamagotchi_id, store_id, attribute_id)
                .await
        }
        TamagotchiFactoryAction::CheckTamagotchiState(tamagotchi_id) => {
            factory.check_tamagotchi_state(tamagotchi_id).await;
        }
        TamagotchiFactoryAction::ReserveGasToTamagotchi {
            tamagotchi_id,
            reservation_amount,
            duration,
        } => {
            factory
                .reserve_gas_to_tamagotchi(tamagotchi_id, reservation_amount, duration)
                .await;
        }
    }
}

#[no_mangle]
extern fn state() {
    msg::reply(state_ref(), 0).expect("Failed to share state");
}

fn state_ref() -> &'static TamagotchiFactory {
    let state = unsafe { TAMAGOTCHI_FACTORY.as_ref() };
    debug_assert!(state.is_some(), "State is not initialized");
    unsafe { state.unwrap_unchecked() }
}
