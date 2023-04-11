use methods::{PREF_MATCH_ELF, PREF_MATCH_ID};
use risc0_zkvm::Prover;

fn main() {
    // Make the prover.
    let mut prover =
        Prover::new(PREF_MATCH_ELF).expect("Prover should be constructed from valid ELF binary");

    // TODO: Implement communication with the guest here

    // Run prover & generate receipt
    let receipt = prover.run().expect(
        "Code should be provable unless it had an error or exceeded the maximum cycle limit",
    );

    // TODO: Implement code for transmitting or serializing the receipt for
    // other parties to verify here

    // Optional: Verify receipt to confirm that recipients will also be able to
    // verify your receipt
    receipt.verify(&PREF_MATCH_ID).expect(
        "Code you have proven should successfully verify; did you specify the correct method ID?",
    );
}
