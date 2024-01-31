use escrow_factory_io::*;
#[allow(unused_imports)]
use escrow_io::Escrow;
use gstd::ActorId;
#[allow(unused_imports)]
use gtest::{Log, Program, System, TestError};

pub const SELLER: u64 = 15;
pub const BUYER: u64 = 16;
pub const PRICE: u128 = 15_000_000_000_000;
pub const ONE_VARA: u128 = 1_000_000_000_000;
// address previously obtained from the state, to be
// able to perform the unit tests correctly
pub const ADDRESS_SCROW: ActorId = ActorId::new([
    240, 35, 217, 33, 79, 57, 144, 77, 203, 216, 17, 51, 38, 135, 252, 73, 206, 23, 79, 12, 248,
    73, 207, 171, 26, 91, 216, 6, 202, 243, 156, 250,
]);

#[test]
fn init_escrow_factory() {
    let system = System::new();
    let escrow_code_id =
        system.submit_code("../target/wasm32-unknown-unknown/release/escrow.opt.wasm");
    let escrow_factory = Program::current(&system);
    let res = escrow_factory.send(100, escrow_code_id);
    assert!(!res.main_failed());
}

#[test]
fn create_new_escrow() {
    let system = System::new();
    let escrow_code_id =
        system.submit_code("../target/wasm32-unknown-unknown/release/escrow.opt.wasm");
    let escrow_factory = Program::current(&system);
    let res = escrow_factory.send(100, escrow_code_id);
    assert!(!res.main_failed());

    // we create the payload to create a new scrow
    let payload = FactoryAction::CreateEscrow {
        seller: SELLER.into(),
        buyer: BUYER.into(),
        price: PRICE.into(),
    };

    let res = escrow_factory.send(BUYER, payload);

    let log = Log::builder()
        .dest(BUYER)
        .payload(FactoryEvent::EscrowCreated {
            escrow_id: 1u64.into(),
            escrow_address: ADDRESS_SCROW,
        });

    assert!(!res.main_failed());
    assert!(res.contains(&log));
}

#[test]
fn deposit_to_escrow() {
    let system = System::new();
    let escrow_code_id =
        system.submit_code("../target/wasm32-unknown-unknown/release/escrow.opt.wasm");
    let escrow_factory = Program::current(&system);
    let mut res = escrow_factory.send(100, escrow_code_id);
    assert!(!res.main_failed());

    let mut payload = FactoryAction::CreateEscrow {
        seller: SELLER.into(),
        buyer: BUYER.into(),
        price: PRICE.into(),
    };

    res = escrow_factory.send(BUYER, payload);

    let mut log = Log::builder()
        .dest(BUYER)
        .payload(FactoryEvent::EscrowCreated {
            escrow_id: 1u64.into(),
            escrow_address: ADDRESS_SCROW,
        });

    assert!(!res.main_failed());
    assert!(res.contains(&log));

    system.mint_to(BUYER, 2 * PRICE + ONE_VARA);

    payload = FactoryAction::Deposit(1u64.into());

    res = escrow_factory.send_with_value(BUYER, payload, PRICE);

    log = Log::builder()
        .dest(BUYER)
        .payload(FactoryEvent::Deposited(1u32.into()));

    assert!(!res.main_failed());
    assert!(res.contains(&log));
}

#[test]
fn deposit_to_escrow_fail() {
    let system = System::new();
    let escrow_code_id =
        system.submit_code("../target/wasm32-unknown-unknown/release/escrow.opt.wasm");
    let escrow_factory = Program::current(&system);
    let mut res = escrow_factory.send(100, escrow_code_id);
    assert!(!res.main_failed());

    let mut payload = FactoryAction::CreateEscrow {
        seller: SELLER.into(),
        buyer: BUYER.into(),
        price: PRICE.into(),
    };

    res = escrow_factory.send(BUYER, payload);

    let mut log = Log::builder()
        .dest(BUYER)
        .payload(FactoryEvent::EscrowCreated {
            escrow_id: 1u64.into(),
            escrow_address: ADDRESS_SCROW,
        });

    assert!(!res.main_failed());
    assert!(res.contains(&log));

    system.mint_to(BUYER, 3 * PRICE + ONE_VARA);

    payload = FactoryAction::Deposit(1u64.into());

    // must fail since the value is less than the price
    res = escrow_factory.send_with_value(BUYER, payload, PRICE - ONE_VARA);

    assert!(res.main_failed());

    payload = FactoryAction::Deposit(1u64.into());

    res = escrow_factory.send_with_value(BUYER, payload, PRICE);

    log = Log::builder()
        .dest(BUYER)
        .payload(FactoryEvent::Deposited(1u32.into()));

    assert!(!res.main_failed());
    assert!(res.contains(&log));
}

#[test]
fn confirm_delivery() {
    let system = System::new();
    let escrow_code_id =
        system.submit_code("../target/wasm32-unknown-unknown/release/escrow.opt.wasm");
    let escrow_factory = Program::current(&system);
    let mut res = escrow_factory.send(100, escrow_code_id);
    assert!(!res.main_failed());

    let mut payload = FactoryAction::CreateEscrow {
        seller: SELLER.into(),
        buyer: BUYER.into(),
        price: PRICE.into(),
    };

    res = escrow_factory.send(BUYER, payload);

    let mut log = Log::builder()
        .dest(BUYER)
        .payload(FactoryEvent::EscrowCreated {
            escrow_id: 1u64.into(),
            escrow_address: ADDRESS_SCROW,
        });

    assert!(!res.main_failed());
    assert!(res.contains(&log));

    system.mint_to(BUYER, 2 * PRICE + ONE_VARA);

    payload = FactoryAction::Deposit(1u64.into());

    res = escrow_factory.send_with_value(BUYER, payload, PRICE);

    log = Log::builder()
        .dest(BUYER)
        .payload(FactoryEvent::Deposited(1u64.into()));

    assert!(!res.main_failed());
    assert!(res.contains(&log));

    payload = FactoryAction::ConfirmDelivery(1u64.into());

    res = escrow_factory.send(BUYER, payload);

    log = Log::builder()
        .dest(BUYER)
        .payload(FactoryEvent::DeliveryConfirmed(1u32.into()));

    assert!(!res.main_failed());
    assert!(res.contains(&log));
}

#[test]
fn confirm_delivery_fail() {
    let system = System::new();
    let escrow_code_id =
        system.submit_code("../target/wasm32-unknown-unknown/release/escrow.opt.wasm");
    let escrow_factory = Program::current(&system);
    let mut res = escrow_factory.send(100, escrow_code_id);
    assert!(!res.main_failed());

    let mut payload = FactoryAction::CreateEscrow {
        seller: SELLER.into(),
        buyer: BUYER.into(),
        price: PRICE.into(),
    };

    res = escrow_factory.send(BUYER, payload);

    let mut log = Log::builder()
        .dest(BUYER)
        .payload(FactoryEvent::EscrowCreated {
            escrow_id: 1u64.into(),
            escrow_address: ADDRESS_SCROW,
        });

    assert!(!res.main_failed());
    assert!(res.contains(&log));

    system.mint_to(BUYER, 2 * PRICE + ONE_VARA);

    payload = FactoryAction::Deposit(1u64.into());

    res = escrow_factory.send_with_value(BUYER, payload, PRICE);

    log = Log::builder()
        .dest(BUYER)
        .payload(FactoryEvent::Deposited(1u64.into()));

    assert!(!res.main_failed());
    assert!(res.contains(&log));

    payload = FactoryAction::ConfirmDelivery(1u64.into());

    // must fail since is not the buyer
    res = escrow_factory.send(SELLER, payload);

    assert!(res.main_failed());

    payload = FactoryAction::ConfirmDelivery(1u64.into());

    res = escrow_factory.send(BUYER, payload);

    log = Log::builder()
        .dest(BUYER)
        .payload(FactoryEvent::DeliveryConfirmed(1u32.into()));

    assert!(!res.main_failed());
    assert!(res.contains(&log));
}

#[test]
fn confirm_delivery_wrong_buyer() {
    let system = System::new();
    let escrow_code_id =
        system.submit_code("../target/wasm32-unknown-unknown/release/escrow.opt.wasm");
    let escrow_factory = Program::current(&system);
    let mut res = escrow_factory.send(100, escrow_code_id);
    assert!(!res.main_failed());

    let mut payload = FactoryAction::CreateEscrow {
        seller: SELLER.into(),
        buyer: BUYER.into(),
        price: PRICE,
    };

    res = escrow_factory.send(BUYER, payload);

    let mut log = Log::builder()
        .dest(BUYER)
        .payload(FactoryEvent::EscrowCreated {
            escrow_id: 1u64,
            escrow_address: ADDRESS_SCROW,
        });

    assert!(!res.main_failed());
    assert!(res.contains(&log));

    system.mint_to(BUYER, 2 * PRICE + ONE_VARA);

    payload = FactoryAction::Deposit(1u64);

    res = escrow_factory.send_with_value(BUYER, payload, PRICE);

    log = Log::builder()
        .dest(BUYER)
        .payload(FactoryEvent::Deposited(1u64));

    assert!(!res.main_failed());
    assert!(res.contains(&log));

    payload = FactoryAction::ConfirmDelivery(1u64);

    // Intentamos confirmar la entrega siendo un comprador diferente
    let wrong_buyer = BUYER + 1;
    res = escrow_factory.send(wrong_buyer, payload);

    // Debería fallar ya que el remitente no es el comprador correcto
    assert!(res.main_failed());
}
