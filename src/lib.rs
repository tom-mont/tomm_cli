use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    pub name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    pub count: u8,
}

pub fn run(args: &Args) -> Result<(), Box<dyn Error>> {
    for _ in 0..args.count {
        println!("Hello {}!", args.name);
    }
    let data = args.name.as_bytes();
    let mut pos = 0;
    let mut buffer = File::create("foo.txt")?;

    while pos < data.len() {
        let bytes_written = buffer.write(&data[pos..])?;
        pos += bytes_written;
    }
    Ok(())
}
