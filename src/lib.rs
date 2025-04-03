use chrono::Local;
use clap::Parser;
use std::env;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    // thing to remember
    #[arg(short, long)]
    pub remember: String,
}

pub fn run(args: &Args) -> Result<(), Box<dyn Error>> {
    let mut data_string: String = "\n".to_string();
    // Before we push the argument, let's add a datetime
    let now = Local::now();
    let timestamp = now.format("(%Y-%m-%d %H:%M:%S) - ").to_string();

    data_string.push_str(&timestamp);
    data_string.push_str(&args.remember);
    let data = data_string.as_bytes();
    let mut pos = 0;

    // Creates a file:
    // let mut buffer = File::create("foo.txt")?;

    let vault_path = match env::var("OBSIDIAN_VAULT_PATH") {
        Ok(path) => PathBuf::from(path),
        Err(_) => {
            // Fallback to default location in home directory
            let home_dir = env::var("HOME").expect("Failed to get home directory");
            PathBuf::from(home_dir)
                .join("Documents")
                .join("Obsidian Vault")
        }
    };

    let file_path = vault_path.join("working-memory.md");

    let mut buffer = OpenOptions::new()
        .read(true)
        .append(true) // write will append to a file
        .create(true)
        .open(file_path)
        .expect("Failed to open or create file");

    while pos < data.len() {
        let bytes_written = buffer.write(&data[pos..])?;
        pos += bytes_written;
    }
    Ok(())
}
