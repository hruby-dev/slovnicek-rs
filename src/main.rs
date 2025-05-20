use std::fs::File;
use std::io::{self, Read};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "slovnicek", version = "1.0", about = "Extracts readable ASCII strings from a file.")]
struct Args {
    #[arg(short = 'f', long, value_name = "FILE")]
    file_path: String,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let mut file = File::open(&args.file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let mut current = String::new();
    for &byte in &buffer {
        if (32..=126).contains(&byte) {
            current.push(byte as char);
        } else {
            if current.len() >= 4 {
                println!("{}", current);
            }
            current.clear();
        }
    }
    if current.len() >= 4 {
        println!("{}", current);
    }

    Ok(())
}
