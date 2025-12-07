use crate::blockchain::Blockchain;
use axum::{Json, Router, routing::get};
use std::sync::{Arc, Mutex};

pub fn app(chain: Arc<Mutex<Blockchain>>) -> Router {
    Router::new().route(
        "/chain",
        get(move || {
            let c = chain.lock().unwrap();
            Json(&c.chain)
        }),
    )
}
