use crate::lexer::*;

#[allow(dead_code)]
pub mod syntax_tree {
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
                    OperatorToken::Frac => format!("\\op{{{}{{{}}}{{{}}}}}", op.into_tex(), lhs.extract_inner_tex(), rhs.extract_inner_tex() ),
                    _ => format!("{{{}}}\\op{{{}}}{{{}}}", lhs.into_tex(), op.into_tex(), rhs.into_tex()),
                },
                SyntaxNode::PreOp { op, rhs } => format!("\\op{{{}}}{{{}}}", op.into_tex(), rhs.into_tex()),
                SyntaxNode::SufOp { lhs, op } => format!("{{{}}}\\op{{{}}}", lhs.into_tex(), op.into_tex()),
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
}
use syntax_tree::*;

#[allow(dead_code)]
mod stack {
    use std::collections::LinkedList;

    /// A collection adapter
    pub struct Stack<T>(LinkedList<T>);

    impl<T> Stack<T> {
        pub fn new() -> Self {
            Self(LinkedList::new())
        }

        pub fn push(&mut self, item: T) {
            self.0.push_front(item);
        }

        /// Panics if the collection was already empty
        pub fn pop(&mut self) -> T {
            self.0.pop_front().unwrap()
        }

        pub fn is_empty(&self) -> bool {
            self.0.is_empty()
        }

        /// Panics if the collection is empty
        pub fn top(&self) -> &T {
            self.0.front().unwrap()
        }

        /// Panics if the collection is empty
        pub fn top_mut(&mut self) -> &mut T {
            self.0.front_mut().unwrap()
        }
    }
}
use stack::*;

fn group_tokens<'doc>(tokens: Vec<Token<'doc>>) -> SyntaxTree<'doc> {
    let mut stack = Stack::<SyntaxTree>::new();
    stack.push(SyntaxTree::new());
    // Form groups
    for token in tokens {
        match token {
            Token::GroupCtrl(GroupCtrlToken { kind: _, ctrl: GroupControl::Open }) => {
                let mut new_group = SyntaxTree::new();
                new_group.push_token(token);
                stack.push(new_group);
            },

            Token::GroupCtrl(GroupCtrlToken { kind: _, ctrl: GroupControl::Close }) => {
                let mut group = stack.pop();
                group.push_token(token);
                stack.top_mut().push_group(group);
            },

            _ => {
                stack.top_mut().push_token(token);
            },
        }
    }
    let result = stack.pop();
    if !stack.is_empty() {
        panic!("Groups not properly closed");
    }
    result
}

fn group_operators<'doc>(tree: &mut SyntaxTree<'doc>) {
    // DFS
    for node in tree.0.iter_mut() {
        if let SyntaxNode::Group(group) = node {
            group_operators(group);
        }
    }

    'a: loop {
        let operator_tokens: Vec<_> = tree.0
            .iter()
            .enumerate()
            .filter_map(|(i, item)|
                if let SyntaxNode::Token(Token::Operator(op_token)) = item {
                    Some((i, op_token))
                } else {
                    None
                }
            )
            .collect();

        let mut current_target = match operator_tokens.first() {
            Some(&(i, op_token)) => (i, op_token.bind_power()),
            None => break 'a,
        };

        for (i, op_token) in operator_tokens {
            let bind_power = op_token.bind_power();
            if bind_power > current_target.1 {
                current_target = (i, bind_power);
            }
        }
        let (i, _) = current_target;

        if let SyntaxNode::Token(Token::Operator(op_token)) = tree.0[i] {
            let is_lhs_nonnull = i > 0;
            let is_rhs_nonnull = i < tree.0.len() - 1;
            let nary = op_token.nary();
            for argn in nary {
                match (is_lhs_nonnull, argn, is_rhs_nonnull) {
                    (true, NAry::Binary, true) => {
                        let drained: Vec<_> = tree.0.drain((i - 1)..=(i + 1)).collect();
                        let lhs = Box::new(drained[0].clone());
                        let rhs = Box::new(drained[2].clone());
                        tree.0.insert(i - 1, SyntaxNode::BinOp { lhs, op: op_token, rhs, });
                        continue 'a;
                    },

                    (_, NAry::Prefix, true) => {
                        let drained: Vec<_> = tree.0.drain(i..=(i + 1)).collect();
                        let rhs = Box::new(drained[1].clone());
                        tree.0.insert(i - 1, SyntaxNode::PreOp { op: op_token, rhs });
                        continue 'a;
                    },

                    (true, NAry::Suffix, _) => {
                        let drained: Vec<_> = tree.0.drain((i - 1)..=i).collect();
                        let lhs = Box::new(drained[0].clone());
                        tree.0.insert(i - 1, SyntaxNode::SufOp { lhs, op: op_token, });
                        continue 'a;
                    },

                    _ => (),
                }
            }
        }
        break;
    }
}

pub fn parse<'doc>(tokens: Vec<Token<'doc>>) -> SyntaxTree<'doc> {
    let mut tree = group_tokens(tokens);
    group_operators(&mut tree);
    tree
}
