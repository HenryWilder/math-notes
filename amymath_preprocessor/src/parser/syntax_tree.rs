#![allow(dead_code)]
use std::fmt::Debug;

use crate::{to_tex::ToTex, lexer::*};

#[derive(Clone)]
pub enum SyntaxNode<'doc> {
    /// A normal token
    Token(Token<'doc>),

    /// Binary operator
    BinOp{
        lhs: Box<SyntaxNode<'doc>>,
        op:  OperatorToken,
        rhs: Box<SyntaxNode<'doc>>,
    },

    /// Unary prefix operator
    PreOp{
        op:  OperatorToken,
        rhs: Box<SyntaxNode<'doc>>,
    },

    /// Unary suffix (postfix) operator
    SufOp{
        lhs: Box<SyntaxNode<'doc>>,
        op:  OperatorToken,
    },

    /// First item is opening bracket, last item is closing bracket
    Group(SyntaxTree<'doc>),
}

impl<'doc> Debug for SyntaxNode<'doc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Token(arg0)
                => write!(f, "Token({arg0:?})"),
            Self::BinOp { lhs, op, rhs }
                => f.debug_struct("BinOp").field("lhs", lhs).field("op", op).field("rhs", rhs).finish(),
            Self::PreOp { op, rhs }
                => f.debug_struct("PreOp").field("op", op).field("rhs", rhs).finish(),
            Self::SufOp { lhs, op }
                => f.debug_struct("SufOp").field("lhs", lhs).field("op", op).finish(),
            Self::Group(arg0)
                => f.debug_tuple("Group").field(arg0).finish(),
        }
    }
}

impl<'doc> SyntaxNode<'doc> {
    pub fn new_token(token: Token<'doc>) -> Self {
        Self::Token(token)
    }

    pub fn new_group() -> Self {
        Self::Group(SyntaxTree::new())
    }

    fn extract_inner_tex(self) -> String {
        match self {
            SyntaxNode::Group(group) => {
                let front_offset = match group.0[..] {
                    [SyntaxNode::Token(Token::GroupCtrl(GroupCtrlToken { kind: BracketKind::Paren, ctrl: GroupControl::Open })), ..] => 1,
                    _ => 0,
                };
                let back_offset = match group.0[..] {
                    [.., SyntaxNode::Token(Token::GroupCtrl(GroupCtrlToken { kind: BracketKind::Paren, ctrl: GroupControl::Close }))] => 1,
                    _ => 0,
                };
                let end = group.0.len() - back_offset;
                group.0
                    .into_iter()
                    .take(end)
                    .skip(front_offset)
                    .map(|node| node.to_tex())
                    .collect::<Vec<String>>()
                    .join(" ")
            },
            _ => self.to_tex(),
        }
    }
}

impl<'doc> ToTex for SyntaxNode<'doc> {
    fn to_tex(self) -> String {
        match self {
            SyntaxNode::Token(token)
                => token.to_tex().to_owned(),

            SyntaxNode::BinOp { lhs, op: op @ OperatorToken::Frac, rhs }
                => format!(r"\op{{{}{{\ColorReset{{{}}}}}{{\ColorReset{{{}}}}}}}",
                    op.to_tex(),
                    lhs.extract_inner_tex(),
                    rhs.extract_inner_tex(),
                ),
            SyntaxNode::BinOp { lhs, op: op @ (OperatorToken::Subscript | OperatorToken::Superscript), rhs }
                => format!(r"{{{}}}{}{{{}}}",
                    lhs.to_tex(),
                    op.to_tex(),
                    rhs.extract_inner_tex(),
                ),
            SyntaxNode::BinOp { lhs, op, rhs }
                => format!(r"{{{}}}\op{{{}}}{{{}}}",
                    lhs.to_tex(),
                    op.to_tex(),
                    rhs.to_tex(),
                ),

            SyntaxNode::PreOp { op, rhs }
                => format!(r"\op{{{}}}{{{}}}",
                    op.to_tex(),
                    rhs.to_tex(),
                ),

            SyntaxNode::SufOp { lhs, op: op @ OperatorToken::Prime }
                => format!(r"{{{}}}^{{\op{{{}}}}}",
                    lhs.to_tex(),
                    op.to_tex(),
                ),
            SyntaxNode::SufOp { lhs, op }
                => format!(r"{{{}}}\op{{{}}}",
                    lhs.to_tex(),
                    op.to_tex(),
                ),

            SyntaxNode::Group(group)
                => group.to_tex(),
        }
    }
}

#[derive(Clone)]
pub struct SyntaxTree<'doc>(pub Vec<SyntaxNode<'doc>>);

impl<'doc> Debug for SyntaxTree<'doc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.0.iter()).finish()
    }
}

impl<'doc> SyntaxTree<'doc> {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push_group(&mut self, group: Self) {
        self.0.push(SyntaxNode::Group(group));
    }

    pub fn push_token(&mut self, token: Token<'doc>) {
        self.0.push(SyntaxNode::Token(token));
    }
}

impl<'doc> ToTex for SyntaxTree<'doc> {
    fn to_tex(self) -> String {
        self.0
            .into_iter()
            .map(|node| node.to_tex())
            .collect::<Vec<String>>()
            .join(" ")
    }
}
