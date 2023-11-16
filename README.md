# Signature verification using Risc0-zkVM

A small example of a p256 signature verification using the Risc0 zkVM and the p256 crate.

## Build 

```bash
cargo build
```

## Different running modes

Run in dev mode (`WARNING: proving in dev mode. This will not generate valid, secure proofs.`), run locally or run with Bonsai. 

```bash
# Takes a few seconds to run
RISC0_DEV_MODE=1 cargo run

# Takes too long
cargo run

# takes about 1,5 min
BONSAI_API_KEY=<YOUR_API_KEY> BONSAI_API_URL=<BONSAI_URL> cargo run
```
