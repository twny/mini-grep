use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_args(&args);

    let contents = fs::read_to_string(&config.filepath).expect("Wanted to read a file");

    println!("first arg: {}", &config.query);
    println!("second arg: {}", &config.filepath);
    println!("poem\n{contents}");
    // println!("{:?}", args);
}

struct Config {
    query: String,
    filepath: String,
}

fn parse_args(args: &[String]) -> Config {
    let query = &args[1].clone();
    let filepath = &args[2].clone();

    Config { query: query.to_string(), filepath: filepath.to_string() }
}
