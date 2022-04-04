use self::block::*;
use self::blockchain::*;
pub use self::transaction::*;
use sha2::{Digest, Sha256};
use std::sync::Mutex;

mod block;
mod blockchain;
mod transaction;

pub struct AppState {
  pub blockchain: Mutex<Blockchain>,
}

fn is_hash_valid(hash: &String) -> bool {
  let difficulty: usize = 4;
  hash.get(..difficulty).unwrap() == "0".repeat(difficulty)
}
