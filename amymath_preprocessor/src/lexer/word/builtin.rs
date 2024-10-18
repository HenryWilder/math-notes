use crate::to_tex::ToTex;
use crate::processor::DefKind;

macro_rules! builtin_word_tokens {
    {
        #[$meta:meta]
        $vis:vis enum $name:ident {
            $($def_kind:ident {
                $($($src_token:literal)|+ => $token:ident => $out_tex:literal,)*
            },)*
        }
    } => {
        #[$meta]
        pub enum $name {
            $($($token,)*)*
        }

        impl $name {
            pub fn try_from(token: &str) -> Option<Self> {
                match token {
                    $($($($src_token)|* => Some(Self::$token),)*)*
                    _ => None,
                }
            }

            pub fn kind(&self) -> DefKind {
                match self {
                    $($(Self::$token)|* => DefKind::$def_kind,)*
                }
            }

            pub fn command(&self) -> &'static str {
                match self {
                    $($(Self::$token => $out_tex,)*)*
                }
            }
        }
    };
}

builtin_word_tokens!{
    #[derive(Debug, Clone, Copy)]
    pub enum BuiltinWordToken {
        Literal {
            "e"                => E          => r"e",
            "pi"               => Pi         => r"\pi",
            "varphi" | "gold"  => VarPhi     => r"\varphi",
            "none"   | "empty" => VarNothing => r"\varnothing",
        },
        Variable {
            "theta" => Theta => r"\theta",
            "phi"   => Phi   => r"\phi",
            "psi"   => Psi   => r"\psi",
        },
        Function {
            // Todo: Some of these operators, NOT functions.
            // They are currently erroring in LaTeX because they aren't getting their mandatory arguments.
            "sqrt" => Sqrt => r"\sqrt",
            "log"  => Log  => r"\log",
            "ln"   => Ln   => r"\ln",
            "sum"  => Sum  => r"\sum",
            "prod" => Prod => r"\prod",

               "sin"  =>    Sin  =>    r"\sin",
               "cos"  =>    Cos  =>    r"\cos",
               "tan"  =>    Tan  =>    r"\tan",
               "csc"  =>    Csc  =>    r"\csc",
               "sec"  =>    Sec  =>    r"\sec",
               "cot"  =>    Cot  =>    r"\cot",
               "sinh" =>    SinH =>    r"\sinh",
               "cosh" =>    CosH =>    r"\cosh",
               "tanh" =>    TanH =>    r"\tanh",
               "csch" =>    CscH =>    r"\csch",
               "sech" =>    SecH =>    r"\sech",
               "coth" =>    CotH =>    r"\coth",
            "arcsin"  => ArcSin  => r"\arcsin",
            "arccos"  => ArcCos  => r"\arccos",
            "arctan"  => ArcTan  => r"\arctan",
            "arccsc"  => ArcCsc  => r"\arccsc",
            "arcsec"  => ArcSec  => r"\arcsec",
            "arccot"  => ArcCot  => r"\arccot",
            "arcsinh" => ArcSinH => r"\arcsinh",
            "arccosh" => ArcCosH => r"\arccosh",
            "arctanh" => ArcTanH => r"\arctanh",
            "arccsch" => ArcCscH => r"\arccsch",
            "arcsech" => ArcSecH => r"\arcsech",
            "arccoth" => ArcCotH => r"\arccoth",
        },
    }
}

impl ToTex for BuiltinWordToken {
    fn to_tex(self) -> String {
        format!("{}{{{}}}", self.kind().to_tex(), self.command())
    }
}
