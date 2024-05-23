use methods::METHOD_ELF;
use risc0_zkvm::{default_prover, sha::Digestible, ExecutorEnv};
use serde_json;

use base64::prelude::*;
use clap::{Parser, Subcommand};
use hyle_contract::{HyleInput, HyleOutput};

fn main() {
    let receipt = prove();

    let claim = receipt.inner.get_claim().unwrap();

    let receipt_json = serde_json::to_string(&receipt).unwrap();
    std::fs::write("proof.json", receipt_json).unwrap();

    let hyle_output = receipt.journal.decode::<HyleOutput<String>>().unwrap();

    let initial_state_b64 = BASE64_STANDARD.encode(&hyle_output.initial_state);
    let next_state_b64 = BASE64_STANDARD.encode(&hyle_output.next_state);
    let block_number = hyle_output.block_number;
    let block_time = hyle_output.block_time;
    let program_outputs = hyle_output.program_outputs;

    println!("{}", "-".repeat(20));
    println!("Method ID: {:?} (hex)", claim.pre.digest());
    println!("proof.json written, transition from {} to {}", initial_state_b64, next_state_b64);
    println!("Aiming block {} at time {}.", block_number, block_time);
    println!("Program outputted {:?}", program_outputs);
}

fn prove() -> risc0_zkvm::Receipt {
    let env = ExecutorEnv::builder()
        .write(&HyleInput {
            initial_state: vec![0x93, 0x69, 0x97, 0x25, 0x7c, 0xba, 0xd5, 0x79, 0xbd, 0xb4, 0x59, 0x0b, 0x1b, 0xe8, 0xb4, 0xd9, 0x90, 0xad, 0x3f, 0x81, 0xa6, 0xee, 0x41, 0x62, 0xdc, 0xb3, 0x99, 0xa6, 0x92, 0x5f, 0x71, 0x07],
            sender: "".to_string(), //TODO
            caller: "".to_string(), //TODO
            block_number: 0, //TODO
            block_time: 0, //TODO
            tx_hash: vec![1], //TODO
            program_inputs: ("chat1", "password"),
        })
        .unwrap()
        .build()
        .unwrap();

    let prover = default_prover();
    let binary =
        std::fs::read("target/riscv-guest/riscv32im-risc0-zkvm-elf/docker/method/method")
            .expect("Could not read ELF binary at target/riscv-guest/riscv32im-risc0-zkvm-elf/docker/method/method");
    prover.prove(env, &binary).unwrap()
}
