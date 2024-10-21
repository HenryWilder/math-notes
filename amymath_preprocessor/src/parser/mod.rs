use crate::lexer::*;
use crate::stack::*;

/// Syntax tree for the parser.
pub mod syntax_tree;
/// `parser` error module.
pub mod error;

use syntax_tree::*;
use error::ParseError;

/// Groups [`GroupCtrlToken`]-delimited subexpressions
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
                let mut iter = group.0.into_iter();
                if let (
                    Some(SyntaxNode::Token(Token::GroupCtrl(GroupCtrlToken { kind: opened_with, ctrl: GroupControl::Open  }))),
                    Some(SyntaxNode::Token(Token::GroupCtrl(GroupCtrlToken { kind: closed_with, ctrl: GroupControl::Close }))),
                 ) = (iter.next(), iter.next_back()) {
                    if opened_with.is_compatible(&closed_with) {
                        stack.top_mut().unwrap()
                            .push_group(
                                opened_with,
                                iter.collect::<Vec<SyntaxNode>>(),
                                closed_with,
                            );
                    } else {
                        return Err(ParseError::BracketMismatch {
                            opened_with,
                            closed_with,
                        });
                    }
                } else {
                    panic!("Group should not be created without open & close delimiters");
                }
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

/// Groups operators with their arguments in-place
fn group_operators<'doc>(tree: &mut SyntaxTree<'doc>) -> Result<(), ParseError> {
    // DFS
    for node in tree.0.iter_mut() {
        if let SyntaxNode::Group{ inner, .. } = node {
            group_operators(inner)?; // Modify in place
        }
    }

    'operator_loop: loop {
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
            None => break 'operator_loop,
        };

        for (i, op_token) in operator_tokens {
            let bind_power = op_token.bind_power();
            if bind_power > current_target.1 {
                current_target = (i, bind_power);
            }
        }
        let (i, _) = current_target;

        if let SyntaxNode::Token(Token::Operator(op_token)) = tree.0[i] {
            'nary_loop: for (num_lhs, num_rhs) in op_token.nary() {
                assert!(!(num_lhs == 0 && num_rhs == 0), "operator must take argument(s)");
                let start = i.checked_sub(num_lhs);
                let end = i.checked_add(num_rhs);
                if let (Some(start), Some(end)) = (start, end) {
                    let lhs = &tree.0[start..i];
                    let rhs = &tree.0[(i+1)..=end];
                    if lhs.len() == num_lhs && lhs.len() == num_rhs {
                        tree.0.splice(start..=end, [
                            SyntaxNode::Operator {
                                lhs: lhs.to_vec(),
                                op: op_token,
                                rhs: rhs.to_vec(),
                            }
                        ]);
                        continue 'operator_loop;
                    } else {
                        continue 'nary_loop;
                    }
                } else {
                    continue 'nary_loop;
                }
            }

            // Reached end of `'nary_loop` without finding a match
            return Err(ParseError::OperatorMissingArguments {
                num_lhs: i,
                op_token,
                num_rhs: tree.0.len() - i - 1,
            });
        }
        break;
    }
    Ok(())
}

/// Apply clumping and lookaround to the document.
pub fn parse<'doc>(tokens: Vec<Token<'doc>>) -> Result<SyntaxTree<'doc>, ParseError> {
    let mut tree = group_subexpressions(tokens)?;
    group_operators(&mut tree)?;
    Ok(tree)
}
