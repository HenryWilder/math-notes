use crate::to_tex::ToTex;

macro_rules! operator_tokens {
    {
        #[$meta:meta]
        $vis:vis enum $name:ident { $(
            { $(
                #[$kind:ident, $($argn:ident)|+]
                $($token:literal)|+ => $variant:ident => $tex:literal,
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

            pub fn try_from(token: &str) -> Option<Self> {
                match token {
                    $($($(| $token)* => Some(Self::$variant),)*)*
                    _ => None,
                }
            }

            pub fn nary(&self) -> Vec<NAry> {
                match self {
                    $($(Self::$variant => vec![$(NAry::$argn),*],)*)*
                }
            }

            pub fn kind(&self) -> OpType {
                match self {
                    $($(Self::$variant => OpType::$kind,)*)*
                }
            }
        }

        impl ToTex for $name {
            fn to_tex(self) -> String {
                match self {
                    $($(Self::$variant => $tex,)*)*
                }.to_string()
            }
        }
    };
}

#[derive(Debug)]
pub enum OpType {
    /// Represents an operation (\mathbin)
    Operation,
    /// Represents an assertion (\mathrel)
    Assertion,
}

impl ToTex for OpType {
    fn to_tex(self) -> String {
        match self {
            OpType::Operation => r"\op",
            OpType::Assertion => r"\stmt",
        }.to_string()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum NAry {
    /// Binary
    Binary,
    /// Unary prefix
    Prefix,
    /// Unary suffix (postfix)
    Suffix,
}

impl OperatorToken {
    pub fn regex_items() -> Vec<String> {
        let mut tokens: Vec<_> = Self::TOKENS
            .iter()
            .map(|token| {
                let [front_b, back_b] = [
                    token.chars().next(),
                    token.chars().next_back()
                ].map(|x| if char::is_alphabetic(x.unwrap()) { r"\b" } else { "" });
                format!("{front_b}{}{back_b}", regex::escape(token))
            }).collect();
        tokens.sort_by(|a, b| b.len().cmp(&a.len()));
        tokens
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
            #[Operation, Binary] "_" => Subscript => "_",
        },
        {
            #[Operation, Suffix] "!"   => Factorial => "!",
            #[Operation, Suffix] "'"   => Prime     => r"\prime",
            #[Operation, Prefix] "not" => Not       => r"\lnot",
        },
        {
            #[Operation, Binary] "^" => Superscript  => "^",
        },
        {
            #[Operation, Binary] "*" => CDot => r"\cdot",
            #[Operation, Binary] "/" => Frac => r"\frac",
        },
        {
            #[Operation, Binary|Prefix] "+/-" => Pm    => r"\pm",
            #[Operation, Binary|Prefix] "-/+" => Mp    => r"\mp",
            #[Operation, Binary|Prefix] "+"   => Plus  => "+",
            #[Operation, Binary|Prefix] "-"   => Minus => "-",
        },
        {
            #[Operation, Binary] "choose" => Choose => r"\binom",
        },
        {
            #[Operation, Binary] "->" => To   => r"\to",
            #[Operation, Binary] "<-" => Gets => r"\gets",
        },
        {
            #[Assertion, Binary] ">"  => Gt  => ">",
            #[Assertion, Binary] ">=" => Ge  => r"\ge",
            #[Assertion, Binary] "<"  => Lt  => r"<",
            #[Assertion, Binary] "<=" => Le  => r"\le",
            #[Assertion, Binary] "in" => In  => r"\in",
            #[Assertion, Binary] "~"  => Sim => r"\sim",
        },
        {
            #[Assertion, Binary] "==" | "="   => Eq     => "=",
            #[Assertion, Binary] "!=" | "=/=" => Ne     => r"\ne",
            #[Assertion, Binary] "==="        => Equiv  => r"\equiv",
            #[Assertion, Binary] "!=="        => NEquiv => r"\nequiv",
        },
        {
            #[Operation, Binary] "&" | "cap" | "intersection" => Intersection => r"\cap",
        },
        {
            #[Operation, Binary] "|" | "cup" | "union" => Union => r"\cup",
        },
        {
            #[Operation, Binary] r"/\"  => Wedge => r"\bigwedge",
            #[Operation, Binary] "and"  => And   => r"\land",
            #[Operation, Binary] "nand" => Nand  => r"\lnand",
        },
        {
            #[Operation, Binary] "xor"  => Xor  => r"\lxor",
            #[Operation, Binary] "xnor" => Xnor => r"\lxnor",
        },
        {
            #[Operation, Binary] r"\/" => Vee => r"\bigvee",
            #[Operation, Binary] "or"  => Or  => r"\lor",
            #[Operation, Binary] "nor" => Nor => r"\lnor",
        },
        {
            #[Operation, Binary] r"\" => Setminus => r"\setminus",
            // #[Operation, Binary] ","  => Comma    => ",",
        },
        {
            #[Assertion, Binary] ":" => Colon => ":",
        },
        {
            #[Assertion, Binary] "|=>" | "|->" => MapsTo   => r"\mapsto",
            #[Assertion, Binary] "<=|" | "<-|" => MapsFrom => r"\mapsfrom",
        },
        {
            #[Assertion, Binary] "==>" | "=>" => Implies   => r"\implies",
            #[Assertion, Binary] "<=="        => Impliedby => r"\impliedby",
            #[Assertion, Binary] "<=>"        => Iff       => r"\iff",
        },
        {
            #[Assertion, Binary] "so" => Therefore => r"\therefore",
        },
        {
            #[Assertion, Binary] "where" => Where => r"\where",
        },
    }
}
