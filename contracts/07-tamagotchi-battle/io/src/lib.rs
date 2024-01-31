#![no_std]

use gmeta::{InOut, Metadata, Out};
use gstd::{
    collections::{BTreeMap, BTreeSet},
    exec, msg,
    prelude::*,
    ActorId, ReservationId,
};
use tamagotchi_auto_io::{TmgAction, TmgEvent};
use tamagotchi_store_io::TamagotchiId;
use tamagotchi_store_io::{AttributeId, StoreAction, StoreEvent};

#[derive(PartialEq, Debug, Default, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum BattleState {
    #[default]
    Registration,
    Moves,
    Waiting,
    GameIsOver,
}
#[derive(Encode, Decode, TypeInfo, Default, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct Player {
    pub owner: ActorId,
    pub tmg_id: TamagotchiId,
    pub energy: u16,
    pub power: u16,
    pub attributes: AttributesPerRound,
    pub actual_attribute: u8,
    pub actual_side: DirectionOfMovement,
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum BattleAction {
    Register {
        tamagotchi_id: TamagotchiId,
        attributes: AttributesPerRound,
    },
    Move(DirectionOfMovement),
    UpdateInfo,
    StartNewGame,
    SendNewAttributesToNextRound {
        new_attributes: AttributesPerRound,
    },
    ReserveGas {
        reservation_amount: u64,
        duration: u32,
    },
}

#[derive(Encode, Decode, TypeInfo, Default)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum BattleEvent {
    #[default]
    InfoUpdated,
    MoveMade,
    GoToWaitingState,
    GameIsOver,
    Registered {
        tmg_id: TamagotchiId,
    },
    AttributesUpdated,
    ContractReinstated,
    GasReserved,
    OpponentDodgedTheAttack,
}

#[derive(Encode, Decode, TypeInfo, Default, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct AttributesPerRound {
    round_1: Option<AttributeId>,
    round_2: Option<AttributeId>,
    round_3: Option<AttributeId>,
}

#[derive(Encode, Decode, TypeInfo, Default, Clone, Eq, PartialEq)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum DirectionOfMovement {
    #[default]
    Right,
    Left,
}

#[derive(Encode, Decode, TypeInfo, Default)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct BattleInit {
    pub tmg_store_id: ActorId,
}

#[derive(Encode, Decode, TypeInfo, Default)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct Battle {
    pub players: Vec<Player>,
    pub state: BattleState,
    pub current_turn: u8,
    pub tmg_store_id: ActorId,
    pub winner: ActorId,
    pub steps: u8,
    pub weapons_data: BTreeMap<AttributeId, PowerTmg>,
    pub shields_data: BTreeMap<AttributeId, ProtectionTmg>,
    pub reservations: Vec<ReservationId>,
}

pub type ProtectionTmg = u16;
pub type TmgId = ActorId;
pub type PowerTmg = u16;

const MAX_POWER: u16 = 5_000;
const MIN_POWER: u16 = 3_000;
const MAX_ENERGY: u16 = 10_000;
const MIN_ENERGY: u16 = 20_000;
const GAS_AMOUNT: u64 = 10_000_000_000;
const TIME_FOR_UPDATE: u32 = 10;
const MAX_STEPS_FOR_ROUND: u8 = 4;

//Mis armas

// SWORDS
pub const SWORD_TMG: PowerTmg = 2;
pub const SWORD_ID: AttributeId = 1;
pub const SWORD_TMG_WOOD: PowerTmg = 1;
pub const SWORD_WOOD_ID: AttributeId = 2;
// Guns
pub const SHOTGUN_POWER: PowerTmg = 6;
pub const SHOTGUN_ID: AttributeId = 3;
pub const RPG_POWER: PowerTmg = 10;
pub const RPG_ID: AttributeId = 4;
// ESCUDITO
pub const SHIELD: ProtectionTmg = 3_500;
pub const SHIELD_ID: AttributeId = 5;

impl Battle {
    pub async fn initiate_registration(
        &mut self,
        tmg_id: &TamagotchiId,
        attributes: AttributesPerRound,
    ) {
        assert_eq!(
            self.state,
            BattleState::Registration,
            "The game has already started"
        );

        let owner = Self::fetch_owner(tmg_id).await;
        let power = Self::generate_random_number_field(MIN_POWER, MAX_POWER);
        let power = MAX_POWER - power;
        let energy = Self::generate_random_number_field(MIN_ENERGY, MAX_ENERGY);
        let energy = MAX_ENERGY - energy;
        let random_position = Self::determine_turn();
        let actual_side = if random_position == 0 {
            DirectionOfMovement::Left
        } else {
            DirectionOfMovement::Right
        };

        let player = Player {
            owner,
            tmg_id: *tmg_id,
            energy,
            power,
            attributes,
            actual_attribute: 1,
            actual_side,
        };

        self.players.push(player);

        if self.players.len() == 2 {
            self.current_turn = Self::determine_turn();
            self.state = BattleState::Moves;
        }

        msg::reply(BattleEvent::Registered { tmg_id: *tmg_id }, 0)
            .expect("Error during a reply `BattleEvent::Registered");
    }

    pub async fn fetch_owner(tmg_id: &ActorId) -> ActorId {
        let reply: TmgEvent = msg::send_for_reply_as(*tmg_id, TmgAction::TmgInfo, 0, 0)
            .expect("Error in sending a message `TmgAction::TmgInfo")
            .await
            .expect("Unable to decode TmgEvent");
        if let TmgEvent::Owner(owner) = reply {
            owner
        } else {
            panic!("Wrong received message");
        }
    }

    async fn fetch_attributes(
        tmg_store_id: &ActorId,
        tmg_id: &TamagotchiId,
    ) -> BTreeSet<AttributeId> {
        let reply: StoreEvent = msg::send_for_reply_as(
            *tmg_store_id,
            StoreAction::GetAttributes {
                tamagotchi_id: *tmg_id,
            },
            0,
            0,
        )
        .expect("Error in sending a message `StoreAction::GetAttributes")
        .await
        .expect("Unable to decode `StoreEvent`");
        if let StoreEvent::Attributes { attributes } = reply {
            attributes
        } else {
            panic!("Wrong received message");
        }
    }

    pub fn determine_turn() -> u8 {
        let random_input: [u8; 32] = array::from_fn(|i| i as u8 + 1);
        let (random, _) = exec::random(random_input).expect("Error in getting random number");
        random[0] % 2
    }

    pub fn generate_random_number_field(min_field: u16, max_field: u16) -> u16 {
        let random_input: [u8; 32] = array::from_fn(|i| i as u8 + 1);
        let (random, _) = exec::random(random_input).expect("Error in getting random number");
        let bytes: [u8; 2] = [random[0], random[1]];
        let random_number: u16 = u16::from_be_bytes(bytes) % max_field;
        if random_number < min_field {
            return max_field / 2;
        }
        random_number
    }

    pub fn execute_move(&mut self, direction: DirectionOfMovement) {
        assert_eq!(
            self.state,
            BattleState::Moves,
            "The game is not in `Moves` state"
        );
        let turn = self.current_turn as usize;

        let next_turn = ((turn + 1) % 2) as usize;

        let mut player = self.players[turn].clone();

        assert_eq!(
            player.owner,
            msg::source(),
            "You are not in the game or it is not your turn"
        );

        let mut opponent = self.players[next_turn].clone();

        let player_attribute_id = Self::get_player_actual_attribute_id(&player);

        player.actual_attribute += 1;
        player.actual_side = direction;
        if !self.shields_data.contains_key(&player_attribute_id) {
            let total_atack = self.calculate_tamagotchi_total_attack(&player, &opponent);
            if player.actual_side == opponent.actual_side {
                opponent.energy = opponent.energy.saturating_sub(total_atack);
            } else {
                msg::send(player.owner, BattleEvent::OpponentDodgedTheAttack, 0)
                    .expect("error sending message to player");
            }
        }

        self.players[next_turn] = opponent.clone();
        self.players[turn] = player.clone();
        if opponent.energy == 0 {
            self.players = Vec::new();
            self.state = BattleState::GameIsOver;
            self.winner = player.tmg_id;
            self.steps = 0;

            msg::send(opponent.owner, BattleEvent::GameIsOver, 0)
                .expect("Error sending message to opponent");

            msg::reply(BattleEvent::GameIsOver, 0)
                .expect("Error in sending a reply `BattleEvent::GameIsOver`");
            return;
        }

        if self.steps <= MAX_STEPS_FOR_ROUND {
            self.steps += 1;
            self.current_turn = next_turn as u8;

            msg::reply(BattleEvent::MoveMade, 0)
                .expect("Error in sending a reply `BattleEvent::MoveMade`");
        } else {
            self.state = BattleState::Waiting;
            self.steps = 0;
            self.players[turn].actual_attribute = 1;
            self.players[next_turn].actual_attribute = 1;

            if !self.reservations.is_empty() {
                let Some(reservation_id) = self.reservations.pop() else {
                    panic!("Error getting transaction id, reservations is empty");
                };

                msg::send_delayed_from_reservation(
                    reservation_id,
                    exec::program_id(),
                    BattleAction::UpdateInfo,
                    0,
                    TIME_FOR_UPDATE,
                )
                .expect("Error in sending a delayed message `BattleAction::UpdateInfo`");
            } else {
                msg::send_with_gas_delayed(
                    exec::program_id(),
                    BattleAction::UpdateInfo,
                    GAS_AMOUNT,
                    0,
                    TIME_FOR_UPDATE,
                )
                .expect("Error in sending a delayed message `BattleAction::UpdateInfo`");
            }

            msg::send(opponent.owner, BattleEvent::GoToWaitingState, 0)
                .expect("Error sending message to opponent");

            msg::reply(BattleEvent::GoToWaitingState, 0)
                .expect("Error in sending a reply `BattleEvent::MoveMade`");
        }
    }

    pub fn calculate_tamagotchi_total_attack(&self, player: &Player, opponent: &Player) -> u16 {
        let mut total_attack = player.power;
        let player_weapon_attribute = Self::get_player_actual_attribute_id(player);
        let opponent_attribute = Self::get_player_actual_attribute_id(opponent);
        if self.weapons_data.contains_key(&player_weapon_attribute) {
            total_attack *= *self.weapons_data.get(&player_weapon_attribute).unwrap();
        }
        if self.shields_data.contains_key(&opponent_attribute) {
            let protection = *self.shields_data.get(&opponent_attribute).unwrap();
            total_attack = total_attack.saturating_sub(protection);
        }

        total_attack
    }

    pub fn get_player_actual_attribute_id(player: &Player) -> u32 {
        match player.actual_attribute {
            1 => {
                if let Some(attribute_id) = player.attributes.round_1 {
                    attribute_id
                } else {
                    0
                }
            }
            2 => {
                if let Some(attribute_id) = player.attributes.round_2 {
                    attribute_id
                } else {
                    0
                }
            }
            3 => {
                if let Some(attribute_id) = player.attributes.round_3 {
                    attribute_id
                } else {
                    0
                }
            }
            _ => 0,
        }
    }

    pub fn update_tamagotchi_attributes(&mut self, new_attributes: AttributesPerRound) {
        assert_eq!(self.state, BattleState::Waiting, "`Waiting`");

        let caller = msg::source();

        if self.players[0].owner == caller {
            self.players[0].attributes = new_attributes;
        } else if self.players[1].owner == caller {
            self.players[1].attributes = new_attributes;
        } else {
            panic!("You are not in the game or it is not your turn");
        }

        msg::reply(BattleEvent::AttributesUpdated, 0).expect("Error sending reply");
    }

    pub async fn update_game_info(&mut self) {
        assert_eq!(
            msg::source(),
            exec::program_id(),
            "Only the contract itself can call that action"
        );
        assert_eq!(self.state, BattleState::Waiting, "`Waiting`");

        self.state = BattleState::Moves;
        self.current_turn = Self::determine_turn();

        msg::send(self.players[0].owner, BattleEvent::InfoUpdated, 0)
            .expect("Error during a reply BattleEvent::InfoUpdated");

        msg::send(self.players[1].owner, BattleEvent::InfoUpdated, 0)
            .expect("Error during a reply BattleEvent::InfoUpdated");
    }

    pub fn initiate_reservation(&mut self, reservation_amount: u64, reservation_duration: u32) {
        let reservation_id = ReservationId::reserve(reservation_amount, reservation_duration)
            .expect("reservation across executions");

        self.reservations.push(reservation_id);

        msg::reply(BattleEvent::GasReserved, 0).expect("Error in reply");
    }
    pub fn reset_game(&mut self) {
        assert_eq!(self.state, BattleState::GameIsOver, "`GameIsOver`");

        self.state = BattleState::Registration;

        msg::reply(BattleEvent::ContractReinstated, 0).expect("Error sending reply");
    }
}

pub struct ProgramMetadata;

impl Metadata for ProgramMetadata {
    type Init = InOut<BattleInit, String>;
    type Handle = InOut<BattleAction, BattleEvent>;
    type State = Out<Battle>;
    type Reply = ();
    type Others = InOut<BattleAction, BattleEvent>;
    type Signal = ();
}
