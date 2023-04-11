use matcher_core::Inputs;
use methods::{PREF_MATCH_ELF, PREF_MATCH_ID};
use risc0_zkvm::{Prover, serde::{to_vec, from_slice}};

fn main() {
    // Make the prover.
    let mut prover =
        Prover::new(PREF_MATCH_ELF).expect("Prover should be constructed from valid ELF binary");

    let men_preferences = vec![vec![0, 1, 2], vec![0, 2, 1], vec![1, 2, 0]];
    let women_preferences = vec![vec![0, 1, 2], vec![2, 1, 0], vec![0, 2, 1]];
    let inputs = Inputs {
        preferences: (men_preferences, women_preferences),
        pref_hashes: vec![],
    };
    // TODO: Implement communication with the guest here
    prover.add_input_u32_slice(&to_vec(&inputs).expect("input should be serializable"));

    // Run prover & generate receipt
    let receipt = prover.run().expect(
        "Code should be provable unless it had an error or exceeded the maximum cycle limit",
    );

    receipt.verify(&PREF_MATCH_ID).expect(
        "Code you have proven should successfully verify; did you specify the correct method ID?",
    );
    let vec = receipt.journal;
    let matches: Vec<usize> = from_slice(&vec).unwrap();
    println!("Proven matches {matches:?}, in {:?} cycles", prover.cycles);
}
