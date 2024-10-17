use std::collections::{BTreeMap, HashMap};
use regex::Regex;
use crate::{lexer::*, parser::{parse, syntax_tree::*}};

#[derive(Debug, Clone, Copy)]
pub enum DefKind {
    Variable,
    Constant,
    Function,
}

#[derive(Debug, Clone)]
struct Heading<'doc> {
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
    Math(Vec<&'doc str>),
}

pub fn process_document<'doc>(document: &'doc str, template: &str) -> String {
    let rx_def = Regex::new(r"^(?<kind>fn|let|const)\s+(?<names>(?:[a-zA-Z]+)(?:,\s*[a-zA-Z]+)*)\b").unwrap();

    let lines = document
        .lines()
        .map(|line| match line.split_once('%') {
            Some((non_comment, _comment)) => non_comment,
            None => line,
        })
        .map(|line| line.trim())
        .filter(|line| !line.is_empty());

    let mut meta = BTreeMap::<&str, &str>::from([
        ("author", "Unknown"),
        ("title", "Unknown"),
    ]);
    let mut definitions = HashMap::<&'doc str, DefKind>::new();
    let mut content = Vec::<ContentItem>::new();

    let lexer = Lexer::new();
    for line in lines {
        if line.starts_with("@") {
            match line[1..].split_once(" ") {
                Some((key, value)) => meta.insert(key, value),
                None => meta.insert(&line[1..], ""),
            };
        } else if line.starts_with("#") {
            let heading = match line.split_once(" ") {
                Some((depth, name)) => Heading{ depth: depth.len(), name },
                None => panic!("todo"),
            };
            content.push(ContentItem::Heading(heading));
        } else if let Some(caps) = rx_def.captures(line) {
            let kind = match caps.name("kind").unwrap().as_str() {
                "let"   => DefKind::Variable,
                "const" => DefKind::Constant,
                "fn"    => DefKind::Function,
                _ => unreachable!(),
            };

            let names = (caps.name("names").unwrap().as_str())
                .split(",")
                .map(|name| name.trim());

            for name in names {
                definitions.insert(name, kind);
            }
        } else {
            println!("line: {line}");

            let tokens = lexer.tokenize(line);
            println!("tokens: {tokens:#?}");

            let syntax_tree = parse(tokens);
            println!("syntax tree: {syntax_tree:#?}");
        }
    }

    let mut content_str = String::new();
    for item in content {
        let text = match item {
            ContentItem::Heading(Heading { depth, name }) => {
                let depth_name = Heading::DEPTH_NAMES[depth - 1];
                format!("\\{depth_name}{{{name}}}")
            }
            ContentItem::Math(items) => {
                format!("\\begin{{gather*}}\n{}\n\\end{{gather*}}", items.join("\\\\\n"))
            }
        };
        content_str.push_str(&text);
    }

    let mut output = template.replace("@{content}", &content_str);
    for (key, value) in meta {
        let key_search = format!("@{{{key}}}");
        output = output.replace(&key_search, value);
    }
    output
}
