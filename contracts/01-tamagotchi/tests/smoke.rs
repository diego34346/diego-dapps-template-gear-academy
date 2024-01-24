use gtest::{Log, Program, System};
use tamagotchi_io::{TmgAction, TmgEvent};

#[test]
fn smoke_test() {
    let sys = System::new();
    sys.init_logger();
    let program = Program::current(&sys);

    let init_name = "Diego".to_string();
    let res_init = program.send(0, init_name.clone());
    assert!(!res_init.main_failed());

    program.send(2, String::from("Diego"));
    let res = program.send(2, TmgAction::Name);
    let expected_log = Log::builder()
        .dest(2)
        .payload(TmgEvent::Name("Diego".to_string()));
    assert!(res.contains(&expected_log));
}
