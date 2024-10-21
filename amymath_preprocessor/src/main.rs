//! A preprocessor for converting AmyMath into LaTeX.

#![warn(missing_docs)]

use std::{fs::File, io::Write};

/// Extracts information from command-line arguments.
pub mod program_args;
/// A stack collection.
pub mod stack;
/// TeX conversion trait.
pub mod to_tex;
/// Tokenization module. Handles breakup.
pub mod lexer;
/// Parsing module. Handles lookaround and clumping.
pub mod parser;
/// Main preprocessor module. Applies lexer and parser, then converts to TeX.
pub mod processor;

use processor::process_document;
use program_args::*;

fn main() -> std::io::Result<()> {
    match ProgramArgs::try_from(std::env::current_dir().unwrap(), std::env::args()) {
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
