use methods::{
    PROVER_ELF, PROVER_ID
};
use risc0_zkvm::{default_prover, ExecutorEnv};
use p256::{ecdsa::{SigningKey, Signature, signature::Signer}, EncodedPoint};
use rand_core::OsRng;

#[derive(serde::Serialize, serde::Deserialize)]
struct Input {
  verifying_key: EncodedPoint,
  message: Vec<u8>,
  signature: Signature
}

fn main() {

    // Generate a random signing key (private key)
    let signing_key = SigningKey::random(&mut OsRng);

    // Derive corresponding verifying key (public key)
    let verifying_key = signing_key.verifying_key();

    // Message to sign
    let message = b"Example message for signature";

    // Sign the message
    let signature: Signature = signing_key.sign(message);

    // Output the signature and public key for verification
    println!("Signature: {:?}", signature);
    println!("Public Key: {:?}", verifying_key);
    
    let encoded_verifying_key = verifying_key.to_encoded_point(true);
    let input = Input {
      verifying_key: encoded_verifying_key,
      message: message.to_vec(),
      signature: signature
    };

    let env = ExecutorEnv::builder()
      .write_slice(&bincode::serialize(&input).unwrap())
        .build()
        .unwrap();

    // Obtain the default prover.
    let prover = default_prover();

    let receipt = prover.prove_elf(env, PROVER_ELF).unwrap();
    // We read the result commited to the journal by the guest code.
    let (receipt_verifying_key, receipt_message): (EncodedPoint, Vec<u8>) =
        receipt.journal.decode().unwrap();

    assert_eq!(receipt_verifying_key, encoded_verifying_key);
    assert_eq!(receipt_message, message);

    println!("DONE");
}
