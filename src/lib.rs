use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        // &args[0] = /path/to/rust/binary/main.rs
        args.next(); // we skip the pathname and go to query value.alloc

        let query = match args.next() {
          Some(arg) => arg,
          None => return Err("Didn't receive a query string."),
        };
        let filename = match args.next() {
          Some(arg) => arg,
          None => return Err("Did not receive a filename."),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
    
        Ok(Config { 
            query,
            filename,
            case_sensitive
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // `Box<dyn Error>` means the function will return a type that implements the trait `Error`
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    let mut matches: u8 = 0;
    for line in results {
        println!("{}", line);
        matches += 1;
    }

    if matches == 0 {
        println!("No results found.");
    }

    // Using `unit- value ()` is stylistically stating `run()` is executed purely for side-effects and not for a return value.
    Ok(()) 
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // `'a` states the return value of the function will have identicla lifetime to `contents`
    contents
      .lines()
      .filter(|line| line.contains(query))
      .collect()
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
