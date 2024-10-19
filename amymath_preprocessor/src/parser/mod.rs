use crate::lexer::*;
use crate::stack::*;

pub mod syntax_tree;
pub mod error;

use syntax_tree::*;
use error::ParseError;

fn group_subexpressions<'doc>(tokens: Vec<Token<'doc>>) -> Result<SyntaxTree<'doc>, ParseError> {
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
                if let [
                    SyntaxNode::Token(Token::GroupCtrl(GroupCtrlToken { kind: opened_with, ctrl: GroupControl::Open  })),
                    ..,
                    SyntaxNode::Token(Token::GroupCtrl(GroupCtrlToken { kind: closed_with, ctrl: GroupControl::Close })),
                ] = &group.0[..] {
                    if !opened_with.is_compatible(closed_with) {
                        return Err(ParseError::BracketMismatch {
                            opened_with: *opened_with,
                            closed_with: *closed_with,
                        });
                    }
                } else {
                    panic!("Group should not be created without open & close delimiters");
                }
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
            let lhs: Option<&SyntaxNode> =
                i.checked_sub(1)
                .and_then(|n| tree.0.get(n))
                .filter(|token| !matches!(token,
                    SyntaxNode::Token(Token::GroupCtrl(GroupCtrlToken { kind: _, ctrl: GroupControl::Open }))
                ));

            let rhs: Option<&SyntaxNode> =
                i.checked_add(1)
                .and_then(|n| tree.0.get(n))
                .filter(|token| !matches!(token,
                    SyntaxNode::Token(Token::GroupCtrl(GroupCtrlToken { kind: _, ctrl: GroupControl::Close }))
                ));

            let nary = op_token.nary();
            for argn in nary {
                if let Some((range, replacement)) = match (lhs, argn, rhs) {
                    (Some(lhs), NAry { n_before: 1, n_after: 1 }, Some(rhs)) => {
                        Some(((i - 1)..=(i + 1), SyntaxNode::BinOp {
                            lhs: Box::new(lhs.clone()),
                            op: op_token,
                            rhs: Box::new(rhs.clone()),
                        }))
                    },

                    (_, NAry { n_before: 0, n_after: 1 }, Some(rhs)) => {
                        Some((i..=(i + 1), SyntaxNode::PreOp {
                            op: op_token,
                            rhs: Box::new(rhs.clone()),
                        }))
                    },

                    (Some(lhs), NAry { n_before: 1, n_after: 0 }, _) => {
                        Some(((i - 1)..=i, SyntaxNode::SufOp {
                            lhs: Box::new(lhs.clone()),
                            op: op_token,
                        }))
                    },

                    _ => None,
                } {
                    tree.0.splice(range, [replacement].into_iter());
                    continue 'a;
                }
            }
            return Err(ParseError::OperatorMissingArguments {
                lhs_exists: lhs.is_some(),
                op_token,
                rhs_exists: rhs.is_some(),
            });
        }
        break;
    }
    Ok(())
}

pub fn parse<'doc>(tokens: Vec<Token<'doc>>) -> Result<SyntaxTree<'doc>, ParseError> {
    let mut tree = group_subexpressions(tokens)?;
    group_operators(&mut tree)?;
    Ok(tree)
}
