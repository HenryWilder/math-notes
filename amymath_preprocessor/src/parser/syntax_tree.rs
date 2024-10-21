#![allow(dead_code)]
use std::fmt::Debug;

use crate::{to_tex::ToTex, lexer::*};

/// A node in a token tree.
#[derive(Clone)]
pub enum SyntaxNode<'doc> {
    /// A normal token
    Token(Token<'doc>),

    /// Operator
    Operator {
        /// Left hand side arguments
        lhs: Vec<SyntaxNode<'doc>>,

        /// Operator
        op:  OperatorToken,

        /// Right hand side arguments
        rhs: Vec<SyntaxNode<'doc>>,
    },

    /// A subtree
    Group {
        /// Implied to be [`GroupControl::Open`].
        open: BracketKind,

        /// The content within the brackets
        inner: SyntaxTree<'doc>,

        /// Implied to be [`GroupControl::Close`].
        close: BracketKind,
    },
}

impl<'doc> Debug for SyntaxNode<'doc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Token(arg0)
                => write!(f, "Token({arg0:?})"), // Ensure non-pretty debug

            Self::Operator { lhs, op, rhs }
                => f.debug_struct("BinOp")
                    .field("lhs", lhs)
                    .field("op", op)
                    .field("rhs", rhs)
                    .finish(),

            Self::Group{ open, inner: subtree, close }
                => f.debug_tuple("Group")
                    .field(open)
                    .field(subtree)
                    .field(close)
                    .finish(),
        }
    }
}

impl<'doc> SyntaxNode<'doc> {
    /// Construct a SyntaxNode representing a single Token and nothing else.
    pub fn new_token(token: Token<'doc>) -> Self {
        Self::Token(token)
    }

    /// Construct a SyntaxNode representing a delimited subtree.
    pub fn new_group() -> Self {
        Self::Group{
            open: BracketKind::Blank,
            inner: SyntaxTree::new(),
            close: BracketKind::Blank,
        }
    }

    /// If the node within a parenthetical `()` group, get the TeX of the contents of that group without the parentheses.
    pub fn extract_inner(self) -> SyntaxTree<'doc> {
        match self {
            SyntaxNode::Group{ open: BracketKind::Paren, inner, close: BracketKind::Paren }
                => inner,

            _ => SyntaxTree(vec![self]),
        }
    }
}

impl<'doc> ToTex for SyntaxNode<'doc> {
    fn to_tex(self) -> String {
        match self {
            SyntaxNode::Token(token)
                => token.to_tex().to_owned(),

            SyntaxNode::Operator{ lhs, op, rhs } =>
                op.format(lhs, rhs),

            SyntaxNode::Group{ open, inner, close }
                => format!("{}{}{}",
                    GroupCtrlToken::open(open).to_tex(),
                    inner.to_tex(),
                    GroupCtrlToken::close(close).to_tex(),
                ),
        }
    }
}

/// A collection adapter for a [`Vec<SyntaxNode>`] with methods for pushing particular types of nodes.
#[derive(Clone)]
pub struct SyntaxTree<'doc>(pub Vec<SyntaxNode<'doc>>);

impl<'doc> Debug for SyntaxTree<'doc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.0.iter()).finish()
    }
}

impl<'doc> SyntaxTree<'doc> {
    /// Construct an empty tree.
    pub fn new() -> Self {
        Self(Vec::new())
    }

    /// Add a delimited subtree to the tree.
    pub fn push_group(&mut self, open: BracketKind, inner: Vec<SyntaxNode<'doc>>, close: BracketKind) {
        self.0.push(SyntaxNode::Group { open, inner: Self(inner), close });
    }

    /// Add a [`Token`] to the tree.
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
