use crate::to_tex::ToTex;
use crate::processor::DefKind;

macro_rules! builtin_word_tokens {
    {
        $(#[$meta:meta])*
        $vis:vis enum $name:ident {
            $($def_kind:ident {
                $(
                    $(#[$token_meta:meta])*
                    $($src_token:literal)|+ => $token:ident => $out_tex:literal,
                )*
            },)*
        }
    } => {
        $(#[$meta])*
        $vis enum $name {
            $($(
                $(#[$token_meta])*
                $token,
            )*)*
        }

        impl $name {
            /// Try to construct a builtin word token. If `None`, the word is not built in.
            pub fn try_from(token: &str) -> Option<Self> {
                match token {
                    $($($(
                        $src_token)|* => Some(Self::$token),
                    )*)*
                    _ => None,
                }
            }

            /// What DefKind the token represents.
            pub fn kind(&self) -> DefKind {
                match self {
                    $(
                        $(Self::$token)|* => DefKind::$def_kind,
                    )*
                }
            }

            /// Just the command alone, without the DefKind command.
            pub fn command(&self) -> &'static str {
                match self {
                    $($(
                        Self::$token => $out_tex,
                    )*)*
                }
            }
        }
    };
}

builtin_word_tokens!{
    /// A word token whose TeX is a command.
    ///
    /// Can also be a mathematical constant.
    #[derive(Debug, Clone, Copy)]
    pub enum BuiltinWordToken {
        Literal {
            /// Euler's number
            "e" => E => r"e",
            /// Ratio of a circle's diameter to its circumference
            "pi" => Pi => r"\pi",
            /// The golden ratio
            "varphi" | "gold" => VarPhi => r"\varphi",
            /// The empty set
            "none" | "empty" => VarNothing => r"\varnothing",
        },
        Variable {
            /// Typically an angle
            "theta" => Theta => r"\theta",
            /// Typically an angle
            "phi" => Phi => r"\phi",
            /// Typically an angle
            "psi" => Psi => r"\psi",
        },
        Function {
            // Todo: Some of these operators, NOT functions.
            // They are currently erroring in LaTeX because they aren't getting their mandatory arguments.

            /// Square (or n) root
            /// TODO: This is an OPERATOR not a function.
            "sqrt" => Sqrt => r"\sqrt{}",
            /// Logarithm
            "log" => Log => r"\log",
            /// Natural (base-e) logarithm
            "ln" => Ln => r"\ln",
            /// Summation
            "sum" => Sum => r"\sum",
            /// Production
            "prod" => Prod => r"\prod",
            /// Zeta function
            "Zeta" => ZZeta => r"\Zeta",

            /// Sine
            "sin" => Sin => r"\sin",
            /// Cosine
            "cos" => Cos => r"\cos",
            /// Tangent
            "tan" => Tan => r"\tan",
            /// Cosecant
            "csc" => Csc => r"\csc",
            /// Secant
            "sec" => Sec => r"\sec",
            /// Cotangent
            "cot" => Cot => r"\cot",
            /// Hyperbolic Sine
            "sinh" => SinH => r"\sinh",
            /// Hyperbolic Cosine
            "cosh" => CosH => r"\cosh",
            /// Hyperbolic Tangent
            "tanh" => TanH => r"\tanh",
            /// Hyperbolic Cosecant
            "csch" => CscH => r"\csch",
            /// Hyperbolic Secant
            "sech" => SecH => r"\sech",
            /// Hyperbolic Cotangent
            "coth" => CotH => r"\coth",
            /// Inverse Sine
            "arcsin"  => ArcSin => r"\arcsin",
            /// Inverse Cosine
            "arccos"  => ArcCos => r"\arccos",
            /// Inverse Tangent
            "arctan"  => ArcTan => r"\arctan",
            /// Inverse Cosecant
            "arccsc"  => ArcCsc => r"\arccsc",
            /// Inverse Secant
            "arcsec"  => ArcSec => r"\arcsec",
            /// Inverse Cotangent
            "arccot"  => ArcCot => r"\arccot",
            /// Inverse Hyperbolic Sine
            "arcsinh" => ArcSinH => r"\arcsinh",
            /// Inverse Hyperbolic Cosine
            "arccosh" => ArcCosH => r"\arccosh",
            /// Inverse Hyperbolic Tangent
            "arctanh" => ArcTanH => r"\arctanh",
            /// Inverse Hyperbolic Cosecant
            "arccsch" => ArcCscH => r"\arccsch",
            /// Inverse Hyperbolic Secant
            "arcsech" => ArcSecH => r"\arcsech",
            /// Inverse Hyperbolic Cotangent
            "arccoth" => ArcCotH => r"\arccoth",
        },
    }
}

impl ToTex for BuiltinWordToken {
    fn to_tex(self) -> String {
        format!("{}{{{}}}", self.kind().to_tex(), self.command())
    }
}
