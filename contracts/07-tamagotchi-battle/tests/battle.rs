use gstd::collections::BTreeSet;
use gstd::msg;
use tamagotchi_battle_io::*;

#[gstd::test]
fn test_initiate_registration() {
    let mut battle = Battle::default();
    let tmg_id = ActorId::new([1; 32]);
    let attributes = AttributesPerRound {
        round_1: Some(1),
        round_2: Some(2),
        round_3: Some(3),
    };

    // Mock the source actor ID for the test
    msg::set_source(ActorId::new([2; 32]));

    // Perform the registration
    gstd::runtime::execute(|| {
        battle.initiate_registration(&tmg_id, attributes).await;
    });

    // Verify the expected outcome
    assert_eq!(battle.players.len(), 1);
    assert_eq!(battle.state, BattleState::Registration);
    assert_eq!(battle.current_turn, 0);
    // Add more assertions based on your specific logic
}

#[gstd::test]
fn test_execute_move() {
    let mut battle = Battle::default();
    let player1 = Player {
        owner: ActorId::new([1; 32]),
        tmg_id: ActorId::new([2; 32]),
        energy: 100,
        power: 50,
        attributes: AttributesPerRound::default(),
        actual_attribute: 1,
        actual_side: DirectionOfMovement::Right,
    };
    let player2 = Player {
        owner: ActorId::new([3; 32]),
        tmg_id: ActorId::new([4; 32]),
        energy: 80,
        power: 60,
        attributes: AttributesPerRound::default(),
        actual_attribute: 1,
        actual_side: DirectionOfMovement::Left,
    };

    battle.players.push(player1.clone());
    battle.players.push(player2.clone());
    battle.state = BattleState::Moves;
    battle.current_turn = 0;

    // Mock the source actor ID for the test
    msg::set_source(player1.owner);

    // Perform the move
    gstd::runtime::execute(|| {
        battle.execute_move(DirectionOfMovement::Right);
    });

    // Verify the expected outcome
    assert_eq!(battle.state, BattleState::Waiting);
    assert_eq!(battle.steps, 1);
    // Add more assertions based on your specific logic
}