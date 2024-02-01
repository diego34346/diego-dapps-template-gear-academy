use gstd::ActorId;
use gtest::{Log, Program, System};
use tamagotchi_store_io::StoreInit;
use tamagotchi_store_io::{AttrMetadata, AttributeStore, StoreAction, StoreEvent};

#[test]
fn create_attribute_test() {
    let sys = System::new();
    sys.init_logger();
    let program = Program::current(&sys);

    let init_data = StoreInit {
        ft_contract_id: ActorId::new([1; 32]),
        cost_to_upgrade_weapons: 100,
    };
    program.send_init(0, init_data);
    assert!(!program.res().main_failed());

    let metadata = AttrMetadata {
        title: "Strength".to_string(),
        description: "Increase strength attribute".to_string(),
        media: "image.png".to_string(),
    };
    program.send(
        2,
        StoreAction::CreateAttribute {
            attribute_id: 1,
            attribute_upgrade_id: 2,
            attr_metadata: metadata.clone(),
            can_upgrade: true,
            price: 50,
        },
    );
    let expected_log = Log::builder()
        .dest(2)
        .payload(StoreEvent::AttributeCreated { attribute_id: 1 });
    assert!(!program.res().contains(&expected_log));

    let res_state = program.send(
        2,
        StoreAction::GetAttributes {
            tamagotchi_id: ActorId::new([2; 32]),
        },
    );
    let expected_log = Log::builder().dest(2).payload(StoreEvent::Attributes {
        attributes: [1].iter().cloned().collect(),
    });
    assert!(!res_state.contains(&expected_log));
}

#[test]
fn purchase_attribute_test() {
    let sys = System::new();
    sys.init_logger();
    let program = Program::current(&sys);

    let init_data = StoreInit {
        ft_contract_id: ActorId::new([1; 32]),
        cost_to_upgrade_weapons: 100,
    };
    program.send_init(0, init_data);
    assert!(!program.res().main_failed());

    let metadata = AttrMetadata {
        title: "Strength".to_string(),
        description: "Increase strength attribute".to_string(),
        media: "image.png".to_string(),
    };
    program.send(
        2,
        StoreAction::CreateAttribute {
            attribute_id: 1,
            attribute_upgrade_id: 2,
            attr_metadata: metadata.clone(),
            can_upgrade: true,
            price: 50,
        },
    );
    assert!(!program.res().main_failed());

    program.send(3, StoreAction::BuyAttribute { attribute_id: 1 });
    let expected_log = Log::builder()
        .dest(3)
        .payload(StoreEvent::AttributeSold { success: true });
    assert!(!program.res().contains(&expected_log));

    let res_state = program.send(
        3,
        StoreAction::GetAttributes {
            tamagotchi_id: ActorId::new([3; 32]),
        },
    );
    let expected_log = Log::builder().dest(3).payload(StoreEvent::Attributes {
        attributes: [1].iter().cloned().collect(),
    });
    assert!(!res_state.contains(&expected_log));
}

#[test]
fn upgrade_attribute_test() {
    let sys = System::new();
    sys.init_logger();
    let program = Program::current(&sys);

    let init_data = StoreInit {
        ft_contract_id: ActorId::new([1; 32]),
        cost_to_upgrade_weapons: 100,
    };
    program.send_init(0, init_data);
    assert!(!program.res().main_failed());

    let metadata = AttrMetadata {
        title: "Strength".to_string(),
        description: "Increase strength attribute".to_string(),
        media: "image.png".to_string(),
    };
    program.send(
        2,
        StoreAction::CreateAttribute {
            attribute_id: 1,
            attribute_upgrade_id: 2,
            attr_metadata: metadata.clone(),
            can_upgrade: true,
            price: 50,
        },
    );
    assert!(!program.res().main_failed());

    program.send(3, StoreAction::BuyAttribute { attribute_id: 1 });
    assert!(!program.res().main_failed());

    program.send(3, StoreAction::UpgradeAttribute { attribute_id: 1 });
    let expected_log = Log::builder()
        .dest(3)
        .payload(StoreEvent::AttributeUpgrade { success: true });
    assert!(!program.res().contains(&expected_log));

    let res_state = program.send(
        3,
        StoreAction::GetAttributes {
            tamagotchi_id: ActorId::new([3; 32]),
        },
    );
    let expected_log = Log::builder().dest(3).payload(StoreEvent::Attributes {
        attributes: [2].iter().cloned().collect(),
    });
    assert!(!res_state.contains(&expected_log));
}
