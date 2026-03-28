use crate::prelude::*;
use figlet_rs::FIGfont;
use reqwest::StatusCode;

pub fn banner(url: &str, wordlist: &str) {
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

pub fn build_progress_bar() -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.red} {pos} requests")
            .unwrap(),
    );
    pb
}

pub fn print_result(status: StatusCode, url: &str) {
    println!("[{}] {}", status.as_str().green(), url);
}
