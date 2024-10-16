use regex::Regex;

#[derive(Debug, Clone, Copy)]
pub enum OperatorToken {
    Iff,
    Implies,
    Impliedby,
    Equiv,
    NEquiv,
    Pm,
    Mp,
    To,
    Gets,
    Eq,
    Ne,
    Gt,
    Ge,
    Lt,
    Le,
    Wedge,
    Vee,
    Factorial,
    Plus,
    Minus,
    CDot,
    Frac,
    Superscript,
    Subscript,
    Sim,
    Prime,
    Comma,
}

impl OperatorToken {
    pub fn regex() -> Regex {
        let mut tokens = [
            "<=>",
            "=>",
            "==>",
            "<==",
            "===",
            "!==",
            "+/-",
            "-/+",
            "->",
            "<-",
            "==",
            "=",
            "!=",
            "=/=",
            "<=",
            ">=",
            r"/\",
            r"\/",
            "<",
            ">",
            "!",
            "+",
            "-",
            "*",
            "/",
            "^",
            "_",
            "~",
            "'",
            ",",
        ];
        tokens.sort_by(|a, b| b.len().cmp(&a.len()));
        Regex::new(tokens.map(regex::escape).join("|").as_str()).unwrap()
    }

    pub fn try_from(token: &str) -> Option<Self> {
        match token {
            "<=>" | "=>" => Some(Self::Iff),
            "==>"        => Some(Self::Implies),
            "<=="        => Some(Self::Impliedby),
            "==="        => Some(Self::Equiv),
            "!=="        => Some(Self::NEquiv),
            "+/-"        => Some(Self::Pm),
            "-/+"        => Some(Self::Mp),
            "->"         => Some(Self::To),
            "<-"         => Some(Self::Gets),
            "==" | "="   => Some(Self::Eq),
            "!=" | "=/=" => Some(Self::Ne),
            "<="         => Some(Self::Le),
            ">="         => Some(Self::Ge),
            r"/\"        => Some(Self::Wedge),
            r"\/"        => Some(Self::Vee),
            "<"          => Some(Self::Lt),
            ">"          => Some(Self::Gt),
            "!"          => Some(Self::Factorial),
            "+"          => Some(Self::Plus),
            "-"          => Some(Self::Minus),
            "*"          => Some(Self::CDot),
            "/"          => Some(Self::Frac),
            "^"          => Some(Self::Superscript),
            "_"          => Some(Self::Subscript),
            "~"          => Some(Self::Sim),
            "'"          => Some(Self::Prime),
            ","          => Some(Self::Comma),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum BracketKind {
    Paren,
    Brack,
    Brace,
}

#[derive(Debug, Clone, Copy)]
pub enum GroupControl {
    Open,
    Close,
}

#[derive(Debug, Clone, Copy)]
pub struct GroupCtrlToken {
    pub kind: BracketKind,
    pub ctrl: GroupControl,
}

impl GroupCtrlToken {
    pub fn regex() -> Regex {
        let mut tokens = [
            "(",
            ")",
            "[",
            "]",
            "{",
            "}",
        ];
        tokens.sort_by(|a, b| b.len().cmp(&a.len()));
        Regex::new(tokens.map(regex::escape).join("|").as_str()).unwrap()
    }

    pub fn try_from(token: &str) -> Option<Self> {
        match token {
            "(" => Some(Self { kind: BracketKind::Paren, ctrl: GroupControl::Open  }),
            ")" => Some(Self { kind: BracketKind::Paren, ctrl: GroupControl::Close }),
            "[" => Some(Self { kind: BracketKind::Brack, ctrl: GroupControl::Open  }),
            "]" => Some(Self { kind: BracketKind::Brack, ctrl: GroupControl::Close }),
            "{" => Some(Self { kind: BracketKind::Brace, ctrl: GroupControl::Open  }),
            "}" => Some(Self { kind: BracketKind::Brace, ctrl: GroupControl::Close }),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Token<'doc> {
    Word(&'doc str),
    Number(&'doc str),
    Operator(OperatorToken),
    GroupCtrl(GroupCtrlToken),
    Other(&'doc str),
}

pub struct Lexer {
    rx_word: Regex,
    rx_number: Regex,
    rx_tokenize: Regex,
}

impl Lexer {
    pub fn new() -> Self {
        let rx_operators = OperatorToken::regex();
        let rx_group_ctrl = GroupCtrlToken::regex();
        let rx_word = Regex::new(r"\b[a-zA-Z]+\b").unwrap();
        let rx_number = Regex::new(r"[0-9]*\.?[0-9]+").unwrap();
        let rx_tokenize =
            Regex::new([
                rx_word.as_str(),
                rx_number.as_str(),
                rx_group_ctrl.as_str(),
                rx_operators.as_str()
            ].join("|").as_str())
            .unwrap();

        Self {
            rx_word,
            rx_number,
            rx_tokenize,
        }
    }

    pub fn tokenize<'doc>(&'_ self, line: &'doc str) -> Vec<Token<'doc>> {
        self.rx_tokenize
            .find_iter(line)
            .map(|m| m.as_str())
            .map(|token_str: &'doc str| {
                if let Some(op_token) = OperatorToken::try_from(token_str) {
                    Token::Operator(op_token)
                } else if let Some(gc_token) = GroupCtrlToken::try_from(&token_str) {
                    Token::GroupCtrl(gc_token)
                } else if self.rx_number.is_match(&token_str) {
                    Token::Number(token_str)
                } else if self.rx_word.is_match(&token_str) {
                    Token::Word(token_str)
                } else {
                    Token::Other(token_str)
                }
            })
            .collect()
    }
}

