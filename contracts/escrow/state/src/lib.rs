#![no_std]
use escrow_io::{EscrowState, ProgramMetadata};
use gmeta::{metawasm, Metadata};
#[allow(unused_imports)]
use gstd::{prelude::*, ActorId, Vec};

#[metawasm]
pub mod metafns {
    pub type State = <ProgramMetadata as Metadata>::State;

    pub fn seller(state: State) -> ActorId {
        // Desestructura la tupla para acceder a los campos
        let (_, escrow) = state;
        escrow.seller
    }

    pub fn buyer(state: State) -> ActorId {
        let (_, escrow) = state;
        escrow.buyer
    }

    pub fn escrow_state(state: State) -> EscrowState {
        let (_, escrow) = state;
        escrow.state
    }
}
