use std::fs;
use std::result::Result::Err;
use std::error::Error;

pub struct Config {
    query: String,
    filepath: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Args: query, filepath. Not enough args");
        }
        let query = &args[1].clone();
        let filepath = &args[2].clone();

        Ok(Config { query: query.to_string(), filepath: filepath.to_string() })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filepath).expect("Wanted to read a file");

    println!("first arg: {}", &config.query);
    println!("second arg: {}", &config.filepath);
    println!("poem\n{contents}");
    Ok(())
}

