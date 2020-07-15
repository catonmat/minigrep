use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);

    println!("searching for: {} in: {}...", config.query, config.filename);

    let contents = fs::read_to_string(config.filename).expect("Something went wrong reading the file.");

    println!("Text is:\n{}", contents);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        // &args[0] = /path/to/rust/binary/main.rs
        let query = args[1].clone();
        let filename = args[2].clone();
    
        Config { query, filename }
    }
}
