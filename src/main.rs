use std::env;
use std::process;
use tomm_cli::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    // First item in the vector is the name of the 
    // bin, hence we start at [1]:
    println!("Searching for: {}", config.query);
    println!("In file: {}", config.file_path);
    if let Err(e) = tomm_cli::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

