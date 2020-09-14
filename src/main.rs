use std::env;
use std::process;
use minigrep::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err_msg| {
        // env::args() returns an interator
        eprintln!("Error: {}", err_msg);
        process::exit(1);
    });

    println!("searching for `{}` in `{}`...\n", config.query, config.filename);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
