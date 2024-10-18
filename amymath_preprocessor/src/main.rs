use std::{fs::File, io::Write};

pub mod program_args;
pub mod stack;
pub mod lexer;
pub mod parser;
pub mod processor;


use processor::process_document;
use program_args::*;

fn process_file() -> Result<(), std::io::Error> {
    let args = get_args(std::env::current_dir().unwrap(), std::env::args())
        .map_err(|e| std::io::Error::other(e))?;

    let output = process_document(&args.source, &args.template)
        .map_err(|e| std::io::Error::other(e))?;

    let mut output_file = File::create(&args.out_path)?;
    output_file.write_all(output.as_bytes())?;

    Ok(())
}

fn main() {
    if let Err(error) = process_file() {
        eprintln!("{error}")
    }
}
