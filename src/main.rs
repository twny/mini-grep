use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    // string
    let query = &args[1];
    // filepath
    let filepath = &args[2];

    let contents = fs::read_to_string(filepath).expect("Wanted to read a file");

    println!("first arg: {query}");
    println!("second arg: {filepath}");
    println!("poem\n{contents}");
    // println!("{:?}", args);
}
