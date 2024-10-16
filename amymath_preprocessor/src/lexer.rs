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
    Colon,

    Cup,
    Cap,
    In,

    And,
    Nand,
    Or,
    Nor,
    Xor,
    Xnor,
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
            ":",
            "|",
            "&",
            "in",
            "and",
            "nand",
            "or",
            "nor",
            "xor",
            "xnor",
        ];
        tokens.sort_by(|a, b| b.len().cmp(&a.len()));
        Regex::new(tokens.map(regex::escape).join("|").as_str()).unwrap()
    }

    pub fn into_tex(self) -> &'static str {
        match self {
            Self::Iff         => r"\iff",
            Self::Implies     => r"\implies",
            Self::Impliedby   => r"\impliedby",
            Self::Equiv       => r"\equiv",
            Self::NEquiv      => r"\nequiv",
            Self::Pm          => r"\pm",
            Self::Mp          => r"\mp",
            Self::To          => r"\to",
            Self::Gets        => r"\gets",
            Self::Eq          => r"=",
            Self::Ne          => r"\ne",
            Self::Gt          => r">",
            Self::Ge          => r"\ge",
            Self::Lt          => r"<",
            Self::Le          => r"\le",
            Self::Wedge       => r"\bigwedge",
            Self::Vee         => r"\bigvee",
            Self::Factorial   => r"!",
            Self::Plus        => r"+",
            Self::Minus       => r"-",
            Self::CDot        => r"\cdot",
            Self::Frac        => r"\frac",
            Self::Superscript => r"^",
            Self::Subscript   => r"_",
            Self::Sim         => r"\sim",
            Self::Prime       => r"\prime",
            Self::Comma       => r",",
            Self::Colon       => r":",
            Self::Cup        => r"\cup",
            Self::Cap        => r"\cap",
            Self::In         => r"\in",
            Self::And        => r"\land",
            Self::Nand       => r"\lnand",
            Self::Or         => r"\lor",
            Self::Nor        => r"\lnor",
            Self::Xor        => r"\lxor",
            Self::Xnor       => r"\lxnor",
        }
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
            ":"          => Some(Self::Colon),
            "|"          => Some(Self::Cup),
            "&"          => Some(Self::Cap),
            "in"         => Some(Self::In),
            "and"        => Some(Self::And),
            "nand"       => Some(Self::Nand),
            "or"         => Some(Self::Or),
            "nor"        => Some(Self::Nor),
            "xor"        => Some(Self::Xor),
            "xnor"       => Some(Self::Xnor),
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
        let tokens = [
            "(",
            ")",
            "[",
            "]",
            "{",
            "}",
        ];
        Regex::new(tokens.map(regex::escape).join("|").as_str()).unwrap()
    }

    pub fn into_tex(self) -> &'static str {
        match (self.kind, self.ctrl) {
            (BracketKind::Paren, GroupControl::Open ) => r"{\br({",
            (BracketKind::Paren, GroupControl::Close) => r"})}",
            (BracketKind::Brack, GroupControl::Open ) => r"{\br[{",
            (BracketKind::Brack, GroupControl::Close) => r"}]}",
            (BracketKind::Brace, GroupControl::Open ) => r"{\br\{{",
            (BracketKind::Brace, GroupControl::Close) => r"}\}",
        }
    }

    pub fn try_from(token: &str) -> Option<Self> {
        let kind = match token {
            "(" | ")" => BracketKind::Paren,
            "[" | "]" => BracketKind::Brack,
            "{" | "}" => BracketKind::Brace,
            _ => return None,
        };
        let ctrl = match token {
            "(" | "[" | "{" => GroupControl::Open,
            ")" | "]" | "}" => GroupControl::Close,
            _ => return None,
        };
        Some(Self { kind, ctrl })
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TrigFn {
    /// sin or csc
    Sin,
    /// cos or sec
    Cos,
    /// tan or cot
    Tan,
}

#[derive(Debug, Clone, Copy)]
pub struct TrigFnToken {
    /// - sin (or csc)
    /// - cos (or sec)
    /// - tan (or cot)
    pub func: TrigFn,

    /// - sin <-> csc
    /// - cos <-> sec
    /// - tan <-> cot
    pub is_reciprocal: bool,

    /// ______h
    pub is_hyperbolic: bool,

    /// arc____
    pub is_inverse: bool,
}

impl TrigFnToken {
    const INVERSE_PREFIX: &'static str = "arc";
    const HYPERBOLIC_SUFFIX: &'static str = "h";

    pub fn into_tex(self) -> &'static str {
        let full = match (self.func, self.is_reciprocal) {
            (TrigFn::Sin, false) => r"\arcsinh",
            (TrigFn::Cos, false) => r"\arccosh",
            (TrigFn::Tan, false) => r"\arctanh",
            (TrigFn::Sin,  true) => r"\arccsch",
            (TrigFn::Cos,  true) => r"\arcsech",
            (TrigFn::Tan,  true) => r"\arccoth",
        };
        let start = if self.is_inverse { 0 } else { Self::INVERSE_PREFIX.len() };
        let end = full.len() - if self.is_hyperbolic { 0 } else { Self::HYPERBOLIC_SUFFIX.len() };
        &full[start..end]
    }

    pub fn try_from(token: &str) -> Option<Self> {
        let is_inverse = token.starts_with(Self::INVERSE_PREFIX);
        let token_fn_start = if is_inverse { Self::INVERSE_PREFIX.len() } else { 0 };
        let is_hyperbolic = token.ends_with(Self::HYPERBOLIC_SUFFIX);
        let token_fn_end = token.len() - if is_hyperbolic { Self::HYPERBOLIC_SUFFIX.len() } else { 0 };
        let token_fn = &token[token_fn_start..token_fn_end];
        let is_reciprocal = matches!(token_fn, "csc" | "sec" | "cot");
        let func = match token_fn {
            "sin" | "csc" => TrigFn::Sin,
            "cos" | "sec" => TrigFn::Cos,
            "tan" | "cot" => TrigFn::Tan,
            _ => return None,
        };
        Some(Self { func, is_reciprocal, is_hyperbolic, is_inverse })
    }
}

#[derive(Debug, Clone, Copy)]
pub enum BuiltinWordToken {
    Sqrt,
    Log,
    Ln,
    Sum,
    Prod,

    E,
    Pi,
    VarPhi,

    Theta,
    Phi,
    Psi,

    VarNothing,

    Trig(TrigFnToken),
}

impl BuiltinWordToken {
    // regex() is not defined, because it is already caught by `rx_word`.

    pub fn into_tex(self) -> &'static str {
        match self {
            Self::Sqrt       => r"\sqrt",
            Self::Log        => r"\log",
            Self::Ln         => r"\ln",
            Self::Sum        => r"\sum",
            Self::Prod       => r"\prod",

            Self::Pi         => r"\pi",
            Self::VarPhi     => r"\varphi",
            Self::E          => "e",

            Self::Theta      => r"\theta",
            Self::Phi        => r"\phi",
            Self::Psi        => r"\psi",

            Self::VarNothing => r"\varnothing",

            Self::Trig(trig_fn_token) => trig_fn_token.into_tex(),
        }
    }

    pub fn try_from(token: &str) -> Option<Self> {
        match token {
            "sqrt" => Some(Self::Sqrt),
            "log"  => Some(Self::Log),
            "ln"   => Some(Self::Ln),
            "sum"  => Some(Self::Sum),
            "prod" => Some(Self::Prod),

            "pi"              => Some(Self::Pi),
            "varphi" | "gold" => Some(Self::VarPhi),
            "e"               => Some(Self::E),

            "theta" => Some(Self::Theta),
            "phi"   => Some(Self::Phi),
            "psi"   => Some(Self::Psi),

            "none" | "empty" => Some(Self::VarNothing),
            
            _ =>  TrigFnToken::try_from(token)
                .map(|trig| Self::Trig(trig)),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Token<'doc> {
    /// Name of a variable, constant, or function
    Word(&'doc str),
    /// A literal number (excluding mathematical constants)
    Number(&'doc str),
    /// Name of a built-in variable, constant, or function
    BuiltinWord(BuiltinWordToken),
    /// A binary or relational operator
    Operator(OperatorToken),
    /// Brackets
    GroupCtrl(GroupCtrlToken),
}

impl<'doc> Token<'doc> {
    pub fn into_tex(self) -> &'doc str {
        match self {
            | Self::Word  (token)
            | Self::Number(token)
                => token,

            Self::BuiltinWord(bw_token) => bw_token.into_tex(),
            Self::Operator   (op_token) => op_token.into_tex(),
            Self::GroupCtrl  (gc_token) => gc_token.into_tex(),
        }
    }
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
                    match BuiltinWordToken::try_from(&token_str) {
                        Some(bw_token) => Token::BuiltinWord(bw_token),
                        None => Token::Word(token_str)
                    }
                } else {
                    panic!("Unrecognized token: \"{token_str}\"");
                }
            })
            .collect()
    }
}

