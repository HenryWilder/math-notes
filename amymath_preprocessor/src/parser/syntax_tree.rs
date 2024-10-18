#![allow(dead_code)]
use std::fmt::Debug;

use crate::lexer::*;

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
                assert!(matches!(group.0[..], [
                    SyntaxNode::Token(Token::GroupCtrl(GroupCtrlToken { kind: _, ctrl: GroupControl::Open })),
                    ..,
                    SyntaxNode::Token(Token::GroupCtrl(GroupCtrlToken { kind: _, ctrl: GroupControl::Close })),
                ]), "Groups should include their delimiters");
                let end = group.0.len() - 1;
                group.0
                    .into_iter()
                    .take(end)
                    .skip(1)
                    .map(|node| node.into_tex())
                    .collect::<Vec<String>>()
                    .join(" ")
            },
            _ => self.into_tex(),
        }
    }

    pub fn into_tex(self) -> String {
        match self {
            SyntaxNode::Token(token) => token.into_tex().to_owned(),
            SyntaxNode::BinOp { lhs, op, rhs } => match op {
                OperatorToken::Frac => format!("\\op{{{}{{\\lit{{{}}}}}{{\\lit{{{}}}}}}}", op.into_tex(), lhs.extract_inner_tex(), rhs.extract_inner_tex() ),
                _ => format!("{{\\lit{{{}}}}}\\op{{{}}}{{\\lit{{{}}}}}", lhs.into_tex(), op.into_tex(), rhs.into_tex()),
            },
            SyntaxNode::PreOp { op, rhs } => format!("\\op{{{}}}{{\\lit{{{}}}}}", op.into_tex(), rhs.into_tex()),
            SyntaxNode::SufOp { lhs, op } => format!("{{\\lit{{{}}}}}\\op{{\\lit{{{}}}}}", lhs.into_tex(), op.into_tex()),
            SyntaxNode::Group(group) => group.into_tex(),
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

    pub fn into_tex(self) -> String {
        self.0
            .into_iter()
            .map(|node| node.into_tex())
            .collect::<Vec<String>>()
            .join(" ")
    }
}
