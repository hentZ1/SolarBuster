pub use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "SolarBuster",
    version,
    about = "Fast web directory enumerator in Rust"
)]
pub struct Args {
    #[arg(
        short = 'u',
        long = "url",
        help = "the url of the website you want to enumerate"
    )]
    pub url: String,

    #[arg(
        short = 'w',
        long = "wordlist",
        help = "your word list for the enumerator"
    )]
    pub path: String,

    #[arg(
        short = 'c',
        default_value_t = 50,
        long = "concurrency",
        help = "number of competing workers"
    )]
    pub workers: usize,
}
