use clap::Parser;
use std::process;
use tomm_cli::Args;

fn main() {
    let args: Args = Args::parse();
    if let Err(e) = tomm_cli::run(&args) {
        eprintln!("CLI Error: {e}");
        process::exit(1);
    };
}
