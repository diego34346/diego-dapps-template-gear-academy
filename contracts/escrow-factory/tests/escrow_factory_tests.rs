#[allow(unused_imports)]
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
