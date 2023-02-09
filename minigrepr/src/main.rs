use std::env;
use std::process;

use minigrepr::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    //dbg!(&args);

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    println!("Searching for \"{}\" \nIn file {}", config.query, config.file_path);

    if let Err(e) = minigrepr::run(config) {
        println!("Application Error: {}", e);
        process::exit(1);
    }    
}




