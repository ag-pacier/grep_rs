use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    //println!("{:?}", args); debug line

    let config = Config::new(env::args()).unwrap_or_else(|err| { eprintln!("Problem parsing arguments: {}", err); process::exit(1)});

    println!("Searching for {} in file: {}", config.query, config.filename); //debug line

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}

