use regex::Regex;

/// Operator tokens.
pub mod operator;
/// Delimited group tokens.
pub mod group_ctrl;
/// Variable/constant/function tokens.
pub mod word;
/// Generic Token type.
pub mod token;
/// Lexer error module.
pub mod error;

pub use operator::*;
pub use group_ctrl::*;
pub use word::*;
pub use token::*;
pub use error::LexerError;

/// The machine that breaks a document string into tokens.
pub struct Lexer {
    rx_word: Regex,
    rx_number: Regex,
    rx_tokenize: Regex,
}

impl Lexer {
    /// Constructs a new lexer, initializing the regex functions.
    ///
    /// ...Because regex can't be precompiled in Rust :/
    pub fn new() -> Self {
        const RX_WORD_STR: &str = r"\b[a-zA-Z]+\b";
        let rx_word = Regex::new(RX_WORD_STR).unwrap();

        const RX_NUMBER_STR: &str = r"[0-9]*\.?[0-9]+";
        let rx_number = Regex::new(RX_NUMBER_STR).unwrap();

        // Combine all regex patterns into one
        let operators = OperatorToken::regex_items();
        let group_ctrls = GroupCtrlToken::regex_items();
        let mut special = operators.iter()
            .chain(group_ctrls.iter())
            .map(|x| x.as_str())
            .collect::<Vec<_>>();
        special.sort_by(|a, b| b.len().cmp(&a.len()));
        let special = special;
        let rx_tokenize = Regex::new(
            special.into_iter()
                .chain([RX_WORD_STR].into_iter())
                .chain([RX_NUMBER_STR].into_iter())
                .collect::<Vec<_>>()
                .join("|")
                .as_str()
        ).unwrap();

        Self {
            rx_word,
            rx_number,
            rx_tokenize,
        }
    }

    /// Break a document string into tokens.
    pub fn tokenize<'doc>(&'_ self, line: &'doc str) -> Result<Vec<Token<'doc>>, LexerError> {
        let tokens = self.rx_tokenize
            .find_iter(line)
            .map(|token_match| {
                let token_str: &'doc str = token_match.as_str();
                if let Some(op_token) = OperatorToken::try_from(token_str) {
                    Ok(Token::Operator(op_token))
                } else if let Some(gc_token) = GroupCtrlToken::try_from(&token_str) {
                    Ok(Token::GroupCtrl(gc_token))
                } else if self.rx_number.is_match(&token_str) {
                    Ok(Token::Number(token_str))
                } else if self.rx_word.is_match(&token_str) {
                    Ok(Token::Word(WordToken::from(token_str)))
                } else {
                    Err(LexerError::UnknownToken { token: token_str.to_string() })
                }
            })
            .collect::<Result<_, _>>()?;

        Ok(tokens)
    }
}

