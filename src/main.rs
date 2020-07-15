use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let (query, filename) = parse_config(&args);

    println!("searching for: {} in: {}...", query, filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file.");

    println!("Text is:\n{}", contents);
}

fn parse_config(args: &[String]) -> (&str, &str){
    // &args[0] = /path/to/rust/binary/main.rs
    let query = &args[1];
    let filename = &args[2];

    (query, filename)
}
