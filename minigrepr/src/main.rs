use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    //dbg!(&args);

    let (query, file_path) = parse_config(&args);
    println!("Searching for \"{}\" in file: {}", query, file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    println!("With text:\n{}", contents);
    
}

fn parse_config(args: &Vec<String>) -> (&String, &String) {
    let query = &args[1];
    let file_path = &args[2];
    
    (query, file_path)
}
