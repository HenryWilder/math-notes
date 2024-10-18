use std::collections::{BTreeMap, HashMap};
use regex::Regex;
use crate::{to_tex::ToTex, lexer::*, parser::parse};

pub mod error;
use error::*;

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

pub fn process_document<'doc>(document: &'doc str, template: &str) -> Result<String, PreprocError> {
    let rx_def = Regex::new(r"^(?<kind>fn|let|const)\s+(?<names>(?:[a-zA-Z]+)(?:,\s*[a-zA-Z]+)*)\b").unwrap();

    if !template.contains(CONTENT_ANCHOR) {
        return Err(PreprocError::TemplateMissingContent)
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
        ("title", "Unnamed"),
    ]);
    let mut definitions = HashMap::<&'doc str, DefKind>::new();
    let mut content = Vec::<ContentItem>::new();

    let lexer = Lexer::new();
    for (line_number, line) in lines {
        // Meta item
        if line.starts_with("@") {
            match line[1..].split_once(" ") {
                Some((key, value)) => {
                    println!("Meta item: \"{key}\"=\"{value}\"");
                    meta.insert(key, value);
                },
                None => return Err(PreprocError::line_error(line_number, LineErrorKind::InvalidMetaItem)),
            };
        }
        // Heading item
        else if line.starts_with("#") {
            let heading = match line.split_once(" ") {
                Some((depth, name)) if depth.len() <= 4 && depth.chars().all(|c| c == '#') => {
                    assert!(!depth.is_empty(), "Heading should not be created with 0 '#' symbols");
                    assert!(!name.is_empty(), "Heading should not be created without text");
                    let depth = depth.len();
                    println!("Heading: \"{name}\" Depth: {depth}");
                    Heading{ depth, name }
                },
                _ => return Err(PreprocError::line_error(line_number, LineErrorKind::InvalidHeading)),
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
                println!("Defining \"{name}\" as {kind:?}");
                definitions.insert(name, kind);
            }
        }
        // Math
        else {
            println!("line: {line}");

            let tokens = lexer.tokenize(line)
                .map_err(|error| PreprocError::lexer_error(line_number, error))?;
            println!("tokens: {tokens:#?}");

            let syntax_tree = parse(tokens)
                .map_err(|error| PreprocError::parse_error(line_number, error))?;
            // println!("syntax tree: {syntax_tree:#?}");

            let tex = syntax_tree.to_tex();
            // println!("syntax tree TeX: {tex}");

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
        println!("Assigning `{key_search}` anchors with \"{value}\"");
        output = output.replace(&key_search, value);
    }

    Ok(output)
}
