// TODO: Rename this file to change the name of this method from METHOD_NAME

#![no_main]
// If you want to try std support, also update the guest Cargo.toml file
// #![no_std]  // std support is experimental
use matcher_core::{Inputs, gale_shapley};
use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

pub fn main() {
    let inputs: Inputs = env::read();
    // TODO: check the hash is correct  
    let preferences = inputs.preferences;
    let matches = gale_shapley(preferences.0, preferences.1);
    
    env::commit(&matches);
}
