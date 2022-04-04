use crate::model::*;
use actix_web::{
  body::BoxBody,
  get, post,
  web::{Data, Json},
  HttpRequest, HttpResponse, Responder,
};
use serde::Serialize;

#[derive(Serialize, Default)]
struct DefaultResponse {
  message: String,
}

impl Responder for DefaultResponse {
  type Body = BoxBody;

  fn respond_to(mut self, req: &HttpRequest) -> HttpResponse<Self::Body> {
    self.message = format!(
      "{} request is supported for this path {}",
      req.method().as_str(),
      req.path()
    );
    let body = serde_json::to_string(&self).unwrap();
    HttpResponse::NotFound().json(body)
  }
}

pub async fn default() -> impl Responder {
  DefaultResponse::default()
}

#[post("/blocks")]
pub async fn create_block(data: Data<AppState>) -> impl Responder {
  let mut blockchain = data.blockchain.lock().unwrap();
  let previous_block = blockchain.get_previous_block();
  let block = blockchain.create_block(previous_block.hash);
  HttpResponse::Created().json(block)
}

#[post("/transactions")]
pub async fn create_transaction(
  data: Data<AppState>,
  transaction: Json<Transaction>,
) -> impl Responder {
  let mut blockchain = data.blockchain.lock().unwrap();
  blockchain.add_transaction(transaction.into_inner());
  HttpResponse::Created().json(DefaultResponse {
    message: "Transaction created successfully".to_owned(),
  })
}

#[get("/blocks")]
pub async fn get_blocks(data: Data<AppState>) -> impl Responder {
  HttpResponse::Ok().json(&data.blockchain)
}

#[post("/blocks/validate")]
pub async fn validate_blocks(data: Data<AppState>) -> impl Responder {
  let blockchain = data.blockchain.lock().unwrap();

  let mut message = String::new();
  if blockchain.is_chain_valid() {
    message += "All good. The blockchain is valid";
  } else {
    message += "We have a problem. The blockchain is not valid";
  }

  HttpResponse::Ok().json(DefaultResponse { message })
}
