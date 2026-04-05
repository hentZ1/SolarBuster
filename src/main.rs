use solar_buster::modules::{
    config::{Args, Parser},
    http::{build_client, measure_noise},
    output::{banner, build_progress_bar},
    reader::read,
    scanner::dir_scanner,
};
use solar_buster::prelude::*;

const QUEUE_SIZE: usize = 1000;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let pb = Arc::new(build_progress_bar());

    banner(&args.url, &args.path);

    let client = build_client()?;

    let sem = Arc::new(Semaphore::new(args.workers));
    let (tx, mut rx) = mpsc::channel::<String>(QUEUE_SIZE);

    tokio::spawn(read(args.path, tx));
    let noise = measure_noise(args.url.clone(), client.clone()).await;

    while let Some(word) = rx.recv().await {
        let permit = sem.clone().acquire_owned().await.unwrap();
        let client = client.clone();
        let url = args.url.clone();
        let pb = pb.clone();
        tokio::spawn(async move {
            let _permit = permit;
            dir_scanner(client, url, word, noise, pb).await;
        });
    }
    Ok(())
}
