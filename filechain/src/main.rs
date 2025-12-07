mod api;
mod blockchain;
mod crypto;
mod mempool;
mod models;
mod p2p;
mod storage;

use blockchain::Blockchain;
use std::sync::{Arc, Mutex};
use tokio::join;

#[tokio::main]
async fn main() {
    let chain = Arc::new(Mutex::new(Blockchain::new()));

    let api = axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(api::app(chain.clone()).into_make_service());

    let p2p = p2p::start_p2p();

    join!(api, p2p);
}
