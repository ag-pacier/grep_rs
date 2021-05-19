use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    //println!("{:?}", args); debug line

    let query = &args[1];
    let filename =&args[2];

    println!("Searching for {} in file: {}", query, filename); //debug line

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}
