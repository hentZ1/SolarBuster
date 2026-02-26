use clap::Parser;
use colored::*;
use figlet_rs::FIGfont;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use std::{sync::Arc, time::Duration};
use tokio::{
    fs::{self, File},
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

    //total ate acabar as palavras da wordlist
    let ttl_finish = fs::read_to_string(&path)
        .await
        .expect("failed to ProgressBar read wordlist")
        .lines()
        .filter(|line| !line.trim().is_empty())
        .count() as u64;

    let pb = Arc::new(ProgressBar::new(ttl_finish));
    banner(url.clone(), path.clone());

    pb.set_style(
        ProgressStyle::default_bar()
            .template("{bar:40.red/orange} {pos}/{len} ({percent}%) ETA:{eta}")
            .unwrap(),
    );
    let (tx, mut rx) = mpsc::channel::<String>(QUEUE_SIZE);

    let sem = Arc::new(Semaphore::new(workers));

    //lê 1000 palavras a manda para o worker mandar requests
    tokio::spawn(reader(path, tx));

    //builda o client e define regras para a requests
    let client = Client::builder()
        .user_agent("SolarBuster")
        .pool_max_idle_per_host(100)
        .timeout(Duration::from_secs(5))
        .connect_timeout(Duration::from_secs(2))
        .build()
        .unwrap();

    //manda uma requests falsa para saber o tamanho do "barulho" feito por ela para depois o worker
    //saber oq é falso e oq é um status code real, assim evitando que os sites voltem sempre
    //codigos de status falsos
    let noise: u64 = measure_noise(url.clone(), client.clone())
        .await
        .unwrap_or(0);

    //spawna os workers ate o reader nao puder ler mais nada, ou seja ate a wordlist acabar
    while let Some(word) = rx.recv().await {
        let permit = sem.clone().acquire_owned().await.unwrap();
        let client = client.clone();
        let url = url.clone();
        let pb = pb.clone();
        tokio::spawn(async move {
            worker(client, url, word, noise, pb).await;
            drop(permit);
        });
    }
    pb.finish_with_message("Scan complete");
}

fn banner(url: String, wordlist: String) {
    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert("SolarBuster").unwrap();
    println!(
        "/{}/\n{}\n/{}/\n",
        "*".repeat(100),
        figure.to_string().bright_red(),
        "*".repeat(100)
    );
    println!("scanning: {}\n{}\n", url, "-".repeat(100));
    println!("wordlist used: {}\n{}\n", wordlist, "-".repeat(100));
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

async fn worker(client: Client, url: String, word: String, noise: u64, pb: Arc<ProgressBar>) {
    let full_url = format!("{}/{}", url.trim_end_matches('/'), word);

    let mut attempts = 0;
    let max_retries = 3;

    while attempts < max_retries {
        match client.head(&full_url).send().await {
            Ok(resp) => {
                if resp.status().is_success()
                    && let Some(len) = resp.content_length()
                    && len != noise
                {
                    println!("[{}] {}", resp.status().to_string().green(), full_url);
                }
                break;
            }
            Err(_) => {
                attempts += 1;
                if attempts >= max_retries {
                    break;
                }
            }
        }
    }
    pb.inc(1);
}
