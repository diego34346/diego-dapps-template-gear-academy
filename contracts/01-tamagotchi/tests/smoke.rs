use gtest::{Program, System};

#[test]
fn smoke_test() {
    let sys = System::new();
    sys.init_logger();
    let program = Program::current(&sys);

    let init_name = "InitialTamagotchiName".to_string();
    let res_init = program.send(0, init_name.clone());
    assert!(!res_init.main_failed());
}
