#![no_std]
use escrow_factory_io::*;
#[allow(unused_imports)]
use gstd::{msg, prelude::*, ActorId, CodeId};

static mut ESCROW_FACTORY: Option<EscrowFactory> = None;

#[no_mangle]
extern fn init() {
    let escrow_code_id: CodeId =
        msg::load().expect("Unable to decode CodeId of the Escrow program");
    let escrow_factory = EscrowFactory {
        escrow_code_id,
        ..Default::default()
    };
    unsafe { ESCROW_FACTORY = Some(escrow_factory) };
}

#[gstd::async_main]
async fn main() {
    let action: FactoryAction = msg::load().expect("Unable to decode `FactoryAction`");
    let factory = unsafe { ESCROW_FACTORY.get_or_insert(Default::default()) };
    match action {
        FactoryAction::CreateEscrow {
            seller,
            buyer,
            price,
        } => factory.create_escrow(&seller, &buyer, price).await,
        FactoryAction::Deposit(escrow_id) => factory.deposit(escrow_id).await,
        FactoryAction::ConfirmDelivery(escrow_id) => factory.confirm_delivery(escrow_id).await,
    }
}

#[no_mangle]
extern fn state() {
    msg::reply(state_ref(), 0).expect("Failed to share state");
}

fn state_ref() -> &'static EscrowFactory {
    let state = unsafe { ESCROW_FACTORY.as_ref() };
    debug_assert!(state.is_some(), "state is not initialized");
    unsafe { state.unwrap_unchecked() }
}
