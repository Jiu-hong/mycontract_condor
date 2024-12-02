#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;

use alloc::{string::String, vec::Vec};

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    ApiError, CLType, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, Key,
};

const KEY_NAME: &str = "my-key-name";
const RUNTIME_ARG_NAME: &str = "message";

/// An error enum which can be converted to a `u16` so it can be returned as an `ApiError::User`.
#[repr(u16)]
enum Error {
    KeyAlreadyExists = 0,
    KeyMismatch = 1,
}

impl From<Error> for ApiError {
    fn from(error: Error) -> Self {
        ApiError::User(error as u16)
    }
}

#[no_mangle]
pub extern "C" fn apple() {}
pub fn apple_ep() -> EntryPoint {
    EntryPoint::new(
        String::from("apple"),
        Vec::new(),
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Called,
        casper_types::EntryPointPayment::Caller,
    )
}

/// Returns the default set of CEP-18 token entry points.
pub fn generate_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(apple_ep());
    entry_points
}

#[no_mangle]
pub extern "C" fn call() {
    let entry_points = generate_entry_points();
    let (contract_hash, _) = storage::new_contract(
        entry_points,
        None,
        Some(String::from("my_hash")),
        None,
        None,
    );

    runtime::put_key("apple_contract", contract_hash.into());
}
