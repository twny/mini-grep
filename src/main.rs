use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing args: {err}");
        std::process::exit(1);
    });

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


impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Args: query, filepath. Not enough args");
        }
        let query = &args[1].clone();
        let filepath = &args[2].clone();

        Ok(Config { query: query.to_string(), filepath: filepath.to_string() })
    }
}

