use crate::lexer::*;

#[allow(dead_code)]
pub mod syntax_tree {
    use std::fmt::Debug;

    use crate::lexer::*;

    #[derive(Debug)]
    pub enum SyntaxNode<'doc> {
        Token(Token<'doc>),
        /// First item is opening bracket, last item is closing bracket
        Group(SyntaxTree<'doc>),
    }

    impl<'doc> SyntaxNode<'doc> {
        pub fn new_token(token: Token<'doc>) -> Self {
            Self::Token(token)
        }

        pub fn new_group() -> Self {
            Self::Group(SyntaxTree::new())
        }
    }

    pub struct SyntaxTree<'doc>(Vec<SyntaxNode<'doc>>);

    impl<'doc> Debug for SyntaxTree<'doc> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_list().entries(self.0.iter()).finish()
        }
    }

    impl<'doc> SyntaxTree<'doc> {
        pub fn new() -> Self {
            Self(Vec::new())
        }

        pub fn push_node(&mut self, node: SyntaxNode<'doc>) {
            self.0.push(node);
        }

        pub fn push_group(&mut self, group: Self) {
            self.push_node(SyntaxNode::Group(group));
        }

        pub fn push_token(&mut self, token: Token<'doc>) {
            self.push_node(SyntaxNode::Token(token));
        }

        pub fn iter(&self) -> std::slice::Iter<'_, SyntaxNode<'doc>> {
            self.0.iter()
        }
    }

    impl<'doc> IntoIterator for SyntaxTree<'doc> {
        type Item = SyntaxNode<'doc>;

        type IntoIter = <Vec<Self::Item> as IntoIterator>::IntoIter;

        fn into_iter(self) -> Self::IntoIter {
            self.0.into_iter()
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

pub fn parse<'doc>(tokens: Vec<Token<'doc>>) -> SyntaxTree<'doc> {
    group_tokens(tokens)
}
