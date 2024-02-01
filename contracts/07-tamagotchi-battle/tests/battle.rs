use gstd::ActorId;
use tamagotchi_battle_io::*;
use tokio::time::Duration;

#[tokio::main]
async fn main() {
    test_initiate_registration().await;
    test_execute_move().await;
}

async fn test_initiate_registration() {
    let mut battle = Battle::default();
    let tmg_id = ActorId::new([1u8; 32]);

    battle
        .initiate_registration(&tmg_id, AttributesPerRound::default())
        .await;

    assert_eq!(battle.state, BattleState::Moves);
    assert_eq!(battle.players.len(), 1);
    assert_eq!(battle.players[0].tmg_id, tmg_id);
}

async fn test_execute_move() {
    let mut battle = Battle::default();
    let tmg_id_1 = ActorId::new([1u8; 32]);
    let tmg_id_2 = ActorId::new([2u8; 32]);

    battle
        .initiate_registration(&tmg_id_1, AttributesPerRound::default())
        .await;
    battle
        .initiate_registration(&tmg_id_2, AttributesPerRound::default())
        .await;

    battle.execute_move(DirectionOfMovement::Right);

    assert_eq!(battle.state, BattleState::Waiting);
    assert_eq!(battle.steps, 1);
    assert_eq!(battle.current_turn, 1);
}
