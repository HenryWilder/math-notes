use std::fmt::Debug;

use regex::Regex;

#[derive(PartialEq, Eq)]
pub enum NAry {
    /// Binary
    Binary,
    /// Unary prefix
    Prefix,
    /// Unary suffix (postfix)
    Suffix,
}

macro_rules! operator_tokens {
    {
        #[$meta:meta]
        $vis:vis enum $name:ident { $(
            { $(
                $($token:literal)|+ => #[$($argn:ident)|+] $variant:ident => $tex:literal,
            )* },
        )* }
    } => {
        #[$meta]
        $vis enum $name {
            $($($variant,)*)*
        }

        impl $name {
            const TOKENS: &'static [&'static str] = &[
                $($($($token,)*)*)*
            ];

            const PRECEDENCES: &'static [&'static [Self]] = &[
                $(&[$(Self::$variant),*]),*
            ];

            pub fn into_tex(self) -> &'static str {
                match self {
                    $($(Self::$variant => $tex,)*)*
                }
            }

            pub fn try_from(token: &str) -> Option<Self> {
                match token {
                    $($($(| $token)* => Some(Self::$variant),)*)*
                    _ => None,
                }
            }

            pub fn nary(&self) -> &'static [NAry] {
                match self {
                    $($(Self::$variant => &[$(NAry::$argn),*],)*)*
                }
            }
        }
    };
}

impl OperatorToken {
    pub fn regex() -> Regex {
        let mut tokens: Vec<_> = Self::TOKENS.iter().map(|token| {
            let [front_b, back_b] = [
                token.chars().next(),
                token.chars().next_back()
            ].map(|x| if char::is_alphabetic(x.unwrap()) { r"\b" } else { "" });
            format!("{front_b}{}{back_b}", regex::escape(token))
        }).collect();
        tokens.sort_by(|a, b| b.len().cmp(&a.len()));
        Regex::new(tokens.join("|").as_str()).unwrap()
    }

    pub fn precedence(&self) -> usize {
        Self::PRECEDENCES.iter()
            .enumerate()
            .find_map(|(n, items)| items.contains(self).then_some(n))
            .unwrap()
    }

    pub fn bind_power(&self) -> usize {
        Self::PRECEDENCES.len() - self.precedence()
    }
}

operator_tokens!{
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum OperatorToken {
        {
            "_" => #[Binary] Subscript => "_",
        },
        {
            "!"   => #[Suffix] Factorial => "!",
            "'"   => #[Suffix] Prime     => r"\prime",
            "not" => #[Prefix] Not       => r"\lnot",
        },
        {
            "^" => #[Binary] Superscript  => "^",
        },
        {
            "*" => #[Binary] CDot => r"\cdot",
            "/" => #[Binary] Frac => r"\frac",
        },
        {
            "+/-" => #[Binary] Pm    => r"\pm",
            "-/+" => #[Binary] Mp    => r"\mp",
            "+"   => #[Binary] Plus  => "+",
            "-"   => #[Binary|Prefix] Minus => "-",
        },
        {
            "->" => #[Binary] To   => r"\to",
            "<-" => #[Binary] Gets => r"\gets",
        },
        {
            ">"  => #[Binary] Gt  => ">",
            ">=" => #[Binary] Ge  => r"\ge",
            "<"  => #[Binary] Lt  => r"<",
            "<=" => #[Binary] Le  => r"\le",
            "in" => #[Binary] In  => r"\in",
            "~"  => #[Binary] Sim => r"\sim",
        },
        {
            "==" | "="   => #[Binary] Eq     => "=",
            "!=" | "=/=" => #[Binary] Ne     => r"\ne",
            "==="        => #[Binary] Equiv  => r"\equiv",
            "!=="        => #[Binary] NEquiv => r"\nequiv",
        },
        {
            r"/\"  => #[Binary] Wedge        => r"\bigwedge",
            "and"  => #[Binary] And          => r"\land",
            "nand" => #[Binary] Nand         => r"\lnand",
            "&"    => #[Binary] Intersection => r"\cap",
        },
        {
            "xor"  => #[Binary] Xor  => r"\lxor",
            "xnor" => #[Binary] Xnor => r"\lxnor",
        },
        {
            r"\/" => #[Binary] Vee   => r"\bigvee",
            "|"   => #[Binary] Union => r"\cup",
            "or"  => #[Binary] Or    => r"\lor",
            "nor" => #[Binary] Nor   => r"\lnor",
        },
        {
            "," => #[Binary] Comma => ",",
        },
        {
            ":" => #[Binary] Colon => ":",
        },
        {
            "==>" | "=>" => #[Binary] Implies   => r"\implies",
            "<=="        => #[Binary] Impliedby => r"\impliedby",
            "<=>"        => #[Binary] Iff       => r"\iff",
        },
        {
            "so" => #[Binary] Therefore => r"\therefore",
        },
        {
            "where" => #[Binary] Where => r"\where",
        },
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
    // constants
    Pi,
    VarPhi,
    VarNothing,

    // variables
    Theta,
    Phi,
    Psi,

    // functions
    Sqrt,
    Log,
    Ln,
    Sum,
    Prod,
    Trig(TrigFnToken),
}

impl BuiltinWordToken {
    // regex() is not defined, because it is already caught by `rx_word`.

    pub fn into_tex(self) -> &'static str {
        match self {
            // constants
            Self::Pi         => r"\pi",
            Self::VarPhi     => r"\varphi",
            Self::VarNothing => r"\varnothing",

            // variables
            Self::Theta      => r"\theta",
            Self::Phi        => r"\phi",
            Self::Psi        => r"\psi",

            // functions
            Self::Sqrt       => r"\sqrt",
            Self::Log        => r"\log",
            Self::Ln         => r"\ln",
            Self::Sum        => r"\sum",
            Self::Prod       => r"\prod",

            Self::Trig(trig_fn_token) => trig_fn_token.into_tex(),
        }
    }

    pub fn try_from(token: &str) -> Option<Self> {
        match token {
            // constants
            "pi" => Some(Self::Pi),
            "varphi" | "gold" => Some(Self::VarPhi),
            "none" | "empty" => Some(Self::VarNothing),

            // variables
            "theta" => Some(Self::Theta),
            "phi"   => Some(Self::Phi),
            "psi"   => Some(Self::Psi),

            // functions
            "sqrt" => Some(Self::Sqrt),
            "log"  => Some(Self::Log),
            "ln"   => Some(Self::Ln),
            "sum"  => Some(Self::Sum),
            "prod" => Some(Self::Prod),

            _ =>  TrigFnToken::try_from(token)
                .map(|trig| Self::Trig(trig)),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum WordToken<'doc> {
    /// LaTeX is identical to the name
    Direct(&'doc str),

    /// LaTeX is an associated command
    Builtin(BuiltinWordToken),
}

#[derive(Clone, Copy)]
pub enum Token<'doc> {
    /// The name of a variable, constant, or function
    Word(WordToken<'doc>),

    /// A literal number (excluding mathematical constants)
    Number(&'doc str),

    /// A mathematical operator which may look at nodes to its left or right
    Operator(OperatorToken),

    /// A delimiter indicating the start or end of a subexpression
    GroupCtrl(GroupCtrlToken),
}

impl<'doc> Debug for Token<'doc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // force regular debug even if using "pretty" debug
        match self {
            Self::Word(arg0)
                => write!(f, "Word({arg0:?})"),
            Self::Number(arg0)
                => write!(f, "Number({arg0:?})"),
            Self::Operator(arg0)
                => write!(f, "Operator({arg0:?})"),
            Self::GroupCtrl(arg0)
                => write!(f, "GroupCtrl({arg0:?})"),
        }
    }
}

impl<'doc> Token<'doc> {
    pub fn into_tex(self) -> &'doc str {
        match self {
            Self::Word(WordToken::Direct(token)) | Self::Number(token)
                => token,

            Self::Word(WordToken::Builtin(bw_token))
                => bw_token.into_tex(),

            Self::Operator(op_token)
                => op_token.into_tex(),

            Self::GroupCtrl(gc_token)
                => gc_token.into_tex(),
        }
    }
}

#[derive(Debug)]
pub enum LexerError<'doc> {
    UnknownToken{ token: &'doc str },
}

impl<'doc> std::fmt::Display for LexerError<'doc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexerError::UnknownToken{ token }
                => write!(f, "Unrecognized token: `{token}`"),
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

    pub fn tokenize<'doc>(&'_ self, line: &'doc str) -> Result<Vec<Token<'doc>>, LexerError<'doc>> {
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
                    Ok(Token::Word(match BuiltinWordToken::try_from(&token_str) {
                        Some(bw_token) => WordToken::Builtin(bw_token),
                        None => WordToken::Direct(token_str)
                    }))
                } else {
                    Err(LexerError::UnknownToken { token: token_str })
                }
            })
            .collect::<Result<_, _>>()?;

        Ok(tokens)
    }
}

