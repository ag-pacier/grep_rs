use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    //println!("{:?}", args); debug line

    let config = Config::new(&args);

    println!("Searching for {} in file: {}", config.query, config.filename); //debug line

    let contents = fs::read_to_string(config.filename).expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let filename = args[2].clone();

        Config {query, filename}
    }
}