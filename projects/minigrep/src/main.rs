use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| { // use constructor
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    println!("Searching query: {}", config.query);
    println!("Searching in file: {}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        println!("Applicatiion error: {e}");
        process::exit(1);
    }
}