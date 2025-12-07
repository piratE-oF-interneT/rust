use crate::models::FileEvent;
use lazy_static::lazy_static;
use tokio::sync::Mutex;

lazy_static! {
    pub static ref MEMPOOL: Mutex<Vec<FileEvent>> = Mutex::new(vec![]);
}

pub async fn add_event(event: FileEvent) {
    MEMPOOL.lock().await.push(event);
}

pub async fn drain_events() -> Vec<FileEvent> {
    MEMPOOL.lock().await.drain(..).collect()
}
