use gtest::{Log, Program, System};
use tamagotchi_interaction_io::{TmgAction, TmgEvent};

#[test]
fn interaction_test() {
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

    // Action Feed

    let res_feed = program.send(2, TmgAction::Feed);
    assert!(!res_feed.main_failed());

    let res_state = program.send(2, TmgAction::Feed);
    let expected_log = Log::builder().dest(2).payload(TmgEvent::Fed);
    assert!(res_state.contains(&expected_log));

    // Action Entertain

    let res_entertain = program.send(2, TmgAction::Entertain);
    assert!(!res_entertain.main_failed());

    let res_state = program.send(2, TmgAction::Entertain);
    let expected_log = Log::builder().dest(2).payload(TmgEvent::Entertained);
    assert!(res_state.contains(&expected_log));

    // Action Sleep

    let res_sleep = program.send(2, TmgAction::Sleep);
    assert!(!res_sleep.main_failed());

    let res_state = program.send(2, TmgAction::Sleep);
    let expected_log = Log::builder().dest(2).payload(TmgEvent::Slept);
    assert!(res_state.contains(&expected_log));
}
