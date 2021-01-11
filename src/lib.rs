#![allow(unused_imports)]
#![allow(dead_code)]
use near_sdk::{
  borsh::{self, BorshDeserialize, BorshSerialize},
  collections::*,
  *,
};

mod utils;

#[cfg(target = "wasm32")]
#[global_allocator]
static ALLOC: near_sdk::wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Dummy {}

/// Default must be implemented for wasm compilation.
impl Default for Dummy {
  fn default() -> Self {
    panic!("Dummy")
  }
}
#[near_bindgen]
impl Dummy {
  #[init]
  pub fn new() -> Self {
    assert!(!env::state_exists(), "Already, initialized");
    Self {}
  }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
  fn test() {
    assert!(true);
  }
}
