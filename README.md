# Hylé example RISC Zero smart contract

This repository provides an example smart contract for Hylé, implementing a simple password contract

## Installing RISC Zero

Please refer to [RiscZero's installation guide](https://dev.risczero.com/api/zkvm/install)

## Reproducible builds

RISC Zero provides using a docker setup. Simply run
```bash
cargo risczero build --manifest-path methods/guest/Cargo.toml
```
to build the smart contract.

## Running the smart contract

```bash
# to run without generating a proof
RISC0_DEV_MODE=1 cargo run --release
# to generate the proof
```

### Verifying locally

Install the [Hylé RISC Zero verifier](https://github.com/Hyle-org/hyle-risc-zero-verifier).
You can then verify proofs using:
```sh
# The verifier currently expects no `0x` prefix. Pass data as base64 values.
cargo run -p risc0-verifier [program_id] [path_to_proof] [initial_state] [final_state]
```
If the proof is malformed, or doesn't respect the rules of the smart contract, the verifier will return an error.

## Verifying on Hylé

Once you [installed the CLI](https://docs.hyle.eu/getting-started/hyled-install-instructions/) and got [connected to devnet](https://docs.hyle.eu/getting-started/connect-to-devnet/), you should be able to [_register_ and _execute_ for your contract](https://docs.hyle.eu/getting-started/your-first-smart-contract/).
