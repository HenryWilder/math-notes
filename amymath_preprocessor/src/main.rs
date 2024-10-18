use std::{fs::File, io::Write};

pub mod program_args;
pub mod stack;
pub mod to_tex;
pub mod lexer;
pub mod parser;
pub mod processor;

use processor::process_document;
use program_args::*;

fn main() -> std::io::Result<()> {
    match get_args(std::env::current_dir().unwrap(), std::env::args()) {
        Err(error) => {
            eprintln!("Argument Error: {error}");
            Err(std::io::Error::other(error))
        },
        Ok(args) => {
            match process_document(&args.source, &args.template) {
                Err(error) => {
                    eprintln!("Preprocessor Error: {error}");
                    Err(std::io::Error::other(error))
                },
                Ok(output) => {
                    File::create(&args.out_path)?.write_all(output.as_bytes())
                },
            }
        },
    }
}
