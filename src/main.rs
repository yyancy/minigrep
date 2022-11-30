use clap::Parser;
use minigrep::Config;
use std::process;

fn main() {
    let config = Config::parse();
    println!("Search for {} in {}", config.query, config.file_path);
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(2)
    }
}
