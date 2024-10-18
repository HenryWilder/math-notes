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
