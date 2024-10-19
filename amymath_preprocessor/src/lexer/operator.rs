use crate::to_tex::ToTex;

macro_rules! operator_tokens {
    {
        #[$meta:meta]
        $vis:vis enum $name:ident { $(
            { $(
                $(#[$variant_meta:meta])*
                @[$kind:ident, $(l[$left:literal]r[$right:literal])|+]
                $($token:literal)|+ => $variant:ident => $tex:literal,
            )* },
        )* }
    } => {
        #[$meta]
        $vis enum $name {
            $($(
                $(#[$variant_meta])*
                $variant,
            )*)*
        }

        impl $name {
            const TOKENS: &'static [&'static str] = &[
                $($($(
                    $token,
                )*)*)*
            ];

            const PRECEDENCES: &'static [&'static [Self]] = &[
                $(
                    &[$(
                        Self::$variant,
                    )*],
                )*
            ];

            pub fn try_from(token: &str) -> Option<Self> {
                match token {
                    $($(
                        $( $token )|* => Some(Self::$variant),
                    )*)*
                    _ => None,
                }
            }

            pub fn nary(&self) -> Vec<NAry> {
                match self {
                    $($(
                        Self::$variant => vec![$(
                            NAry{
                                n_before: $left,
                                n_after: $right,
                            },
                        )*],
                    )*)*
                }
            }

            pub fn kind(&self) -> OpType {
                match self {
                    $($(
                        Self::$variant => OpType::$kind,
                    )*)*
                }
            }
        }

        impl ToTex for $name {
            fn to_tex(self) -> String {
                match self {
                    $($(
                        Self::$variant => $tex,
                    )*)*
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
pub struct NAry {
    pub n_before: usize,
    pub n_after: usize,
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
            /// Distinguishment or collection indexing
            @[Operation, l[1]r[1]]
            "_" => Subscript => "_",
        },
        {
            @[Operation, l[1]r[0]]
            "!" => Factorial => "!",

            /// Lagrange derivative notation
            @[Operation, l[1]r[0]]
            "'" => Prime => r"\prime",

            /// Logical NOT
            @[Operation, l[0]r[1]]
            "not" => Not => r"\lnot",
        },
        {
            /// Exponent
            @[Operation, l[1]r[1]]
            "^" => Superscript => "^",
        },
        {
            /// Multiplication
            @[Operation, l[1]r[1]]
            "*" => CDot => r"\cdot",

            /// Division
            @[Operation, l[1]r[1]]
            "/" => Frac => r"\frac",
        },
        {
            /// Addition or subtraction
            @[Operation, l[1]r[1] | l[0]r[1]]
            "+/-" => Pm => r"\pm",

            /// Subtraction or addition
            @[Operation, l[1]r[1] | l[0]r[1]]
            "-/+" => Mp => r"\mp",

            /// Addition
            @[Operation, l[1]r[1] | l[0]r[1]]
            "+" => Plus => "+",

            /// Subtraction or negation
            @[Operation, l[1]r[1] | l[0]r[1]]
            "-" => Minus => "-",
        },
        {
            /// Binomial coefficient
            @[Operation, l[1]r[1]]
            "choose" => Choose => r"\binom",
        },
        {
            /// Limit approach
            @[Operation, l[1]r[1]]
            "->" => To => r"\to",

            /// Reserved for future assignment
            @[Operation, l[1]r[1]]
            "<-" => Gets => r"\gets",
        },
        {
            /// Greater than
            @[Assertion, l[1]r[1]]
            ">" => Gt => ">",

            /// Greater than or equal to
            @[Assertion, l[1]r[1]]
            ">=" => Ge => r"\ge",

            /// Less than
            @[Assertion, l[1]r[1]]
            "<" => Lt => r"<",

            /// Less than or equal to
            @[Assertion, l[1]r[1]]
            "<=" => Le => r"\le",

            /// Element of
            @[Assertion, l[1]r[1]]
            "in" => In => r"\in",

            /// Similar to
            @[Assertion, l[1]r[1]]
            "~" => Sim => r"\sim",
        },
        {
            /// Equality
            @[Assertion, l[1]r[1]]
            "==" | "=" => Eq => "=",

            /// Inequality
            @[Assertion, l[1]r[1]]
            "!=" | "=/=" => Ne => r"\ne",

            /// Equivalence
            @[Assertion, l[1]r[1]]
            "===" => Equiv  => r"\equiv",

            /// Inequivalence
            @[Assertion, l[1]r[1]]
            "!==" => NEquiv => r"\nequiv",
        },
        {
            /// Set intersection
            @[Operation, l[1]r[1]]
            "&" | "cap" | "intersection" => Intersection => r"\cap",
        },
        {
            /// Set union
            @[Operation, l[1]r[1]]
            "|" | "cup" | "union" => Union => r"\cup",
        },
        {
            /// Logical AND (large)
            @[Operation, l[1]r[1]]
            r"/\" => Wedge => r"\bigwedge",

            /// Logical AND
            @[Operation, l[1]r[1]]
            "and" => And => r"\land",

            /// Logical NAND
            @[Operation, l[1]r[1]]
            "nand" => Nand => r"\lnand",
        },
        {
            /// Logical XOR
            @[Operation, l[1]r[1]]
            "xor" => Xor => r"\lxor",

            /// Logical XNOR
            @[Operation, l[1]r[1]]
            "xnor" => Xnor => r"\lxnor",
        },
        {
            /// Logical OR (large)
            @[Operation, l[1]r[1]]
            r"\/" => Vee => r"\bigvee",

            /// Logical OR
            @[Operation, l[1]r[1]]
            "or" => Or => r"\lor",

            /// Logical NOR
            @[Operation, l[1]r[1]]
            "nor" => Nor => r"\lnor",
        },
        {
            /// Difference of sets
            @[Operation, l[1]r[1]]
            r"\" => Setminus => r"\setminus",
        },
        {
            /// Requirement; "x such that [condition]"
            @[Assertion, l[1]r[1]]
            ":" => Colon => ":",
        },
        {
            @[Assertion, l[1]r[1]]
            "|=>" | "|->" => MapsTo => r"\mapsto",

            @[Assertion, l[1]r[1]]
            "<=|" | "<-|" => MapsFrom => r"\mapsfrom",
        },
        {
            /// If A then B
            @[Assertion, l[1]r[1]]
            "==>" | "=>" => Implies => r"\implies",

            /// If B then A
            @[Assertion, l[1]r[1]]
            "<==" => Impliedby => r"\impliedby",

            /// A, B only if A AND B
            @[Assertion, l[1]r[1]]
            "<=>" => Iff => r"\iff",
        },
        {
            /// Because of A, B is true
            @[Assertion, l[1]r[1]]
            "so" => Therefore => r"\therefore",

            /// The reason A is true is because B
            @[Assertion, l[1]r[1]]
            "bcus" => Because => r"\because",
        },
        {
            /// The statement requires the following condition(s)
            @[Assertion, l[1]r[1]]
            "where" => Where => r"\where",
        },
    }
}
