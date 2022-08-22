#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;

use alloc::{collections::BTreeMap, string::ToString, vec, vec::Vec};

use casper_contract::contract_api::{runtime, storage};
use casper_types::{CLType, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, Key, URef};

#[no_mangle]
fn test_unsafe() {
    let mut events = Vec::new();
    let mut param = BTreeMap::new();
    param.insert("token_name", "apple");

    param.insert("token_supply", "10");
    param.insert("token_symbol", "APPLE");
    events.push(param);

    for param in events {
        let _: URef = storage::new_uref(param);
    }
}

#[no_mangle]
fn test_safe() {
    let mut events = Vec::new();
    let mut param = BTreeMap::new();
    param.insert("token_name", "apple");

    param.insert("token_supply", "10");
    param.insert("token_symbol", "APPLE");
    events.push(param);

    let events_uref = runtime::get_key("events")
        .unwrap()
        .as_uref()
        .unwrap()
        .clone();

    let events_length_uref = runtime::get_key("events_length")
        .unwrap()
        .as_uref()
        .unwrap()
        .clone();
    let events_item_key: u8 = storage::read(events_length_uref).unwrap().unwrap();
    storage::dictionary_put(events_uref, &(events_item_key + 1).to_string(), events);
}

#[no_mangle]
pub extern "C" fn call() {
    let mut entrypoints = EntryPoints::new();
    let entrypoint1 = EntryPoint::new(
        "test_unsafe",
        vec![],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );
    let entrypoint2 = EntryPoint::new(
        "test_safe",
        vec![],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );
    entrypoints.add_entry_point(entrypoint1);
    entrypoints.add_entry_point(entrypoint2);

    let events_uref = storage::new_dictionary("events").unwrap();

    let mut named_keys = BTreeMap::new();
    named_keys.insert("events".to_string(), Key::from(events_uref));
    runtime::remove_key("events");

    named_keys.insert(
        "events_length".to_string(),
        Key::from(storage::new_uref(0u8)),
    );

    let (contracthash, _) = storage::new_contract(entrypoints, Some(named_keys), None, None);
    runtime::put_key("sse_contract", contracthash.into());
}
