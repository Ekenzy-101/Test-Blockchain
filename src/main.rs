use crate::handler::*;
use crate::model::*;
use actix_web::{web, App, HttpServer};
use std::sync::Mutex;

mod handler;
mod model;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Running on localhost");
    let state = web::Data::new(AppState {
        blockchain: Mutex::default(),
    });
    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(create_block)
            .service(create_transaction)
            .service(get_blocks)
            .service(validate_blocks)
            .default_service(web::to(default))
    })
    .bind(("127.0.0.1", 5000))?
    .shutdown_timeout(3600)
    .workers(2)
    .run()
    .await
}

// fn main() {
//     let mut block = Block::new(1);
//     block.mine();
//     println!("{:?}", block);
// }
