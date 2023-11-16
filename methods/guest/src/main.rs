#![no_main]

use std::{vec::Vec, io::Read};
use risc0_zkvm::guest::env;
use p256::{ecdsa::{VerifyingKey, Signature, signature::Verifier}, EncodedPoint};

risc0_zkvm::guest::entry!(main);

#[derive(serde::Serialize, serde::Deserialize)]
struct Input {
  verifying_key: EncodedPoint,
  message: Vec<u8>,
  signature: Signature
}

pub fn main() {
    let start: usize = env::get_cycle_count();

    // read the input
    let mut input_bytes = Vec::<u8>::new();
    env::stdin().read_to_end(&mut input_bytes).unwrap();
    let input: Input = bincode::deserialize(&input_bytes).unwrap();
    let verifying_key = VerifyingKey::from_encoded_point(&input.verifying_key).unwrap();

    // Verify the signature, panicking if verification fails.
    verifying_key
      .verify(&input.message, &input.signature)
      .expect("ECDSA signature verification failed");

    // write public output to the journal
    env::commit(&(input.verifying_key, input.message));

    // 13.687.221, 13.690.504, 13.680.365, 13.677.344
    let end = env::get_cycle_count();
    eprintln!("total cycle count: {}", end - start);
}
