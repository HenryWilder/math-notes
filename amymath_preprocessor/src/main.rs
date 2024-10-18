use std::{env, fs::{read_to_string, File}, io::Write};

pub mod lexer;
pub mod parser;
pub mod processor;

use processor::process_document;

const DEFAULT_TEMPLATE: &str = include_str!("template.sty");

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let root = env::current_dir().unwrap();

    println!("Root: {root:#?}\nArgs: {args:#?}");

    if args.len() > 4 {
        return Err(std::io::Error::other("Too many arguments, I don't know what to do with these"))
    }

    let source_path = if args.len() >= 2 {
        let mut path = root.clone();
        path.push(&args[1]);
        path.set_extension("math");
        path
    } else {
        return Err(std::io::Error::other("Missing argument for source document"));
    };
    println!("source path: {source_path:#?} exists? {}", File::open(&source_path).is_ok());

    let source = {
        read_to_string(&source_path)?
    };
    println!("source document: {source}");

    let output_path = if args.len() >= 3 {
        let mut path = root.clone();
        path.push(&args[2]);
        path.with_extension("tex");
        path
    } else {
        source_path
            .with_file_name("output")
            .with_extension("tex")
    };
    println!("output path: {output_path:#?}");

    let template: String = {
        if args.len() >= 4 {
            let mut path = root.clone();
            path.push(&args[3]);
            read_to_string(&path)?
        } else {
            DEFAULT_TEMPLATE.to_owned()
        }
    };
    println!("template document:\n{template}");

    let output = match process_document(&source, &template) {
        Ok(output) => output,
        Err(e) => return Err(std::io::Error::other(format!("While processing the document: {e}"))),
    };

    let mut output_file = File::create(output_path)?;
    output_file.write_all(output.as_bytes())?;

    Ok(())
}
