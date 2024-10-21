use crate::{to_tex::ToTex, parser::syntax_tree::SyntaxNode};

macro_rules! as_one {
    ($item:tt) => {
        1
    }
}

macro_rules! const_len {
    ($($item:tt),*) => {
        0 $(+ as_one!($item))*
    };
}

macro_rules! op_fmt_pat {
    (($name:ident)) => {
        $name
    };

    ($name:ident) => {
        $name
    };
}

macro_rules! op_fmt_arg {
    ($name:ident) => {
        $name.to_tex()
    };

    (($name:ident)) => {
        $name.extract_inner().to_tex()
    };
}

macro_rules! operator_tokens {
    {
        $(#[$meta:meta])*
        $vis:vis enum $name:ident { $(
            { $(
                $(#[$variant_meta:meta])*
                @$kind:ident
                $($token:literal)|+ => $variant:ident => $tex:literal,
                $(
                    ([$($lhs_fmt:tt),*] $op_fmt:ident $(<$kind_fmt:ident>)? [$($rhs_fmt:tt),*]) => $nary_fmt:literal,
                )+
            )* },
        )* }
    } => {
        $(#[$meta])*
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

            /// Try to construct an operator token. Returns `None` if the token isn't an operator.
            pub fn try_from(token: &str) -> Option<Self> {
                match token {
                    $($(
                        $( $token )|* => Some(Self::$variant),
                    )*)*
                    _ => None,
                }
            }

            /// The lookaround range of the operator.
            ///
            /// The range is relative to the operator.
            pub fn nary(&self) -> Vec<(usize, usize)> {
                match self {
                    $($(
                        Self::$variant => vec![$(
                            (
                                const_len!($($lhs_fmt)*),
                                const_len!($($rhs_fmt)*)
                            ),
                        )*],
                    )*)*
                }
            }

            /// Format the operator with its arguments as TeX.
            ///
            /// This method is on the operator token itself instead of in the syntax tree because
            /// different operator tokens have different formatting requirements
            pub fn format(&self, lhs: Vec<SyntaxNode<'_>>, rhs: Vec<SyntaxNode<'_>>) -> String {
                match (self, &lhs[..], &rhs[..]) {
                    $($($(
                        (Self::$variant, [$(op_fmt_pat!($lhs_fmt)),*], [$(op_fmt_pat!($rhs_fmt)),*]) => {
                            let $op_fmt: String = self.to_tex();
                            $(
                                let $kind_fmt: String = self.kind().to_tex();
                            )?
                            $(
                                let op_fmt_pat!($lhs_fmt) = $lhs_fmt.clone();
                                let op_fmt_pat!($lhs_fmt) = op_fmt_arg!($lhs_fmt);
                            )*
                            $(
                                let op_fmt_pat!($rhs_fmt) = $rhs_fmt.clone();
                                let op_fmt_pat!($rhs_fmt) = op_fmt_arg!($rhs_fmt);
                            )*
                            format!($nary_fmt)
                        },
                    )*)*)*
                    _ => unimplemented!("No operator at the time of writing supports the arguments `{lhs:?} op {rhs:?}`. Has {self:?} been correctly implemented?"),
                }
            }

            /// Whether the operator is an assertion or operation.
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

/// Whether the [`OperatorToken`] is an operator or assertion
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

impl OperatorToken {
    /// Lists every source token with regex escaping
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

    /// The order in which the operator should be evaluated
    pub fn precedence(&self) -> usize {
        Self::PRECEDENCES.iter()
            .enumerate()
            .find_map(|(n, items)| items.contains(self).then_some(n))
            .unwrap()
    }

    /// How much the operator wants to bind with its current arguments.
    /// Higher binding power ops will clump with arguments before allowing lower binding power ops to see them.
    pub fn bind_power(&self) -> usize {
        Self::PRECEDENCES.len() - self.precedence()
    }
}

operator_tokens!{
    /// A token that specifically represents an operator.
    /// Operators look around to find their arguments.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum OperatorToken {
        {
            /// Distinguishment or collection indexing
            @Operation "_" => Subscript => "_",
            ([l0] op [(r0)]) => r"{{{l0}}}{op}{{{r0}}}",
        },
        {
            /// Factorial
            @Operation "!" => Factorial => "!",
            ([l0] op<kind> []) => r"{{{l0}}}{kind}{{{op}}}",

            /// Lagrange derivative notation
            @Operation "'" => Prime => r"\prime",
            ([l0] op<kind> []) => r"{{{l0}}}^{{{kind}{{{op}}}}}",

            /// Logical NOT
            @Operation "not" => Not => r"\lnot",
            ([] op<kind> [r0]) => r"{kind}{{{op}}}{{{r0}}}",
        },
        {
            /// Exponent
            @Operation "^" => Superscript => "^",
            ([base] op<kind> [(power)]) => r"{{{base}}}{kind}{{{op}}}{{{power}}}",
        },
        {
            /// Multiplication
            @Operation "*" => CDot => r"\cdot",
            ([l0] op<kind> [r0]) => r"{{{l0}}}{kind}{{{op}}}{{{r0}}}",

            /// Division
            @Operation "/" => Frac => r"\frac",
            ([(numer)] op<kind> [(denom)]) => r"{kind}{{{op}{{{numer}}}{{{denom}}}}}",
        },
        {
            /// Addition or subtraction
            @Operation "+/-" => Pm => r"\pm",
            ([l0] op<kind> [r0]) => r"{{{l0}}}{kind}{{{op}}}{{{r0}}}",
            ([] op<kind> [r0]) => r"{kind}{{{op}}}{{{r0}}}",

            /// Subtraction or addition
            @Operation "-/+" => Mp => r"\mp",
            ([l0] op<kind> [r0]) => r"{{{l0}}}{kind}{{{op}}}{{{r0}}}",
            ([] op<kind> [r0]) => r"{kind}{{{op}}}{{{r0}}}",

            /// Addition
            @Operation "+" => Plus => "+",
            ([l0] op<kind> [r0]) => r"{{{l0}}}{kind}{{{op}}}{{{r0}}}",
            ([] op<kind> [r0]) => r"{kind}{{{op}}}{{{r0}}}",

            /// Subtraction or negation
            @Operation "-" => Minus => "-",
            ([l0] op<kind> [r0]) => r"{{{l0}}}{kind}{{{op}}}{{{r0}}}",
            ([] op<kind> [r0]) => r"{kind}{{{op}}}{{{r0}}}",
        },
        {
            /// Binomial coefficient
            @Operation "choose" => Choose => r"\binom",
            ([l0] op<kind> [r0]) => r"{{{l0}}}{kind}{{{op}}}{{{r0}}}",
        },
        {
            /// Limit approach
            @Operation "->" => To => r"\to",
            ([l0] op<kind> [r0]) => r"{{{l0}}}{kind}{{{op}}}{{{r0}}}",

            /// Reserved for future assignment
            @Operation "<-" => Gets => r"\gets",
            ([l0] op<kind> [r0]) => r"{{{l0}}}{kind}{{{op}}}{{{r0}}}",
        },
        {
            /// Greater than
            @Assertion ">" => Gt => ">",
            ([l0] op<kind> [r0]) => r"{{{l0}}}{kind}{{{op}}}{{{r0}}}",

            /// Greater than or equal to
            @Assertion ">=" => Ge => r"\ge",
            ([l0] op<kind> [r0]) => r"{{{l0}}}{kind}{{{op}}}{{{r0}}}",

            /// Less than
            @Assertion "<" => Lt => r"<",
            ([l0] op<kind> [r0]) => r"{{{l0}}}{kind}{{{op}}}{{{r0}}}",

            /// Less than or equal to
            @Assertion "<=" => Le => r"\le",
            ([l0] op<kind> [r0]) => r"{{{l0}}}{kind}{{{op}}}{{{r0}}}",

            /// Element of
            @Assertion "in" => In => r"\in",
            ([l0] op<kind> [r0]) => r"{{{l0}}}{kind}{{{op}}}{{{r0}}}",

            /// Similar to
            @Assertion "~" => Sim => r"\sim",
            ([l0] op<kind> [r0]) => r"{{{l0}}}{kind}{{{op}}}{{{r0}}}",
        },
        {
            /// For all
            @Assertion "for all" => Forall => r"\forall",
            ([] op<kind> [r0]) => r"{kind}{{{op}}}{{{r0}}}",

            /// There exists
            @Assertion "exists" => Exists => r"\exists",
            ([] op<kind> [r0]) => r"{kind}{{{op}}}{{{r0}}}",

            /// There does not exist
            @Assertion "exists no" => NExists => r"\nexists",
            ([] op<kind> [r0]) => r"{kind}{{{op}}}{{{r0}}}",

            /// There exists a unique
            @Assertion "!exists" => ExistsUnique => r"!\exists",
            ([] op<kind> [r0]) => r"{kind}{{{op}}}{{{r0}}}",

            /// There does not exist a unique
            @Assertion "!exists no" => NExistsUnique => r"!\nexists",
            ([] op<kind> [r0]) => r"{kind}{{{op}}}{{{r0}}}",
        },
        {
            /// Equality
            @Assertion "==" | "=" => Eq => "=",
            ([l0] op<kind> [r0]) => r"{{{l0}}}{kind}{{{op}}}{{{r0}}}",

            /// Inequality
            @Assertion "!=" | "=/=" => Ne => r"\ne",
            ([l0] op<kind> [r0]) => r"{{{l0}}}{kind}{{{op}}}{{{r0}}}",

            /// Equivalence
            @Assertion "===" => Equiv  => r"\equiv",
            ([l0] op<kind> [r0]) => r"{{{l0}}}{kind}{{{op}}}{{{r0}}}",

            /// Inequivalence
            @Assertion "!==" => NEquiv => r"\nequiv",
            ([l0] op<kind> [r0]) => r"{{{l0}}}{kind}{{{op}}}{{{r0}}}",
        },
        {
            /// Set intersection
            @Operation "&" | "cap" | "intersection" => Intersection => r"\cap",
            ([l0] op<kind> [r0]) => r"{{{l0}}}{kind}{{{op}}}{{{r0}}}",
        },
        {
            /// Set union
            @Operation "|" | "cup" | "union" => Union => r"\cup",
            ([l0] op<kind> [r0]) => r"{{{l0}}}{kind}{{{op}}}{{{r0}}}",
        },
        {
            /// Logical AND (large)
            @Operation r"/\" => Wedge => r"\bigwedge",
            ([l0] op<kind> [r0]) => r"{{{l0}}}{kind}{{{op}}}{{{r0}}}",

            /// Logical AND
            @Operation "and" => And => r"\land",
            ([l0] op<kind> [r0]) => r"{{{l0}}}{kind}{{{op}}}{{{r0}}}",

            /// Logical NAND
            @Operation "nand" => Nand => r"\lnand",
            ([l0] op<kind> [r0]) => r"{{{l0}}}{kind}{{{op}}}{{{r0}}}",
        },
        {
            /// Logical XOR
            @Operation "xor" => Xor => r"\lxor",
            ([l0] op<kind> [r0]) => r"{{{l0}}}{kind}{{{op}}}{{{r0}}}",

            /// Logical XNOR
            @Operation "xnor" => Xnor => r"\lxnor",
            ([l0] op<kind> [r0]) => r"{{{l0}}}{kind}{{{op}}}{{{r0}}}",
        },
        {
            /// Logical OR (large)
            @Operation r"\/" => Vee => r"\bigvee",
            ([l0] op<kind> [r0]) => r"{{{l0}}}{kind}{{{op}}}{{{r0}}}",

            /// Logical OR
            @Operation "or" => Or => r"\lor",
            ([l0] op<kind> [r0]) => r"{{{l0}}}{kind}{{{op}}}{{{r0}}}",

            /// Logical NOR
            @Operation "nor" => Nor => r"\lnor",
            ([l0] op<kind> [r0]) => r"{{{l0}}}{kind}{{{op}}}{{{r0}}}",
        },
        {
            /// Difference of sets
            @Operation r"\" => Setminus => r"\setminus",
            ([l0] op<kind> [r0]) => r"{{{l0}}}{kind}{{{op}}}{{{r0}}}",
        },
        {
            /// Requirement; "x such that [condition]"
            @Assertion ":" => Colon => ":",
            ([l0] op<kind> [r0]) => r"{{{l0}}}{kind}{{{op}}}{{{r0}}}",
        },
        {
            /// Mapping
            @Assertion "|=>" | "|->" => MapsTo => r"\mapsto",
            ([l0] op<kind> [r0]) => r"{{{l0}}}{kind}{{{op}}}{{{r0}}}",

            /// Mapping
            @Assertion "<=|" | "<-|" => MapsFrom => r"\mapsfrom",
            ([l0] op<kind> [r0]) => r"{{{l0}}}{kind}{{{op}}}{{{r0}}}",
        },
        {
            /// If A then B
            @Assertion "==>" | "=>" => Implies => r"\implies",
            ([l0] op<kind> [r0]) => r"{{{l0}}}{kind}{{{op}}}{{{r0}}}",

            /// If B then A
            @Assertion "<==" => Impliedby => r"\impliedby",
            ([l0] op<kind> [r0]) => r"{{{l0}}}{kind}{{{op}}}{{{r0}}}",

            /// A, B only if A AND B
            @Assertion "<=>" => Iff => r"\iff",
            ([l0] op<kind> [r0]) => r"{{{l0}}}{kind}{{{op}}}{{{r0}}}",
        },
        {
            /// Because of A, B is true
            @Assertion "so" => Therefore => r"\therefore",
            ([l0] op<kind> [r0]) => r"{{{l0}}}{kind}{{{op}}}{{{r0}}}",

            /// The reason A is true is because B
            @Assertion "bcus" => Because => r"\because",
            ([l0] op<kind> [r0]) => r"{{{l0}}}{kind}{{{op}}}{{{r0}}}",
        },
        {
            /// The statement requires the following condition(s)
            @Assertion "where" => Where => r"\where",
            ([l0] op<kind> [r0]) => r"{{{l0}}}{kind}{{{op}}}{{{r0}}}",
        },
    }
}
