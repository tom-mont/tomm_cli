use std::env;
use std::process;
use tomm_cli::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        process::exit(1);
    });
    // First item in the vector is the name of the
    // bin, hence we start at [1]:
    if let Err(e) = tomm_cli::run(config) {
        process::exit(1);
    }
}
