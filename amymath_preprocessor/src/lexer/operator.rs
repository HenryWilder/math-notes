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
            "+/-" => #[Binary]        Pm    => r"\pm",
            "-/+" => #[Binary]        Mp    => r"\mp",
            "+"   => #[Binary]        Plus  => "+",
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
            "&" | "cap" | "intersection" => #[Binary] Intersection => r"\cap",
        },
        {
            "|" | "cup" | "union" => #[Binary] Union => r"\cup",
        },
        {
            r"/\"  => #[Binary] Wedge => r"\bigwedge",
            "and"  => #[Binary] And   => r"\land",
            "nand" => #[Binary] Nand  => r"\lnand",
        },
        {
            "xor"  => #[Binary] Xor  => r"\lxor",
            "xnor" => #[Binary] Xnor => r"\lxnor",
        },
        {
            r"\/" => #[Binary] Vee => r"\bigvee",
            "or"  => #[Binary] Or  => r"\lor",
            "nor" => #[Binary] Nor => r"\lnor",
        },
        {
            r"\" => #[Binary] Setminus => r"\setminus",
            ","  => #[Binary] Comma    => ",",
        },
        {
            ":" => #[Binary] Colon => ":",
        },
        {
            "|=>" | "|->" => #[Binary] MapsTo   => r"\mapsto",
            "<=|" | "<-|" => #[Binary] MapsFrom => r"\mapsfrom",
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
