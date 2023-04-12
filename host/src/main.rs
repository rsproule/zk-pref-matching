use matcher_core::Inputs;
use methods::{PREF_MATCH_ELF, PREF_MATCH_ID};
use risc0_zkvm::{
    serde::{from_slice, to_vec},
    Prover,
};

fn main() {
    let mut prover =
        Prover::new(PREF_MATCH_ELF).expect("Prover should be constructed from valid ELF binary");
    // let group_a_preferences = vec![vec![0, 1, 2], vec![0, 2, 1], vec![1, 2, 0]];
    // let group_b_preferences = vec![vec![0, 1, 2], vec![2, 1, 0], vec![0, 2, 1]];
    let group_a_preferences = gen_pref(20);
    let group_b_preferences = gen_pref(20);
    let inputs = Inputs {
        preferences: (group_a_preferences, group_b_preferences),
        pref_hashes: vec![],
    };
    prover.add_input_u32_slice(&to_vec(&inputs).expect("input should be serializable"));

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

fn gen_pref(size: usize) -> Vec<Vec<usize>> {
    let mut v = Vec::new();
    for _ in 0..size {
        let mut vv = Vec::new();
        for j in 0..size {
            vv.push(j)
        }
        v.push(vv);
    }
    v
}
