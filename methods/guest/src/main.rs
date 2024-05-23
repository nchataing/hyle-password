#![no_main]
#![no_std]

extern crate alloc;

use risc0_zkvm::{
    guest::env,
    sha::{Impl, Sha256}
};
use hyle_contract::{HyleInput, HyleOutput};

use alloc::string::String;

risc0_zkvm::guest::entry!(main);

pub fn main() {
    let input: HyleInput<(String, String)> = env::read();

    let (current_password, next_password) = input.program_inputs;

    let digest = *Impl::hash_bytes(&current_password.as_bytes());

    if input.initial_state != digest.as_bytes() {
        panic!("Wrong password")
    }

    let next_state = *Impl::hash_bytes(&next_password.as_bytes());

    env::commit(&HyleOutput {
        version: 1,
        block_number: input.block_number,
        block_time: input.block_time,
        sender: input.sender,
        caller: input.caller,
        tx_hash: input.tx_hash,
        program_outputs: ":shrug:",
        initial_state: input.initial_state,
        next_state: next_state.as_bytes().to_vec()
    })
}
