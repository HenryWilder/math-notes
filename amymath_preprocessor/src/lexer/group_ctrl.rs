use crate::to_tex::ToTex;

#[derive(Debug, Clone, Copy)]
pub enum GroupControl {
    Open,
    Close,
}

macro_rules! group_ctrl_tokens {
    {
        #[$kind_meta:meta]
        $kind_vis:vis enum $kind_name:ident;
        #[$token_meta:meta]
        $token_vis:vis struct $token_name:ident {
            $($kind:ident($src_open:literal, $src_close:literal) => ($out_open:literal, $out_close:literal),)*
        }
    } => {
        #[$kind_meta]
        $kind_vis enum $kind_name {
            $($kind,)*
        }
        
        #[$token_meta]
        $token_vis struct $token_name {
            pub kind: $kind_name,
            pub ctrl: GroupControl,
        }

        impl GroupCtrlToken {
            pub fn regex_items() -> Vec<String> {
                let mut tokens = [
                    $($src_open, $src_close,)*
                ].into_iter().map(regex::escape).collect::<Vec<_>>();
                tokens.sort_by(|a, b| b.len().cmp(&a.len()));
                tokens
            }
        
            pub fn try_from(token: &str) -> Option<Self> {
                let kind = match token {
                    $($src_open | $src_close => BracketKind::$kind,)*
                    _ => return None,
                };
                let ctrl = match token {
                    $($src_open )|* => GroupControl::Open,
                    $($src_close)|* => GroupControl::Close,
                    _ => return None,
                };
                Some(Self { kind, ctrl })
            }
        }
        
        impl ToTex for GroupCtrlToken {
            fn to_tex(self) -> String {
                match (self.kind, self.ctrl) {
                    $(
                        (BracketKind::$kind, GroupControl::Open ) => format!(r"{{\br{{{}}}{{", $out_open),
                        (BracketKind::$kind, GroupControl::Close) => format!("}}{{{}}}}}", $out_close),
                    )*
                }
            }
        }
    };
}

group_ctrl_tokens!{
    #[derive(Debug, Clone, Copy)]
    pub enum BracketKind;

    #[derive(Debug, Clone, Copy)]
    pub struct GroupCtrlToken {
        Paren(  "(", ")"  ) => (r"\lparen", r"\rparen"),
        Brack(  "[", "]"  ) => (r"\lbrack", r"\rbrack"),
        Brace(  "{", "}"  ) => (r"\lbrace", r"\rbrace"),
        Vert ("||(", ")||") => ( r"\lVert", r"\rVert" ),
        VVert( "|(", ")|" ) => ( r"\lvert", r"\rvert" ),
    }
}
