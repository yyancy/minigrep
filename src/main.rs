use clap::Parser;
use minigrep::Config;
use std::process;
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
fn main() {
    test();
    let config = Config::parse();
    println!("Search for {} in {}", config.query, config.file_path);
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(2)
    }
}

fn test() {
 let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, y.0);
}
