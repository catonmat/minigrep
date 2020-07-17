use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
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

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // `Box<dyn Error>` means the function will return a type that implements the trait `Error`
    let contents = fs::read_to_string(config.filename)?;

    let mut results: u8 = 0;

    for line in search(&config.query, &contents) {
        println!("{}", line);
        results += 1;
    }

    if results == 0 {
        println!("No results found.");
    }

    // Using `unit- value ()` is stylistically stating `run()` is executed purely for side-effects and not for a return value.
    Ok(()) 
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // `'a` states the return value of the function will have identicla lifetime to `contents`
    let mut results = Vec::new();
    
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line)
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

            assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUST";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query,contents)
        );
    }
}