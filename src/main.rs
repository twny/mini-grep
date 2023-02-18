use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // string
    let query = &args[1];
    // filepath
    let filepath = &args[2];

    // case insitive?
    let third_arg = &args[3];

    println!("first arg: {}", query);
    println!("second arg: {}", filepath);
    println!("third arg: {}", third_arg);
    // println!("{:?}", args);
}
