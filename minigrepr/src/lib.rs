use std::fs;
use std::error::Error;

pub struct Config {
    pub query: String,
    pub file_path: String
}

impl Config {
    pub fn build(args: &Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 3 {
           return Err("not enough arguments");  
        }
        Ok(Config {
         query: args[1].clone(),
         file_path: args[2].clone()
        })
     }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.file_path)?;
    println!("With text:\n{}", contents);

    Ok(())
}