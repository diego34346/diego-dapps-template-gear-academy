#![no_std]

use gmeta::{InOut, Metadata, Out};
use gstd::{msg, prelude::*, ActorId};

pub struct ProgramMetadata;

// #[derive(Debug, Default, PartialEq, Eq, Encode, Decode, TypeInfo)]
#[derive(Default, Encode, Decode, TypeInfo, PartialEq, Eq, Debug)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum EscrowState {
    #[default]
    AwaitingPayment,
    AwaitingDelivery,
    Closed,
}

// #[derive(Encode, Decode, TypeInfo)]
#[derive(Default, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum EscrowEvent {
    #[default]
    FundsDeposited,
    DeliveryConfirmed,
    NotEnoughtFounds((u128, u128)),
    FundsPassed(u128),
}

//#[derive(Encode, Decode, TypeInfo)]
#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum EscrowAction {
    Deposit(ActorId),
    ConfirmDelivery(ActorId),
}

//#[derive(Default, Encode, Decode, TypeInfo)]
#[derive(Default, Encode, Decode, TypeInfo, Debug)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct Escrow {
    pub seller: ActorId,
    pub buyer: ActorId,
    pub price: u128,
    pub state: EscrowState,
}

impl Escrow {
    pub fn deposit(&mut self, user_address: ActorId) {
        assert_eq!(
            self.state,
            EscrowState::AwaitingPayment,
            "State must be `AwaitingPayment"
        );
        assert_eq!(
            user_address, self.buyer,
            "The message sender must be a buyer"
        );

        assert_eq!(
            msg::value(),
            self.price,
            "The attached value must be equal to set price"
        );

        self.state = EscrowState::AwaitingDelivery;

        msg::reply(EscrowEvent::FundsDeposited, 0)
            .expect("Error in reply EscrowEvent::FundsDeposited");
    }

    pub fn confirm_delivery(&mut self, user_address: ActorId) {
        assert_eq!(
            user_address, self.buyer,
            "The message sender must be a buyer"
        );
        assert_eq!(
            self.state,
            EscrowState::AwaitingDelivery,
            "State must be `AwaitingDelivery`"
        );
        msg::send(self.seller, "Buyer payment", self.price).expect("Error while sending funds");
        self.state = EscrowState::Closed;
        msg::reply(EscrowEvent::DeliveryConfirmed, 0).expect("Error in sending reply");
    }
}

//#[derive(Encode, Decode, TypeInfo)]
#[derive(Default, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct InitEscrow {
    pub seller: ActorId,
    pub buyer: ActorId,
    pub price: u128,
}

impl Metadata for ProgramMetadata {
    type Init = InOut<InitEscrow, String>;
    type Handle = InOut<EscrowAction, EscrowEvent>;
    type Reply = ();
    type Others = ();
    type Signal = ();
    type State = Out<Escrow>;
}
