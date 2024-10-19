use std::{fs::{read_to_string, File}, path::PathBuf};

pub mod error;
use error::ArgsError;

const DEFAULT_TEMPLATE: &str = include_str!("../template.sty");

pub struct ProgramArgs {
    pub source: String,
    pub out_path: PathBuf,
    pub template: String,
}

pub fn get_args(root: PathBuf, mut args: std::env::Args) -> Result<ProgramArgs, ArgsError> {
    let _exe_path = args.next();

    // Mandatory

    let src_path = root.join(args.next().ok_or(ArgsError::MissingSource)?);
    println!("source path: {src_path:#?} exists? {}", File::open(&src_path).is_ok());

    // Optional

    let mut result = ProgramArgs {
        source: read_to_string(&src_path)?,
        out_path: src_path.with_file_name("output").with_extension("tex"),
        template: DEFAULT_TEMPLATE.to_string(),
    };

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-o" | "--output" => {
                result.out_path = root.join(args.next().ok_or(ArgsError::MissingKVPValue { key: "output" })?);
            },
            "-t" | "--template" => {
                let template_path = root.join(args.next().ok_or(ArgsError::MissingKVPValue { key: "template" })?);
                result.template = read_to_string(&template_path)?;
            },
            _ => return Err(ArgsError::UnknownArg(arg)),
        }
    }

    println!("output path: {:#?}", result.out_path);
    // println!("template document:\n{}", result.template);

    Ok(result)
}
