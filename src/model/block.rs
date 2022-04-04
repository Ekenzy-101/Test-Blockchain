use super::*;
use chrono::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct Block {
  pub index: u64,
  pub timestamp: i64,
  pub nonce: u64,
  #[serde(skip_serializing_if = "String::is_empty")]
  pub hash: String,
  pub previous_hash: String,
  pub transactions: Vec<Transaction>,
}

impl Block {
  pub fn new(index: u64, previous_hash: String, transactions: Vec<Transaction>) -> Self {
    Self {
      index,
      timestamp: Utc::now().timestamp(),
      nonce: 0,
      previous_hash,
      hash: String::new(),
      transactions,
    }
  }

  pub fn mine(&mut self) {
    let mut hash = self.hash();
    while self.nonce <= u64::MAX {
      if is_hash_valid(&hash) {
        self.hash = hash;
        break;
      }

      if self.nonce == u64::MAX {
        self.nonce = 0;
        self.timestamp = Utc::now().timestamp();
      }

      self.nonce += 1;
      hash = self.hash();
    }
  }

  pub fn hash(&self) -> String {
    let data = serde_json::to_vec(self).unwrap();
    format!("{:x}", Sha256::digest(data))
  }
}
