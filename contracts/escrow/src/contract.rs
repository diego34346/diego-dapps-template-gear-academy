use gstd::{msg, prelude::*};

use escrow_io::{Escrow, EscrowAction, EscrowState, InitEscrow};

static mut ESCROW: Option<Escrow> = None;

fn state_mut() -> &'static mut Escrow {
    let state = unsafe { ESCROW.as_mut() };

    debug_assert!(state.is_some(), "state isn't initialized");

    unsafe { state.unwrap_unchecked() }
}

#[no_mangle]
unsafe extern fn init() {
    let init_config: InitEscrow = msg::load().expect("Error in decoding `InitEscrow");
    let escrow = Escrow {
        seller: init_config.seller,
        buyer: init_config.buyer,
        price: init_config.price,
        state: EscrowState::AwaitingPayment,
    };
    ESCROW = Some(escrow);

    msg::reply(String::from("Escrow created"), 0).expect("");
}

#[no_mangle]
unsafe extern fn handle() {
    let action: EscrowAction = msg::load().expect("Unable to decode `EscrowAction");
    let escrow = state_mut();
    match action {
        EscrowAction::Deposit(address) => escrow.deposit(address),
        EscrowAction::ConfirmDelivery(address) => escrow.confirm_delivery(address),
    }
}

#[no_mangle]
extern fn state() {
    let escrow = state_mut();
    msg::reply(escrow, 0).expect("Failed to share state");
}
