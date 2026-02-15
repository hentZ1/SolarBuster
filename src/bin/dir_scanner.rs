use clap::Parser;
use reqwest::Client;
use std::{fs, time::Duration};
use tokio::sync;

#[derive(Parser, Debug)]
#[clap(version)]

struct Args {
    #[arg(short = 'u', long = "url")]
    url: String,

    #[arg(short = 'w', long = "wordlist")]
    wordlist: String,
}
#[tokio::main]
async fn main() {
    let args = Args::parse();
    let url = args.url;
    let wordlist = args.wordlist;

    let _ = request_sender(url, wordlist).await;
}
async fn request_sender(url: String, wordlist: String) -> Result<(), reqwest::Error> {
    let wl_content = fs::read_to_string(wordlist).expect("The program cannot read the file");

    let client = Client::builder()
        .user_agent("SolarBuster")
        .pool_max_idle_per_host(100)
        .timeout(Duration::from_secs(5))
        .connect_timeout(Duration::from_secs(2))
        .build()
        .unwrap();

    for w in wl_content.lines() {
        let full_url = format!("{}/{}", url.trim_end_matches('/'), w.trim());

        let res = client.get(&full_url).send().await?;

        println!("status: {}\nurl: {}", res.status(), full_url);
    }

    Ok(())
}
