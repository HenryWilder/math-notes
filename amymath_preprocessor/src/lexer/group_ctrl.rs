use crate::to_tex::ToTex;

macro_rules! group_ctrl_tokens {
    {
        $(#[$kind_meta:meta])*
        $kind_vis:vis enum $kind_name:ident;
        shared_format = ($open_fmt:literal, $close_fmt:literal);

        $(#[$token_meta:meta])*
        $token_vis:vis struct $token_name:ident {
            $(
                $(#[$kind_variant_meta:meta])*
                $kind:ident ($src_open:literal, $src_close:literal) => ($out_open:literal, $out_close:literal),
            )*
        }
    } => {
        $(#[$kind_meta])*
        $kind_vis enum $kind_name {
            $(
                $(#[$kind_variant_meta])*
                $kind,
            )*
        }

        $(#[$token_meta])*
        $token_vis struct $token_name {
            pub kind: $kind_name,
            pub ctrl: GroupControl,
        }

        impl $token_name {
            pub fn regex_items() -> Vec<String> {
                let mut tokens = [
                    $($src_open, $src_close,)*
                ].into_iter().map(regex::escape).collect::<Vec<_>>();
                tokens.sort_by(|a, b| b.len().cmp(&a.len()));
                tokens
            }

            pub fn try_from(token: &str) -> Option<Self> {
                Some(Self {
                    kind: match token {
                        $($src_open | $src_close => $kind_name::$kind,)*
                        _ => return None,
                    },
                    ctrl: match token {
                        $($src_open )|* => GroupControl::Open,
                        $($src_close)|* => GroupControl::Close,
                        _ => return None,
                    }
                })
            }

            /// The string that would be used in the source document to represent this delimiter.
            pub fn source_str(&self) -> &'static str {
                match self.ctrl {
                    GroupControl::Open => match self.kind {
                        $($kind_name::$kind => $src_open,)*
                    },
                    GroupControl::Close => match self.kind {
                        $($kind_name::$kind => $src_close,)*
                    },
                }
            }
        }

        impl ToTex for $token_name {
            fn to_tex(self) -> String {
                match self.ctrl {
                    GroupControl::Open => format!($open_fmt,
                        match self.kind {
                            $($kind_name::$kind => $out_open,)*
                        }
                    ),
                    GroupControl::Close => format!($close_fmt,
                        match self.kind {
                            $($kind_name::$kind => $out_close,)*
                        }
                    ),
                }
            }
        }
    };
}

/// Whether the delimiter is pushing vs popping scope
#[derive(Debug, Clone, Copy)]
pub enum GroupControl {
    Open,
    Close,
}

group_ctrl_tokens!{
    /// What pairing the token represents.
    #[derive(Debug, Clone, Copy)]
    pub enum BracketKind;

    shared_format = (r"{{\br{{{}}}{{", "}}{{{}}}}}");

    /// A delimiter marking the start or end of a subexpression.
    #[derive(Debug, Clone, Copy)]
    pub struct GroupCtrlToken {
        /// Parentheses `(...)`
        Paren (  "(", ")"  ) => (r"\lparen", r"\rparen"),
        /// Brackets `[...]`
        Brack (  "[", "]"  ) => (r"\lbrack", r"\rbrack"),
        /// Braces `\{...\}`
        Brace (  "{", "}"  ) => (r"\lbrace", r"\rbrace"),
        /// <u>V</u>ert `\|...\|`
        VVert ("||(", ")||") => ( r"\lVert", r"\rVert" ),
        /// Vert `|...|`
        Vert  ( "|(", ")|" ) => ( r"\lvert", r"\rvert" ),
    }
}

impl BracketKind {
    pub fn is_compatible(&self, other: &Self) -> bool {
        matches!((self, other),
            | (BracketKind::Brace, BracketKind::Brace)
            | (BracketKind::Paren | BracketKind::Brack, BracketKind::Paren | BracketKind::Brack)
            | (BracketKind::VVert, BracketKind::VVert)
            | (BracketKind::Vert,  BracketKind::Vert )
        )
    }
}
