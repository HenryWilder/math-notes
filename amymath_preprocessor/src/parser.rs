use crate::lexer::*;
use crate::stack::*;

pub mod syntax_tree;
pub mod error;

use syntax_tree::*;
use error::ParseError;

fn group_tokens<'doc>(tokens: Vec<Token<'doc>>) -> Result<SyntaxTree<'doc>, ParseError> {
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
                let mut group = stack.pop()
                    .ok_or(ParseError::TooManyCloseBrackets)?;
                group.push_token(token);
                stack.top_mut().unwrap().push_group(group);
            },

            _ => {
                stack.top_mut()
                    .expect("Should have errored already if the root was popped")
                    .push_token(token);
            },
        }
    }
    let result = stack.pop()
        .ok_or(ParseError::TooManyCloseBrackets)?;

    if stack.is_empty() {
        Ok(result)
    } else {
        Err(ParseError::NotEnoughCloseBrackets)
    }
}

fn group_operators<'doc>(tree: &mut SyntaxTree<'doc>) -> Result<(), ParseError> {
    // DFS
    for node in tree.0.iter_mut() {
        if let SyntaxNode::Group(group) = node {
            group_operators(group)?;
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
                        tree.0.insert(i + 1, SyntaxNode::PreOp { op: op_token, rhs });
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
            return Err(ParseError::OperatorMissingArguments { is_lhs_nonnull, op_token, is_rhs_nonnull });
        }
        break;
    }
    Ok(())
}

pub fn parse<'doc>(tokens: Vec<Token<'doc>>) -> Result<SyntaxTree<'doc>, ParseError> {
    let mut tree = group_tokens(tokens)?;
    group_operators(&mut tree)?;
    Ok(tree)
}
