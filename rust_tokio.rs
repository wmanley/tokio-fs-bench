use std::time::Instant;
use tokio::{
    fs::{File, OpenOptions},
    io::{AsyncReadExt, AsyncWriteExt},
    task::JoinHandle,
};

async fn compute() {
    let handles: Vec<JoinHandle<_>> = (0..1000)
        .map(|_| {
            tokio::spawn(async move {
                let mut buffer = [0; 10];
                {
                    let mut dev_urandom = File::open("/dev/urandom").await.unwrap();
                    dev_urandom.read_exact(&mut buffer).await.unwrap();
                }
                let mut dev_null = OpenOptions::new()
                    .append(true)
                    .open("/dev/null")
                    .await
                    .unwrap();
                dev_null.write_all(&mut buffer).await.unwrap();
            })
        })
        .collect();
    for handle in handles {
        handle.await.unwrap();
    }
}

#[tokio::main]
async fn main() {
    // warmup
    compute().await;

    let before = Instant::now();
    for _ in 0..1000 {
        compute().await;
    }
    let elapsed = before.elapsed();
    println!(
        "{:?} total, {:?} avg per iteration",
        elapsed,
        elapsed / 1000
    );
}
