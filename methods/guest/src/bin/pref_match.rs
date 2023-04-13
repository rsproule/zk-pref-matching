#![no_main]

use matcher_core::{gale_shapley, Inputs, Output};
use risc0_zkvm::{guest::env, sha::Digest};
use risc0_zkvm::sha::{Impl, Sha256};
use risc0_zkvm::serde::{to_vec};

risc0_zkvm::guest::entry!(main);

pub fn main() {
    let inputs: Inputs = env::read();
    // TODO: check the hash is correct for the preferences,
    // guarantee the server isn't mutating. Should commit the hashes so
    // that users can actually verify
    let preferences = inputs.preferences;
    let matches = gale_shapley(&preferences.0, &preferences.1);
    // TODO: probably want to salt these hashes to prevent bruteforcing.
    let mut hashes: Vec<Digest> = Vec::new();
    for preference in preferences.0 {
        let h = Impl::hash_words(&to_vec(&preference).unwrap());
        hashes.push(*h);
    }
    for preference in preferences.1 {
        let h = Impl::hash_words(&to_vec(&preference).unwrap());
        hashes.push(*h);
    }

    // TODO: encrypt the results?
    // This assumes that this server is being run by trusted entity
    env::commit(&Output {
        stable_matches: matches,
        hashes,
    });
}
