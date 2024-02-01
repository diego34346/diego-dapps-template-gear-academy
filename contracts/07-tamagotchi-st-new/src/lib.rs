#![no_std]

use gstd::{
    collections::{BTreeMap, BTreeSet},
    exec, msg,
    prelude::*,
    ActorId,
};
use sharded_fungible_token_io::{FTokenAction, FTokenEvent, LogicAction};
use tamagotchi_store_io::*;

static mut STORE: Option<AttributeStore> = None;

#[no_mangle]
extern fn init() {
    let StoreInit {
        ft_contract_id,
        cost_to_upgrade_weapons,
    } = msg::load().expect("Unable to decode `ActorId`");
    let store = AttributeStore {
        admin: msg::source(),
        ft_contract_id,
        cost_to_upgrade_weapons,
        ..Default::default()
    };
    unsafe { STORE = Some(store) };
}

#[gstd::async_main]
async fn main() {
    let action: StoreAction = msg::load().expect("Unable to decode `StoreAction");
    let store: &mut AttributeStore =
        unsafe { STORE.as_mut().expect("The contract is not initialized") };
    match action {
        StoreAction::CreateAttribute {
            attribute_id,
            attribute_upgrade_id,
            attr_metadata,
            can_upgrade,
            price,
        } => store.add_attribute(
            attribute_id,
            attribute_upgrade_id,
            &attr_metadata,
            can_upgrade,
            price,
        ),
        StoreAction::BuyAttribute { attribute_id } => store.purchase_attribute(attribute_id).await,
        StoreAction::GetAttributes { tamagotchi_id } => {
            store.get_tamagotchi_attributes(&tamagotchi_id)
        }
        StoreAction::SetFtContractId { ft_contract_id } => {
            store.set_ft_contract_id(&ft_contract_id)
        }
        StoreAction::RemoveTx { tamagotchi_id } => store.remove_transaction(&tamagotchi_id),
        StoreAction::UpgradeAttribute { attribute_id } => {
            // store.update_attributes(attribute_id);
            store.upgrade_attribute(attribute_id).await;
        }
    }
}

#[no_mangle]
extern fn state() {
    let store = unsafe { STORE.as_ref().expect("The contract is not initialized") };

    msg::reply(
        AttributeStore {
            admin: store.admin,
            ft_contract_id: store.ft_contract_id,
            attributes: store.attributes.clone(),
            owners: store.owners.clone(),
            transaction_id: store.transaction_id,
            transactions: store.transactions.clone(),
            cost_to_upgrade_weapons: store.cost_to_upgrade_weapons,
            improvable_attributes: store.improvable_attributes.clone(),
        },
        0,
    )
    .expect("Failed to share state");
}
