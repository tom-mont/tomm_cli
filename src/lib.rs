use chrono::Local;
use clap::Parser;
use std::env;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    // thing to remember
    #[arg(short, long)]
    pub remember: String,
}

pub fn run(args: &Args) -> Result<(), Box<dyn Error>> {
    // Before we push the argument, let's add a datetime
    let now = Local::now();
    let timestamp = now.format("(%Y-%m-%d %H:%M:%S)");

    let data: String = format!("\n{} - {}", timestamp, args.remember);

    // Determine file path
    let file_path = get_file_path()?;

    OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)?
        .write_all(data.as_bytes())?;

    Ok(())
}

fn get_file_path() -> Result<PathBuf, Box<dyn Error>> {
    let vault_path = match env::var("OBSIDIAN_VAULT_PATH") {
        Ok(path) => PathBuf::from(path),
        Err(_) => {
            // Fallback to default location in home directory
            let home_dir = env::var("HOME")?;
            Path::new(&home_dir)
                .join("Documents")
                .join("Obsidian Vault")
        }
    };

    Ok(vault_path.join("working-memory.md"))
}
