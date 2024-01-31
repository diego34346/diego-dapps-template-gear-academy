#![no_std]

use gmeta::{In, InOut, Metadata as GMetadata, Out};
use sharded_fungible_token_io::{FTokenAction, FTokenEvent, LogicAction};

use gstd::{collections::{BTreeMap, BTreeSet},prelude::*,ActorId,msg,exec
};

pub type AttributeId = u32;
pub type Price = u128;
pub type TamagotchiId = ActorId;
pub type TransactionId = u64;





#[derive(Default, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct AttributeStore {
    pub admin: ActorId,
    pub ft_contract_id: ActorId,    
    pub cost_to_upgrade_weapons: Price,
    pub attributes: BTreeMap<AttributeId, (AttrMetadata, Price)>,
    pub improvable_attributes: BTreeMap<AttributeId, AttributeId>,
    pub owners: BTreeMap<TamagotchiId, BTreeSet<AttributeId>>,
    pub transaction_id: TransactionId,
    pub transactions: BTreeMap<TamagotchiId, (TransactionId, AttributeId)>,
}


#[derive(Encode, Decode, Clone, TypeInfo, Debug)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct AttrMetadata {
    pub title: String,
    pub description: String,
    pub media: String,
}

#[derive(Encode, Decode, TypeInfo, Debug)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct StoreInit {
    pub ft_contract_id: ActorId, 
    pub cost_to_upgrade_weapons: Price
}

#[derive(Encode, Decode, TypeInfo, Debug)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum StoreAction {
    CreateAttribute {
        attribute_id: AttributeId,
        attribute_upgrade_id: AttributeId,
        attr_metadata: AttrMetadata,
        can_upgrade: bool,
        price: Price,
    },
    BuyAttribute {
        attribute_id: AttributeId,
    },
    UpgradeAttribute {
        attribute_id: AttributeId  
    },
    GetAttributes {
        tamagotchi_id: TamagotchiId,
    },
    SetFtContractId {
        ft_contract_id: ActorId,
    },
    RemoveTx {
        tamagotchi_id: TamagotchiId,
    },
    
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum StoreEvent {
    AttributeCreated { attribute_id: AttributeId },
    AttributeSold { success: bool },
    AttributeUpgrade { success: bool },
    Attributes { attributes: BTreeSet<AttributeId> },
    CompletePrevTx { attribute_id: AttributeId },
    FtContractIdSet { ft_contract_id: ActorId },
    TxRemoved { tamagotchi_id: ActorId },
    BuyTheAttributeToUpdateIt,
    AttributeCannotBeImproved
}

impl AttributeStore {
    pub fn add_attribute(
        &mut self,
        attribute_id: AttributeId,
        attribute_upgrade_id: AttributeId,
        metadata: &AttrMetadata,
        can_upgrade: bool,
        price: Price,
    ) {
        assert_eq!(msg::source(), self.admin, "Only admin can add attributes");

        if self
            .attributes
            .insert(attribute_id, (metadata.clone(), price))
            .is_some()
        {
            panic!("Attribute with that ID already exists");
        }
        
        if can_upgrade {
            self.improvable_attributes.insert(attribute_id, attribute_upgrade_id);
        }

        msg::reply(StoreEvent::AttributeCreated { attribute_id }, 0)
            .expect("Error in sending a reply `StoreEvent::AttributeCreated");
    }
    
    pub async fn purchase_attribute(&mut self, attribute_id: AttributeId) {
        let (transaction_id, attribute_id) = if let Some((transaction_id, prev_attribute_id)) =
            self.transactions.get(&msg::source())
        {
            if attribute_id != *prev_attribute_id {
                msg::reply(
                    StoreEvent::CompletePrevTx {
                        attribute_id: *prev_attribute_id,
                    },
                    0,
                )
                .expect("Error in sending a reply `StoreEvent::CompletePrevTx`");
                return;
            }
            (*transaction_id, *prev_attribute_id)
        } else {
            let current_transaction_id = self.transaction_id;
            self.transaction_id = self.transaction_id.wrapping_add(1);
            self.transactions
                .insert(msg::source(), (current_transaction_id, attribute_id));
            (current_transaction_id, attribute_id)
        };

        let result = self.sell_attribute(transaction_id, attribute_id).await;
        self.transactions.remove(&msg::source());

        msg::reply(StoreEvent::AttributeSold { success: result }, 0)
            .expect("Error in sending a reply `StoreEvent::AttributeSold`");
    }

    pub async fn sell_attribute(
        &mut self,
        transaction_id: TransactionId,
        attribute_id: AttributeId,
    ) -> bool {
        let (_, price) = self
            .attributes
            .get(&attribute_id)
            .expect("Can`t get attribute_id");

        if transfer_tokens(
            transaction_id,
            &self.ft_contract_id,
            &msg::source(),
            &exec::program_id(),
            *price,
        )
        .await
        .is_ok()
        {
            self.owners
                .entry(msg::source())
                .and_modify(|attributes| {
                    attributes.insert(attribute_id);
                })
                .or_insert_with(|| [attribute_id].into());
            return true;
        }
        false
    }
    
    pub async fn upgrade_attribute(&mut self, attribute_id: AttributeId) {
        let caller = msg::source();
        
        let Some(&upgrade_id) = self.improvable_attributes.get(&attribute_id) else {
            msg::reply(StoreEvent::AttributeCannotBeImproved, 0)
                .expect("Error in sending reply");
            return;  
        };
        
        if !self.owners.contains_key(&caller) {
            msg::reply(StoreEvent::BuyTheAttributeToUpdateIt, 0)
                .expect("Error sendig reply");
            return;
        }
        
        let tamagotchi_attributes = self.owners.get(&caller).unwrap();
        if !tamagotchi_attributes.contains(&attribute_id) {
            msg::reply(StoreEvent::BuyTheAttributeToUpdateIt, 0)
                .expect("Error sendig reply");
            return;
        } 
        
        let (transaction_id, attribute_id) = if let Some((transaction_id, prev_attribute_id)) =
            self.transactions.get(&msg::source())
        { 
            if attribute_id != *prev_attribute_id {
                msg::reply(
                    StoreEvent::CompletePrevTx {
                        attribute_id: *prev_attribute_id,
                    },
                    0,
                )
                .expect("Error in sending a reply `StoreEvent::CompletePrevTx`");
                return;
            }
            (*transaction_id, *prev_attribute_id)
        } else {
            let current_transaction_id = self.transaction_id;
            self.transaction_id = self.transaction_id.wrapping_add(1);
            self.transactions
                .insert(msg::source(), (current_transaction_id, attribute_id));
            (current_transaction_id, attribute_id)
        };
        
        let result = self.apply_attribute_upgrade(transaction_id, attribute_id).await;
        
        if result {
            self.owners
                .entry(caller)
                .and_modify(|attributes| {
                    attributes.remove(&attribute_id);
                    attributes.insert(upgrade_id);
                });
        }
        
        msg::reply(StoreEvent::AttributeUpgrade { success: result }, 0)
            .expect("Error in sending a reply `StoreEvent::AttributeSold`");
    }
    
    pub async fn apply_attribute_upgrade(
        &mut self, 
        transaction_id: TransactionId,
        attribute_id: AttributeId,
    ) -> bool {
        if transfer_tokens(
            transaction_id,
            &self.ft_contract_id,
            &msg::source(),
            &exec::program_id(),
            self.cost_to_upgrade_weapons,
        )
        .await
        .is_ok()
        {
            self.owners
                .entry(msg::source())
                .and_modify(|attributes| {
                    attributes.insert(attribute_id);
                })
                .or_insert_with(|| [attribute_id].into());
            return true;
        }
        false
    }
    
    pub fn can_upgrade_attribute() -> bool {
        false
    }

    pub fn get_tamagotchi_attributes(&self, tmg_id: &TamagotchiId) {
        let attributes = self.owners.get(tmg_id).unwrap_or(&BTreeSet::new()).clone();
        msg::reply(StoreEvent::Attributes { attributes }, 0)
            .expect("Error in sending a reply `StoreEvent::Attributes`");
    }

    pub fn set_ft_contract_id(&mut self, ft_contract_id: &ActorId) {
        assert_eq!(
            msg::source(),
            self.admin,
            "Only admin can set fungible token contract"
        );
        self.ft_contract_id = *ft_contract_id;
        msg::reply(
            StoreEvent::FtContractIdSet {
                ft_contract_id: *ft_contract_id,
            },
            0,
        )
        .expect("Error in sending a reply `StoreEvent::FtContractIdSet`");
    }

    pub fn remove_transaction(&mut self, tmg_id: &TamagotchiId) {
        assert_eq!(
            msg::source(),
            self.admin,
            "Only admin can set remove transactions"
        );
        self.transactions.remove(tmg_id);
        msg::reply(
            StoreEvent::TxRemoved {
                tamagotchi_id: *tmg_id,
            },
            0,
        )
        .expect("Error in sending a reply `StoreEvent::TxRemoved`");
    }
}

pub async fn transfer_tokens(
    transaction_id: TransactionId,
    token_address: &ActorId,
    from: &ActorId,
    to: &ActorId,
    amount_tokens: u128,
) -> Result<(), ()> {
    let reply = msg::send_for_reply_as::<_, FTokenEvent>(
        *token_address,
        FTokenAction::Message {
            transaction_id,
            payload: LogicAction::Transfer {
                sender: *from,
                recipient: *to,
                amount: amount_tokens,
            },
        },
        0,
        0,
    )
    .expect("Error in sending a message `FTokenAction::Message`")
    .await;

    match reply {
        Ok(FTokenEvent::Ok) => Ok(()),
        _ => Err(()),
    }
}








pub struct ProgramMetadata;

impl GMetadata for ProgramMetadata {
    type Init = In<StoreInit>;
    type Handle = InOut<StoreAction, StoreEvent>;
    type Reply = ();
    type Others = ();
    type Signal = ();
    type State = Out<AttributeStore>;
}