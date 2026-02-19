use clap::Parser;
use reqwest::Client;
use std::{sync::Arc, time::Duration};

use tokio::{
    fs::File,
    io::{AsyncBufReadExt, BufReader},
    sync::{Semaphore, mpsc},
};

#[derive(Parser, Debug)]
#[command(
    name = "SolarBuster",
    version,
    about = "Fast web directory enumerator in Rust"
)]
struct Args {
    #[arg(short = 'u', long = "url")]
    url: String,

    #[arg(short = 'w', long = "wordlist")]
    path: String,
}

const QUEUE_SIZE: usize = 1000;
const WORKERS: usize = 50;

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let url = args.url;
    let path = args.path;

    let (tx, mut rx) = mpsc::channel::<String>(QUEUE_SIZE);

    let sem = Arc::new(Semaphore::new(WORKERS));

    tokio::spawn(reader(path, tx));

    let client = Client::builder()
        .user_agent("SolarBuster")
        .pool_max_idle_per_host(100)
        .timeout(Duration::from_secs(5))
        .connect_timeout(Duration::from_secs(2))
        .build()
        .unwrap();

    while let Some(word) = rx.recv().await {
        let permit = sem.clone().acquire_owned().await.unwrap();
        let client = client.clone();
        let url = url.clone();
        tokio::spawn(async move {
            worker(client, url, word).await;
            drop(permit);
        });
    }
}

async fn reader(path: String, tx: mpsc::Sender<String>) {
    let file = File::open(path).await.unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    while let Some(line) = lines.next_line().await.unwrap() {
        let word = line.trim();
        if word.is_empty() {
            continue;
        }

        if tx.send(word.to_string()).await.is_err() {
            break;
        }
    }
}

async fn worker(client: Client, url: String, word: String) {
    let full_url = format!("{}{}", url, word);

    if let Ok(resp) = client.get(&full_url).send().await {
        let status = resp.status();
        if status.is_success() {
            println!("{} {}", full_url, status);
        }
    }
}
