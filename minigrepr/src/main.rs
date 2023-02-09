use std::env;
use std::fs;
use std::process;
use std::error::Error;

fn main() {
    let args: Vec<String> = env::args().collect();
    //dbg!(&args);

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    println!("Searching for \"{}\" \nIn file {}", config.query, config.file_path);

    if let Err(e) = run(config) {
        println!("Application Error: {}", e);
        process::exit(1);
    }    
}

fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.file_path)?;
    println!("With text:\n{}", contents);

    Ok(())
}

struct Config {
    query: String,
    file_path: String
}

impl Config {
    fn new(args: &Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 3 {
           return Err("not enough arguments");  
        }
        Ok(Config {
         query: args[1].clone(),
         file_path: args[2].clone()
        })
     }
}
