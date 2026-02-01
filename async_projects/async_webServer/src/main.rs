use std::{
    f32::consts::E,
    sync::{Arc, Mutex},
    time::Duration,
};

use tokio::{
    sync::{mpsc, oneshot},
    time::{sleep, timeout},
};

#[tokio::main]
async fn main() {
    println!("async web server");

    let (job_tx, job_rx) = mpsc::channel::<Job>(3);

    let job_rx = Arc::new(tokio::sync::Mutex::new(job_rx));

    const worker_cnt: usize = 3;

    for id in 1..=worker_cnt {
        let rx = Arc::clone(&job_rx);

        tokio::spawn(async move {
            worker(id, rx).await;
        });
    }

    tokio::spawn(async move {
        for i in 1..=5 {
            job_tx.send(Job { id: i, attempts: 0 }).await.unwrap();
            sleep(Duration::from_secs(1));
        }
    });

    sleep(Duration::from_secs(10)).await;
    println!("system exited cleanly");
}

struct Job {
    id: usize,
    attempts: u8, // to perform fixed retries
}

async fn execute_job(job: &Job) -> Result<(), &'static str> {
    if job.id == 4 {
        return Err("simulated error on job id 4");
    } else {
        sleep(Duration::from_secs(1)).await;
        return Ok(());
    }
}

async fn run_with_timeout(job: &Job) -> Result<(), &'static str> {
    // timeout -> if job is executed wihting time then result is returned otherwise error is returned
    match timeout(Duration::from_secs(2), execute_job(&job)).await {
        Ok(res) => res,
        Err(_) => Err("timeout"),
    }
}

async fn worker(worker_id: usize, rx: Arc<tokio::sync::Mutex<mpsc::Receiver<Job>>>) {
    println!("worker {worker_id} started");

    loop {
        let job = {
            let mut guard = rx.lock().await;
            guard.recv().await
        };

        let Some(mut job) = job else {
            break;
        };

        println!("worker {worker_id} got job {}", job.id);

        let mut success = false;

        for attempt in 1..=3 {
            job.attempts = attempt;

            println!("worker {worker_id}  , job {}attempt {}", job.id, attempt);

            match run_with_timeout(&job).await {
                Ok(()) => {
                    println!(
                        "worker {worker_id} completed job {} in attemps {}",
                        job.id, job.attempts
                    );
                    success = true;
                    break;
                }
                Err(e) => {
                    println!(
                        "worker {worker_id} failed run job {} in attempt {}",
                        job.id, job.attempts
                    );
                    sleep(Duration::from_secs(1)).await;
                }
            }
        }

        if !success {
            println!("worker {worker_id}")
        }
    }
}
