#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;

use alloc::{collections::BTreeMap, vec::Vec};

use casper_contract::contract_api::storage;
use casper_types::URef;

#[no_mangle]
pub extern "C" fn call() {
    let mut events = Vec::new();
    let mut param = BTreeMap::new();
    param.insert("token_name", "apple");

    param.insert("token_supply", "10");
    param.insert("token_symbol", "APPLE");
    events.push(param);

    let events_uref = URef::from_formatted_str(
        "uref-2403d95c263d092c1b4af04568aba9a1a25fe846987f13e4aff7339d4ff658cc-007",
    )
    .unwrap();

    storage::dictionary_put(events_uref, "1", events);
}
