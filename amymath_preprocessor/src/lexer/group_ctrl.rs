#[derive(Debug, Clone, Copy)]
pub enum BracketKind {
    Paren,
    Brack,
    Brace,
    Vert,
    VVert,
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

// macro_rules! group_ctrl_tokens {
//     {
//         #[$meta:meta]
//     } => {
        
//     };
// }

impl GroupCtrlToken {
    pub fn regex_items() -> Vec<String> {
        let mut tokens = [
            "(",
            ")",
            "[",
            "]",
            "{",
            "}",
            "||(",
            ")||",
            "|(",
            ")|",
        ].into_iter().map(regex::escape).collect::<Vec<_>>();
        tokens.sort_by(|a, b| b.len().cmp(&a.len()));
        tokens
    }

    pub fn into_tex(self) -> &'static str {
        match (self.kind, self.ctrl) {
            (BracketKind::Paren, GroupControl::Open ) => r"{\br\lparen{",
            (BracketKind::Paren, GroupControl::Close) => r"}\rparen}",
            (BracketKind::Brack, GroupControl::Open ) => r"{\br\lbrack{",
            (BracketKind::Brack, GroupControl::Close) => r"}\rbrack}",
            (BracketKind::Brace, GroupControl::Open ) => r"{\br\lbrace{",
            (BracketKind::Brace, GroupControl::Close) => r"}\rbrace}",
            (BracketKind::VVert, GroupControl::Open ) => r"{\br\lVert{",
            (BracketKind::VVert, GroupControl::Close) => r"}\rVert}",
            (BracketKind::Vert,  GroupControl::Open ) => r"{\br\lvert{",
            (BracketKind::Vert,  GroupControl::Close) => r"}\rvert}",
        }
    }

    pub fn try_from(token: &str) -> Option<Self> {
        let kind = match token {
            "(" | ")"     => BracketKind::Paren,
            "[" | "]"     => BracketKind::Brack,
            "{" | "}"     => BracketKind::Brace,
            "||(" | ")||" => BracketKind::VVert,
            "|(" | ")|"   => BracketKind::Vert,
            _ => return None,
        };
        let ctrl = match token {
            "(" | "[" | "{" | "||(" | "|(" => GroupControl::Open,
            ")" | "]" | "}" | ")||" | ")|" => GroupControl::Close,
            _ => return None,
        };
        Some(Self { kind, ctrl })
    }
}
