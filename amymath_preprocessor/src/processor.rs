use std::collections::{BTreeMap, HashMap};
use regex::Regex;
use crate::{lexer::*, parser::{parse, ParseError}};

#[derive(Debug, Clone, Copy)]
pub enum DefKind {
    Variable,
    Constant,
    Function,
}

#[derive(Debug, Clone)]
struct Heading<'doc> {
    /// Must be between 1 and `Heading::DEPTH_NAMES.len()`
    depth: usize,
    name: &'doc str,
}

impl Heading<'_> {
    pub const DEPTH_NAMES: [&'static str; 4] = [
        "chapter",
        "section",
        "subsection",
        "subsubsection",
    ];
}

#[derive(Debug, Clone)]
enum ContentItem<'doc> {
    Heading(Heading<'doc>),
    Math(Vec<String>),
}

const CONTENT_ANCHOR: &str = "@{content}";

#[derive(Debug)]
pub enum AmymathError<'doc> {
    LexerError{
        line_number: usize,
        error: LexerError<'doc>,
    },
    ParseError{
        line_number: usize,
        error: ParseError,
    },
    InvalidHeading{
        line_number: usize,
    },
    TemplateMissingContent,
}

impl<'doc> std::fmt::Display for AmymathError<'doc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AmymathError::LexerError { line_number, error }
                => write!(f, "At line {line_number}: Tokenization error: {error}"),
            AmymathError::ParseError { line_number, error }
                => write!(f, "At line {line_number}: Parse error: {error}"),
            AmymathError::InvalidHeading { line_number }
                => write!(f, "At line {line_number}: Headings must start with 1-4 '#'s followed by a space and then text."),
            AmymathError::TemplateMissingContent
                => write!(f, "Template is missing a `{CONTENT_ANCHOR}`, I don't know where the content should be inserted."),
        }
    }
}

pub fn process_document<'doc>(document: &'doc str, template: &str) -> Result<String, AmymathError<'doc>> {
    let rx_def = Regex::new(r"^(?<kind>fn|let|const)\s+(?<names>(?:[a-zA-Z]+)(?:,\s*[a-zA-Z]+)*)\b").unwrap();

    if !template.contains(CONTENT_ANCHOR) {
        return Err(AmymathError::TemplateMissingContent)
    }

    let lines = document
        .lines()
        .enumerate()
        // Remove comments
        .map(|(n, line)|
            (n+1, if let Some(comment_start) = line.find("%") {
                &line[..comment_start]
            } else {
                line
            }.trim())
        )
        // Remove blank lines
        .filter(|(_n, line)| !line.is_empty());

    let mut meta = BTreeMap::<&str, &str>::from([
        ("author", "Unknown"),
        ("title", "Unknown"),
    ]);
    let mut definitions = HashMap::<&'doc str, DefKind>::new();
    let mut content = Vec::<ContentItem>::new();

    let lexer = Lexer::new();
    for (line_number, line) in lines {
        // Meta item
        if line.starts_with("@") {
            match line[1..].split_once(" ") {
                // Key-value pair
                Some((key, value)) => meta.insert(key, value),
                // Existence vs nonexistence
                None => meta.insert(&line[1..], ""),
            };
        }
        // Heading item
        else if line.starts_with("#") {
            let heading = match line.split_once(" ") {
                Some((depth, name)) if depth.len() <= 4 && depth.chars().all(|c| c == '#') => {
                    assert!(!depth.is_empty(), "Heading should not be created with 0 '#' symbols");
                    assert!(!name.is_empty(), "Heading should not be created without text");
                    let depth = depth.len();
                    Heading{ depth, name }
                },
                _ => return Err(AmymathError::InvalidHeading { line_number }),
            };
            content.push(ContentItem::Heading(heading));
        }
        // Object definition
        else if let Some(caps) = rx_def.captures(line) {
            let kind_str = caps.name("kind").unwrap().as_str();
            let names_str = caps.name("names").unwrap().as_str();

            let kind = match kind_str {
                "let"   => DefKind::Variable,
                "const" => DefKind::Constant,
                "fn"    => DefKind::Function,
                _ => unreachable!("The only strings being captured by rx_def[kind] are `let`, `const`, and `fn`"),
            };

            // The set of items that are all being defined on the same line
            let names = names_str.split(",").map(str::trim);

            for name in names {
                definitions.insert(name, kind);
            }
        }
        // Math
        else {
            println!("line: {line}");

            let tokens = lexer.tokenize(line).map_err(|error| AmymathError::LexerError { line_number, error })?;
            println!("tokens: {tokens:#?}");

            let syntax_tree = parse(tokens).map_err(|error| AmymathError::ParseError { line_number, error })?;
            println!("syntax tree: {syntax_tree:#?}");

            let tex = syntax_tree.into_tex();
            println!("syntax tree TeX: {tex}");

            // Append or create
            match content.last_mut() {
                Some(ContentItem::Math(math)) => math.push(tex),
                _ => content.push(ContentItem::Math(vec![tex])),
            }
        }
    }

    // Convert content structure into text
    let content_str = content
        .into_iter()
        .map(|item|
            match item {
                ContentItem::Heading(Heading { depth, name }) => {
                    assert!(0 < depth && depth <= Heading::DEPTH_NAMES.len(), "Heading depth should have been checked before adding them to `content`");
                    let depth_name = Heading::DEPTH_NAMES[depth - 1];
                    format!("\\{depth_name}{{{name}}}")
                }
                ContentItem::Math(items) => {
                    format!("\\begin{{gather*}}\n{}\n\\end{{gather*}}", items.join("\\\\\n"))
                }
            }
        )
        .collect::<Vec<_>>()
        .join("");

    // Insert content into output
    let mut output = template.replace(CONTENT_ANCHOR, &content_str);

    // Insert meta variables into output
    for (key, value) in meta {
        let key_search = format!("@{{{key}}}");
        output = output.replace(&key_search, value);
    }

    Ok(output)
}
