use regex::Regex;

macro_rules! const_len {
    () => {
        0
    };

    ($_first:tt $($rest:tt)*) => {
        1 + const_len!($($rest)*)
    };
}

macro_rules! operator_tokens {
    {
        #[$meta:meta]
        enum $name:ident {
            $([$($token:literal),+ $(,)?] => $variant:ident => $tex:literal),+ $(,)?
        }
    } => {
        #[$meta]
        pub enum $name {
            $($variant,)*
        }

        impl $name {
            const TOKENS: [&'static str; const_len!($($($token)*)*)] = [
                $($($token,)*)*
            ];

            pub fn into_tex(self) -> &'static str {
                match self {
                    $(Self::$variant => $tex,)*
                }
            }

            pub fn try_from(token: &str) -> Option<Self> {
                match token {
                    $($(| $token)* => Some(Self::$variant),)*
                    _ => None,
                }
            }
        }
    };
}

operator_tokens!{
    #[derive(Debug, Clone, Copy)]
    enum OperatorToken {
        ["<=>"]       => Iff          => r"\iff",
        ["==>", "=>"] => Implies      => r"\implies",
        ["<=="]       => Impliedby    => r"\impliedby",
        ["==="]       => Equiv        => r"\equiv",
        ["!=="]       => NEquiv       => r"\nequiv",
        ["+/-"]       => Pm           => r"\pm",
        ["-/+"]       => Mp           => r"\mp",
        ["->"]        => To           => r"\to",
        ["<-"]        => Gets         => r"\gets",
        ["==", "="]   => Eq           => "=",
        ["!=", "=/="] => Ne           => r"\ne",
        [">"]         => Gt           => ">",
        [">="]        => Ge           => r"\ge",
        [r"/\"]       => Wedge        => r"\bigwedge",
        [r"\/"]       => Vee          => r"\bigvee",
        ["<"]         => Lt           => r"<",
        ["<="]        => Le           => r"\le",
        ["!"]         => Factorial    => "!",
        ["+"]         => Plus         => "+",
        ["-"]         => Minus        => "-",
        ["*"]         => CDot         => r"\cdot",
        ["/"]         => Frac         => r"\frac",
        ["^"]         => Superscript  => "^",
        ["_"]         => Subscript    => "_",
        ["~"]         => Sim          => r"\sim",
        ["'"]         => Prime        => r"\prime",
        [","]         => Comma        => ",",
        [":"]         => Colon        => ":",
        ["|"]         => Union        => r"\cup",
        ["&"]         => Intersection => r"\cap",
        ["in"]        => In           => r"\in",
        ["and"]       => And          => r"\land",
        ["nand"]      => Nand         => r"\lnand",
        ["or"]        => Or           => r"\lor",
        ["nor"]       => Nor          => r"\lnor",
        ["xor"]       => Xor          => r"\lxor",
        ["xnor"]      => Xnor         => r"\lxnor",
        ["where"]     => Where        => r"\where",
    }
}

impl OperatorToken {
    pub fn regex() -> Regex {
        let mut tokens = Self::TOKENS.map(|token| {
            let [front_b, back_b] = [
                token.chars().next(),
                token.chars().next_back()
            ].map(|x| if char::is_alphabetic(x.unwrap()) { r"\b" } else { "" });
            format!("{front_b}{}{back_b}", regex::escape(token))
        });
        tokens.sort_by(|a, b| b.len().cmp(&a.len()));
        Regex::new(tokens.join("|").as_str()).unwrap()
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

#[derive(Debug, Clone, Copy)]
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
                    Token::Word(match BuiltinWordToken::try_from(&token_str) {
                        Some(bw_token) => WordToken::Builtin(bw_token),
                        None => WordToken::Direct(token_str)
                    })
                } else {
                    panic!("Unrecognized token: \"{token_str}\"");
                }
            })
            .collect()
    }
}

