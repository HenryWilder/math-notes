use std::{fs::File, io::{Read, Write}, path::Path};

pub mod lexer;
pub mod parser;
pub mod processor;

use processor::process_document;

pub fn process_file(src: &Path, dest: &Path, template: &Path) -> Result<(), std::io::Error> {
    let mut template_text = String::new();
    {
        let mut template_file = File::open(template)?;
        template_file.read_to_string(&mut template_text)?;
    }
    
    let mut src_text = String::new();
    {
        let mut src_file = File::open(src)?;
        src_file.read_to_string(&mut src_text)?;
    }

    {
        let mut dest_file = File::create(dest)?;
        let output = process_document(&src_text, &template_text);
        dest_file.write_all(output.as_bytes())?;
    }

    Ok(())
}

fn main() {
    match process_file(Path::new("../data.math"), Path::new("../tex/output.tex"), Path::new("../tex/template.tex")) {
        Ok(()) => (),
        Err(e) => eprintln!("{e:?}"),
    }
}
