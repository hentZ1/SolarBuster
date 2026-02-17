use clap::Parser;
use reqwest::Client;
use std::time::Duration;

use tokio::{
    fs::File,
    io::{AsyncBufRead, AsyncBufReadExt, BufReader},
    spawn,
    sync::{
        broadcast::Receiver,
        mpsc::{Sender, channel},
    },
};

#[derive(Parser, Debug)]
#[clap(version)]

struct Args {
    #[arg(short = 'u', long = "url")]
    url: String,

    #[arg(short = 'w', long = "wordlist")]
    path: String,
}

const QUEUE_SIZE: usize = 1000;

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let url = args.url;
    let path = args.path;

    //o channel precisa ser asincrono porem quando fica asincrono tudo piora, porfavor faça o
    //channel aceitar o numero de workers da constante e faça com que nao quebre tudo :(

    let (tx, rx) = tokio::sync::mpsc::channel(QUEUE_SIZE);
    for i in 0..50 {
        let rx_clone = rx.clone();

        spawn(worker(url, i, rx_clone));
    }

    reader(path, tx).await;
}

async fn reader(path: String, tx: Sender<String>) {
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

async fn worker(url: String, id: i32, mut rx: Receiver<String>) {
    let client = Client::builder()
        .user_agent("SolarBuster")
        .pool_max_idle_per_host(100)
        .timeout(Duration::from_secs(5))
        .connect_timeout(Duration::from_secs(2))
        .build()
        .unwrap();

    //aqui esta dando erro de tipos diferentes nao sei pq
    while let Some(word) = rx.recv().await {
        let full_url = format!("{}/{}", url, word);
        match client.get(&full_url).send().await {
            Ok(resp) => {
                let status = resp.status();

                if status.is_success() {
                    println!("[{}] {} {}", id, url, status);
                }
            }
            Err(_) => {}
        }
    }
}
