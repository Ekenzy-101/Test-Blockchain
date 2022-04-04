use super::*;
use serde::Serialize;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize)]
pub struct Blockchain {
  pub blocks: Vec<Block>,
  pub transactions: Vec<Transaction>,
  pub length: u64,
}

impl Default for Blockchain {
  fn default() -> Self {
    let mut instance = Self {
      blocks: Vec::new(),
      length: 0,
      transactions: Vec::new(),
    };
    instance.create_block("0".repeat(64));
    return instance;
  }
}

impl Blockchain {
  pub fn add_transaction(&mut self, mut transaction: Transaction) {
    transaction.id = Uuid::new_v4().to_string();
    self.transactions.push(transaction)
  }

  pub fn create_block(&mut self, previous_hash: String) -> Block {
    self.length += 1;
    let mut block = Block::new(
      self.length,
      previous_hash,
      self.transactions.drain(..).collect(),
    );
    block.mine();
    self.blocks.push(block.clone());
    block
  }

  pub fn get_previous_block(&self) -> Block {
    self.blocks[(self.length - 1) as usize].clone()
  }

  pub fn is_chain_valid(&self) -> bool {
    let mut previous_block = &self.blocks[0];
    if !self.is_nonce_valid(&previous_block) {
      return false;
    }

    let mut block_index: u64 = 1;
    while block_index < self.length {
      let current_block = &self.blocks[block_index as usize];
      if current_block.previous_hash != previous_block.hash {
        return false;
      }

      if !self.is_nonce_valid(&current_block) {
        return false;
      }

      previous_block = current_block;
      block_index += 1;
    }

    true
  }

  fn is_nonce_valid(&self, block: &Block) -> bool {
    let mut block = block.clone();
    block.hash = String::new();
    is_hash_valid(&block.hash())
  }
}
