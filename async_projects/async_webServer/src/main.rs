use std::time::Duration;

use tokio::{
    sync::{mpsc, oneshot},
    time::sleep,
};

#[tokio::main]
async fn main() {
    println!("async web server");

    let (job_tx, mut job_rx) = mpsc::channel::<Job>(3);

    let (done_tx, mut done_rx) = mpsc::channel::<()>(3);

    let (shutdown_tx, mut shutdown_rx) = oneshot::channel::<()>();

    const MAX_WORKERS: usize = 3;

    let mut active_workers = 0;

    // job producer

    tokio::spawn(async move {
        for i in 1..20 {
            println!("incoming job {i}");

            job_tx.send(Job { id: i }).await.unwrap();

            sleep(Duration::from_millis(300)).await;
        }
    });

    // shutdown logic
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_secs(8)).await;

        println!("shutdown signal sent");

        let _ = shutdown_tx.send(());
    });

    // job controller -> assigns workers

    loop {
        tokio::select! {
            Some(job) = job_rx.recv() , if active_workers < MAX_WORKERS => {
                active_workers += 1;

                let done_tx = done_tx.clone();

                tokio::spawn(async move{
                    worker(active_workers, job).await;
                    done_tx.send(()).await.unwrap();

                });
            },
            Some(()) = done_rx.recv() => {
                active_workers -= 1;
            },
            _ = &mut shutdown_rx => {
                println!("controller shutting down");
                break;
            }
        }
    }
}

struct Job {
    id: usize,
}

async fn worker(id: usize, job: Job) {
    println!("worker {id} started job {}", job.id);
    sleep(Duration::from_secs(1)).await;

    println!("worker {id} finished job {}", job.id);
}
