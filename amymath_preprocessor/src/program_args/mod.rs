use std::{fs::{read_to_string, File}, path::PathBuf};

/// `program_args` error module.
pub mod error;
use error::ArgsError;

const DEFAULT_TEMPLATE: &str = include_str!("../template.sty");

/// Program arguments.
pub struct ProgramArgs {
    /// Source document as a string.
    pub source: String,
    /// Path to the file the output should be stored to.
    pub out_path: PathBuf,
    /// The template document as a string.
    pub template: String,
}

impl ProgramArgs {
    /// Extract information from command line.
    pub fn try_from(root: PathBuf, mut args: std::env::Args) -> Result<Self, ArgsError> {
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
}
