use std::{env, time::Duration};
use reqwest::Client;
use tokio::sync;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{

    let mut args = env::args().skip(1);

    if args.len() < 2 { return Err("Try: SolarBuster -u <your_url> -w <path_to_wordlist>".into()); } 
    
    let mut url: Option<String> = None; 
    let mut wordlist: Option<String> = None;

    while let Some(arg) = args.next() {

        match arg.as_str() {

            "-u" => url = args.next(),

            "-w" => wordlist = args.next(),

            _ => {}
        }
    }
 
    dir_scanner(url.expect("Wrong type"), wordlist.expect("Wrong type")).await;

    Ok(())
}

async fn dir_scanner(base_url: String, wordlist: String) {

    let client = Client::builder()
        .user_agent("SolarBuster")
        .pool_max_idle_per_host(100)
        .timeout(Duration::from_millis(300))
        .connect_timeout(Duration::from_millis(80))
        .build()
        .unwrap();

}
