use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Transaction {
  #[serde(skip_deserializing)]
  pub id: String,
  pub destination: String,
  pub source: String,
  pub amount: u64,
}
