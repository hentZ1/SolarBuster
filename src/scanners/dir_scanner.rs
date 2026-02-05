use std::io;
use std::env::args;
use reqwest::Client;
use tokio::sync;

struct params_list{
    url: String,
    wordlist: String,
}

#[tokio::main]
fn main(){
    //arg[0] = dir_scanner.rs arg[1] = -u arg[2] = htpps://specified_url arg[3] = -w arg[4] =
    //PATH_TO_WORDLIST
    let args: Vec<String> = env::args()
        .collect()
        .expect("Lack of args");
    
    let mut Params = params_list { url: String::new(), wordlist: String::new };

    Params.url = "-u".to_string(); 
    Params.wordlist = "-w".to_string();
    
    let necessary_params = [Params.url, Params.wordlist];
    
    let arg_checker = necessary_params.iter().all(|param| args.contains(&param.to_string()));

    if !arg_checker {

        Err(String::from("Try: dir_scanner -u <your_url_here> -w <your_wordlist_here>"))
    }

    let url = args[2];
    let wordlist_name = args[3];
    let wordlist_path = args[4];
    
    dir_scanner(url, wordlist_name, wordlist_path);
}

async fn dir_scanner(base_url: String, wordlist: String, path: String) {

    let client = Client::builder()
        .user_agent("SolarBuster")
        .pool_max_per_idle(100)
        .timeout(Duration::from_millis(300))
        .connect_timeout(Duration::from_millis(80))
        .build()?
        .unwrap();


}
