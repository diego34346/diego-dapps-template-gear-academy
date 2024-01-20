#![no_std]
use escrow_io::{EscrowAction, EscrowEvent, InitEscrow};
use gmeta::{In, InOut, Metadata, Out};
use gstd::{collections::BTreeMap, msg, prelude::*, prog::ProgramGenerator, ActorId, CodeId};

pub type EscrowId = u64;

const GAS_FOR_CREATION: u64 = 1_000_000_000;

pub struct ProgramMetadata;

impl Metadata for ProgramMetadata {
    type Init = In<CodeId>;
    type Handle = InOut<FactoryAction, FactoryEvent>;
    type State = Out<EscrowFactory>;
    type Reply = ();
    type Others = InOut<FactoryAction, FactoryEvent>;
    type Signal = ();
}

#[derive(Encode, Decode, TypeInfo, Default, Debug)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct EscrowFactory {
    pub escrow_number: EscrowId,
    pub id_to_address: BTreeMap<EscrowId, ActorId>,
    pub escrow_code_id: CodeId,
}

impl EscrowFactory {
    pub async fn create_escrow(&mut self, seller: &ActorId, buyer: &ActorId, price: u128) {
        let (address, _) = ProgramGenerator::create_program_with_gas_for_reply::<InitEscrow>(
            self.escrow_code_id,
            InitEscrow {
                seller: *seller,
                buyer: *buyer,
                price,
            },
            GAS_FOR_CREATION,
            0,
            1_000_000_000,
        )
        .expect("Error during Escrow program initialization")
        .await
        .expect("Program was not initialized");
        self.escrow_number = self.escrow_number.saturating_add(1);
        self.id_to_address.insert(self.escrow_number, address);
        msg::reply(
            FactoryEvent::EscrowCreated {
                escrow_id: self.escrow_number,
                escrow_address: address,
            },
            0,
        )
        .expect("Error during a reply `FactoryEvent::ProgramCreated`");
    }

    pub async fn deposit(&self, escrow_id: EscrowId) {
        let escrow_address = self.get_escrow_address(escrow_id);
        Self::send_message(&escrow_address, EscrowAction::Deposit(msg::source())).await;
        msg::reply(FactoryEvent::Deposited(escrow_id), 0)
            .expect("Error during a reply `FactoryEvent::Deposited`");
    }

    pub async fn confirm_delivery(&self, escrow_id: EscrowId) {
        let escrow_address = self.get_escrow_address(escrow_id);
        Self::send_message(
            &escrow_address,
            EscrowAction::ConfirmDelivery(msg::source()),
        )
        .await;
        msg::reply(FactoryEvent::DeliveryConfirmed(escrow_id), 0)
            .expect("Error during a reply `FactoryEvent::DeliveryConfirmed`");
    }

    pub fn get_escrow_address(&self, escrow_id: EscrowId) -> ActorId {
        *self
            .id_to_address
            .get(&escrow_id)
            .expect("The escrow with indicated id does not exist")
    }

    pub async fn send_message(escrow_address: &ActorId, escrow_payload: EscrowAction) {
        msg::send_for_reply_as::<EscrowAction, EscrowEvent>(
            *escrow_address,
            escrow_payload,
            msg::value(),
            0,
        )
        .expect("Error during a sending message to a Escrow program")
        .await
        .expect("Unable to decode EscrowEvent");
    }
}

#[derive(Encode, Decode, TypeInfo, Debug)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum FactoryAction {
    CreateEscrow {
        seller: ActorId,
        buyer: ActorId,
        price: u128,
    },
    Deposit(EscrowId),
    ConfirmDelivery(EscrowId),
}

#[derive(Encode, Decode, TypeInfo, Debug)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum FactoryEvent {
    EscrowCreated {
        escrow_id: EscrowId,
        escrow_address: ActorId,
    },
    Deposited(EscrowId),
    DeliveryConfirmed(EscrowId),
}
