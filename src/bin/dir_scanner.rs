use clap::Parser;
use colored::*;
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
    #[arg(
        short = 'u',
        long = "url",
        help = "the url of the website you want to enumerate"
    )]
    url: String,

    #[arg(
        short = 'w',
        long = "wordlist",
        help = "your word list for the enumerator"
    )]
    path: String,

    #[arg(
        short = 'c',
        default_value_t = 50,
        long = "concurrency",
        help = "number of competing workers"
    )]
    workers: usize,
}

const QUEUE_SIZE: usize = 1000;

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let url = args.url;
    let path = args.path;
    let workers = args.workers;

    let (tx, mut rx) = mpsc::channel::<String>(QUEUE_SIZE);

    let sem = Arc::new(Semaphore::new(workers));

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

async fn measure_noise(url: String, client: Client) -> Option<u64> {
    let fake_url = format!("{}{}", url, "THIS_IS_NOT_A_VALID_URL_ON_PURPOSE");

    let resp = client.head(&fake_url).send().await.ok()?;

    resp.content_length()
}

async fn reader(path: String, tx: mpsc::Sender<String>) {
    let file = File::open(path)
        .await
        .expect("it was not possible to read or find the file ");

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

    let noise = measure_noise(url, client.clone()).await.unwrap_or(0);

    if let Ok(resp) = client.head(&full_url).send().await
        && resp.status().is_success()
        && let Some(len) = resp.content_length()
        && len != noise
    {
        println!("[{}]{}", resp.status().to_string().green(), full_url);
    }
}
