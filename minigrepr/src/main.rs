use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    //dbg!(&args);

    let config = Config::new(&args);
    println!("Searching for \"{}\" in file: {}", config.query, config.file_path);

    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");
    println!("With text:\n{}", contents);
    
}

struct Config {
    query: String,
    file_path: String
}

impl Config {
    fn new(args: &Vec<String>) -> Config {
        if args.len() < 3 {
            panic!("Not enough arguments")
        }
        Config {
         query: args[1].clone(),
         file_path: args[2].clone()
        }
     }
}
