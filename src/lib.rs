use std::fs;
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
    let contents = fs::read_to_string(&config.filepath)?;
    let results = search(&config.query, &contents);
    println!("{:?}", results);

    // println!("first arg: {}", &config.query);
    // println!("second arg: {}", &config.filepath);
    // println!("poem\n{contents}");
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut matches = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            matches.push(line)
        }
        // println!("line is: {line}")
    };
    return matches;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
            let query = "duct";
            let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
