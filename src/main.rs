use std::env;
use std::fs;
use std::process;
use std::error::Error;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err_msg| {
        println!("Error: {}", err_msg);
        process::exit(1);
    });

    println!("searching for: {} in: {}...", config.query, config.filename);

    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
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

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // `Box<dyn Error>` means the function will return a type that implements the trait `Error`
    let contents = fs::read_to_string(config.filename)?;

    println!("Text is:\n{}", contents);

    // Using `unit- value ()` is stylistically stating `run()` is executed purely for side-effects and not for a return value.
    Ok(()) 
}