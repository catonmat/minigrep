use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err_msg| {
        println!("Error: {}", err_msg);
        process::exit(1);
    });

    println!("searching for: {} in: {}...", config.query, config.filename);

    let contents = fs::read_to_string(config.filename).expect("Something went wrong reading the file.");

    println!("Text is:\n{}", contents);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        // &args[0] = /path/to/rust/binary/main.rs
        if args.len() < 3 {
            // panic!("Please enter the search term as the first argument and /path/to/filename as second argument.");
            return Err("Please enter the search term as the first argument and /path/to/filename as second argument.");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
    
        Ok(Config { query, filename })
    }
}
