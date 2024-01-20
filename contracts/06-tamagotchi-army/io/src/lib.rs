#![no_std]

use gmeta::{In, InOut, Metadata, Out};
use gstd::{collections::BTreeMap, msg, prelude::*, prog::ProgramGenerator, ActorId, CodeId};
use tamagotchi_auto_io::*;

pub type TamagotchiId = u64;
pub type AttributeId = u32;

const GAS_FOR_CREATION: u64 = 5_000_000_000; // 1_000_000_000;

pub struct ProgramMetadata;

impl Metadata for ProgramMetadata {
    type Init = In<CodeId>;
    type Handle = InOut<TamagotchiFactoryAction, TamagotchiFactoryEvent>;
    type State = Out<TamagotchiFactory>;
    type Reply = ();
    type Others = InOut<TamagotchiFactoryAction, TamagotchiFactoryEvent>;
    type Signal = ();
}

#[derive(Encode, Decode, TypeInfo, Default)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct TamagotchiFactory {
    pub tamagotchi_number: TamagotchiId,
    pub id_to_address: BTreeMap<TamagotchiId, ActorId>,
    pub tamagotchi_code_id: CodeId,
}

impl TamagotchiFactory {
    pub async fn create_tamagotchi(&mut self, tamagotchi_owner: &ActorId, tamagotchi_name: String) {
        let (address, _) = ProgramGenerator::create_program_with_gas_for_reply(
            self.tamagotchi_code_id,
            TmgInit {
                owner: *tamagotchi_owner,
                name: tamagotchi_name,
            },
            GAS_FOR_CREATION,
            0,
            5_000_000_000,
        )
        .expect("Error during Tamagotchi program initialization")
        .await
        .expect("Program was not initialized");
        self.tamagotchi_number = self.tamagotchi_number.saturating_add(1);
        self.id_to_address.insert(self.tamagotchi_number, address);
        msg::reply(
            TamagotchiFactoryEvent::TamagotchiCreated {
                tamagotchi_id: self.tamagotchi_number,
                tamagotchi_address: address,
            },
            0,
        )
        .expect(" Error during a reply `FactoryEvent::ProgramCreated`");
    }

    pub async fn get_tamagotchi_name(&self, tamagotchi_id: TamagotchiId) {
        let tamagotchi_address = self.get_tamagotchi_address(tamagotchi_id);
        let tamagotchi_name_ans = Self::send_message(&tamagotchi_address, TmgAction::Name).await;

        let TmgEvent::Name(tamagotchi_name) = tamagotchi_name_ans else {
            panic!("Incorrect answer from tamagotchi contract");
        };

        msg::reply(TamagotchiFactoryEvent::Name(tamagotchi_name), 0).expect("Error sending reply");
    }

    pub async fn get_tamagotchi_age(&self, tamagotchi_id: TamagotchiId) {
        let tamagotchi_address = self.get_tamagotchi_address(tamagotchi_id);
        let tamagotchi_age_ans = Self::send_message(&tamagotchi_address, TmgAction::Age).await;

        let TmgEvent::Age(tamagotchi_age) = tamagotchi_age_ans else {
            panic!("Incorrect answer from tamagotchi contract");
        };

        msg::reply(TamagotchiFactoryEvent::Age(tamagotchi_age), 0).expect("Error sending reply");
    }

    pub async fn feed_tamagotchi(&self, tamagotchi_id: TamagotchiId) {
        let tamagotchi_address = self.get_tamagotchi_address(tamagotchi_id);
        let tamagotchi_ans = Self::send_message(&tamagotchi_address, TmgAction::Feed).await;

        if tamagotchi_ans != TmgEvent::Fed {
            panic!("Incorrect answer from tamagotchi contract");
        }

        msg::reply(TamagotchiFactoryEvent::Fed, 0).expect("Error sending reply");
    }

    pub async fn play_with_tamagotchi(&self, tamagotchi_id: TamagotchiId) {
        let tamagotchi_address = self.get_tamagotchi_address(tamagotchi_id);
        let tamagotchi_ans = Self::send_message(&tamagotchi_address, TmgAction::Play).await;

        if tamagotchi_ans != TmgEvent::Entertained {
            panic!("Incorrect answer from tamagotchi contract");
        }

        msg::reply(TamagotchiFactoryEvent::Entertained, 0).expect("Error sending reply");
    }

    pub async fn sleep_tamagotchi(&self, tamagotchi_id: TamagotchiId) {
        let tamagotchi_address = self.get_tamagotchi_address(tamagotchi_id);
        let tamagotchi_ans = Self::send_message(&tamagotchi_address, TmgAction::Sleep).await;

        if tamagotchi_ans != TmgEvent::Slept {
            panic!("Incorrect answer from tamagotchi contract");
        }

        msg::reply(TamagotchiFactoryEvent::Slept, 0).expect("Error sending reply");
    }

    pub async fn transfer_tamagotchi(&self, tamagotchi_id: TamagotchiId, new_owner: ActorId) {
        let tamagotchi_address = self.get_tamagotchi_address(tamagotchi_id);
        let tamagotchi_ans =
            Self::send_message(&tamagotchi_address, TmgAction::Transfer(new_owner)).await;

        if let TmgEvent::Transferred(_new_owner) = tamagotchi_ans {
            panic!("Incorrect answer from tamagotchi contract");
        }

        msg::reply(TamagotchiFactoryEvent::Transferred(new_owner), 0).expect("Error sending reply");
    }

    pub async fn approve_user(&self, tamagotchi_id: TamagotchiId, user: ActorId) {
        let tamagotchi_address = self.get_tamagotchi_address(tamagotchi_id);
        let tamagotchi_ans =
            Self::send_message(&tamagotchi_address, TmgAction::Approve(user)).await;

        if let TmgEvent::Approved(_user) = tamagotchi_ans {
            panic!("Incorrect answer from tamagotchi contract");
        }

        msg::reply(TamagotchiFactoryEvent::Approved(user), 0).expect("Error sending reply");
    }

    pub async fn revoke_approval(&self, tamagotchi_id: TamagotchiId) {
        let tamagotchi_address = self.get_tamagotchi_address(tamagotchi_id);
        let tamagotchi_ans =
            Self::send_message(&tamagotchi_address, TmgAction::RevokeApproval).await;

        if tamagotchi_ans != TmgEvent::ApprovalRevoked {
            panic!("Incorrect answer from tamagotchi contract");
        }

        msg::reply(TamagotchiFactoryEvent::ApprovalRevoked, 0).expect("Error sending reply");
    }

    pub async fn set_ft_token_contract_to_tamagotchi(
        &self,
        tamagotchi_id: TamagotchiId,
        ft_token_contract: ActorId,
    ) {
        let tamagotchi_address = self.get_tamagotchi_address(tamagotchi_id);
        let tamagotchi_ans = Self::send_message(
            &tamagotchi_address,
            TmgAction::SetFTokenContract(ft_token_contract),
        )
        .await;

        if tamagotchi_ans != TmgEvent::FTokenContractSet {
            panic!("Incorrect answer from tamagotchi contract");
        }

        msg::reply(TamagotchiFactoryEvent::FTTokenContractSet, 0).expect("Error sending reply");
    }

    pub async fn approve_tokens_from(
        &self,
        tamagotchi_id: TamagotchiId,
        account: ActorId,
        amount: u128,
    ) {
        let tamagotchi_address = self.get_tamagotchi_address(tamagotchi_id);
        let tamagotchi_ans = Self::send_message(
            &tamagotchi_address,
            TmgAction::ApproveTokens { account, amount },
        )
        .await;

        if tamagotchi_ans == TmgEvent::ApprovalError {
            msg::reply(TamagotchiFactoryEvent::ApprovalError, 0).expect("Error sending reply");
            return;
        }

        let correct_ans = TmgEvent::TokensApproved { account, amount };

        if tamagotchi_ans != correct_ans {
            panic!("Incorrect answer from tamagotchi contract");
        }

        let response = TamagotchiFactoryEvent::TokensApproved { account, amount };

        msg::reply(response, 0).expect("Error sending reply");
    }

    pub async fn buy_attribute_to_tamagotchi(
        &self,
        tamagotchi_id: TamagotchiId,
        store_id: ActorId,
        attribute_id: AttributeId,
    ) {
        let tamagotchi_address = self.get_tamagotchi_address(tamagotchi_id);
        let tamagotchi_ans = Self::send_message(
            &tamagotchi_address,
            TmgAction::BuyAttribute {
                store_id,
                attribute_id,
            },
        )
        .await;

        if tamagotchi_ans == TmgEvent::ErrorDuringPurchase {
            msg::reply(TamagotchiFactoryEvent::ErrorDuringPurchase, 0)
                .expect("Error sending reply");
            return;
        }

        if let TmgEvent::CompletePrevPurchase(prev_purchase) = tamagotchi_ans {
            msg::reply(
                TamagotchiFactoryEvent::CompletePrevPurchase(prev_purchase),
                0,
            )
            .expect("Error sending reply");
            return;
        }

        let expected_ans = TmgEvent::AttributeBought(attribute_id);

        if tamagotchi_ans != expected_ans {
            panic!("Incorrect answer from tamagotchi contract");
        }

        msg::reply(TamagotchiFactoryEvent::AttributeBought(attribute_id), 0)
            .expect("Error sending reply");
    }

    pub async fn check_tamagotchi_state(&self, tamagotchi_id: TamagotchiId) {
        let tamagotchi_address = self.get_tamagotchi_address(tamagotchi_id);
        let tamagotchi_ans = Self::send_message(&tamagotchi_address, TmgAction::CheckState).await;

        let response = match tamagotchi_ans {
            TmgEvent::FeedMe => TamagotchiFactoryEvent::FeedMe,
            TmgEvent::PlayWithMe => TamagotchiFactoryEvent::PlayWithMe,
            TmgEvent::WantToSleep => TamagotchiFactoryEvent::WantToSleep,
            TmgEvent::AllGood => TamagotchiFactoryEvent::AllGood,
            _ => {
                panic!("Incorrect answer from tamagotchi contract");
            }
        };

        msg::reply(response, 0).expect("Error sending reply");
    }

    pub async fn reserve_gas_to_tamagotchi(
        &self,
        tamagotchi_id: TamagotchiId,
        reservation_amount: u64,
        duration: u32,
    ) {
        let tamagotchi_address = self.get_tamagotchi_address(tamagotchi_id);
        let tamagotchi_ans = Self::send_message(
            &tamagotchi_address,
            TmgAction::ReserveGas {
                reservation_amount,
                duration,
            },
        )
        .await;

        if tamagotchi_ans != TmgEvent::GasReserved {
            panic!("Incorrect answer from tamagotchi contract");
        }

        msg::reply(TamagotchiFactoryEvent::GasReserved, 0).expect("Error sending reply");
    }

    pub fn get_tamagotchi_address(&self, tamagotchi_id: TamagotchiId) -> ActorId {
        *self
            .id_to_address
            .get(&tamagotchi_id)
            .expect("The escrow with indicated id does not exist")
    }

    pub async fn send_message(
        tamagotchi_address: &ActorId,
        tamagotchi_payload: TmgAction,
    ) -> TmgEvent {
        msg::send_for_reply_as::<_, TmgEvent>(
            *tamagotchi_address,
            tamagotchi_payload,
            msg::value(),
            0,
        )
        .expect("Error during a sending message to a Escrow program")
        .await
        .expect("Unable to decode EscrowEvent")
    }
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum TamagotchiFactoryAction {
    CreateTamagotchi {
        name: String,
    },
    // Agregar las demas acciones para los tamagotchis
    TamagotchiName(TamagotchiId),
    TamagotchiAge(TamagotchiId),
    FeedTamagotchi(TamagotchiId),
    PlayWithTamagotchi(TamagotchiId),
    SleepTamagotchi(TamagotchiId),
    TransferTamagotchi {
        tamagotchi_id: TamagotchiId,
        new_owner: ActorId,
    },
    ApproveUser {
        tamagotchi_id: TamagotchiId,
        user: ActorId,
    },
    RemoveUserApproval(TamagotchiId),
    SetFTTokenContract {
        tamagotchi_id: TamagotchiId,
        ft_token_contract: ActorId,
    },
    ApproveTokens {
        tamagotchi_id: TamagotchiId,
        user: ActorId,
        amount: u128,
    },
    BuyAttributeToTamagotchi {
        tamagotchi_id: TamagotchiId,
        store_id: ActorId,
        attribute_id: AttributeId,
    },
    CheckTamagotchiState(TamagotchiId),
    ReserveGasToTamagotchi {
        tamagotchi_id: TamagotchiId,
        reservation_amount: u64,
        duration: u32,
    },
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum TamagotchiFactoryEvent {
    TamagotchiCreated {
        tamagotchi_id: TamagotchiId,
        tamagotchi_address: ActorId,
    },
    Name(String),         //
    Age(u64),             //
    Fed,                  //
    Entertained,          //
    Slept,                //
    Transferred(ActorId), //
    Approved(ActorId),    //
    ApprovalRevoked,      //
    FTTokenContractSet,   //
    TokensApproved {
        account: ActorId,
        amount: u128,
    }, //
    ApprovalError,        //
    AttributeBought(AttributeId), //
    CompletePrevPurchase(AttributeId), //
    ErrorDuringPurchase,  //
    // TODO: 3️⃣ Add new events
    FeedMe,          //
    PlayWithMe,      //
    WantToSleep,     //
    AllGood,         // extra field to return if the user check state //
    MakeReservation, //
    GasReserved,
}
