use std::env;
use std::process;

use minigrepr::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    //dbg!(&args);

    //use unwrap_or_else idiom when a function returns a Result with Ok(v) on success or Err(e) on error
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for \"{}\" \nIn file {}", config.query, config.file_path);

    //use if let idiom if a function returns only Err(e) on error and () on success
    if let Err(e) = minigrepr::run(config) {
        eprintln!("Application Error: {}", e);
        process::exit(1);
    }
}