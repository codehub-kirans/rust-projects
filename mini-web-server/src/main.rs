use std::{env, process};

use mini_web_server::run;
use mini_web_server::Config;

fn main() {
    let args = env::args();
    //use unwrap_or_else idiom when a function returns a Result with Ok(v) on success or Err(e) on error
    let config = Config::build(args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    //use if let idiom if a function returns only Err(e) on error and () on success
    if let Err(e) = run(config) {
        eprintln!("Application Error: {e}");
        process::exit(1);
    }
}
