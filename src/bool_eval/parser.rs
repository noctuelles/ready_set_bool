/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   parser.rs                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: plouvel <plouvel@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/07/11 17:59:19 by plouvel           #+#    #+#             */
/*   Updated: 2024/07/11 19:21:29 by plouvel          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::collections::{HashMap, HashSet};

use super::lexer::*;
use super::*;
use crate::binary_tree::BTreeNode;

pub fn parse_ast(tkns: Vec<Token>) -> Result<(ASTBoolEval, HashMap<char, bool>), String> {
    let mut output: Vec<ASTBoolEval> = Vec::new();
    let letters: HashMap<char, bool> = HashMap::new();

    for tkn in tkns {
        match tkn {
            /* Operands */
            Token::True => output.push(BTreeNode {
                value: BoolEvalNodeValue::Operand(Operand::Immediate(true)),
                right: None,
                left: None,
            }),
            Token::False => output.push(BTreeNode {
                value: BoolEvalNodeValue::Operand(Operand::Immediate(false)),
                right: None,
                left: None,
            }),
            Token::Letter(c) => output.push(BTreeNode {
                value: BoolEvalNodeValue::Operand(Operand::Variable(
                    letters.entry(c).or_insert(false),
                )),
                right: None,
                left: None,
            }),
            /* Operators */
            Token::Not => {
                if let Some(a) = output.pop() {
                    output.push(BTreeNode {
                        value: tkn,
                        right: Some(Box::new(a)),
                        left: None,
                    })
                } else {
                    return Err(format!("{} requires one operand", tkn.to_string()));
                }
            }
            Token::And => {
                if let (Some(a), Some(b)) = (output.pop(), output.pop()) {
                    output.push(BTreeNode {
                        value: BoolEvalNodeValue::Operator(Operator::And),
                        right: Some(Box::new(a)),
                        left: Some(Box::new(b)),
                    })
                } else {
                    return Err(format!("{} requires two operands", tkn.to_string()));
                }
            }
            Token::Or => {
                if let (Some(a), Some(b)) = (output.pop(), output.pop()) {
                    output.push(BTreeNode {
                        value: BoolEvalNodeValue::Operator(Operator::Or),
                        right: Some(Box::new(a)),
                        left: Some(Box::new(b)),
                    })
                } else {
                    return Err(format!("{} requires two operands", tkn.to_string()));
                }
            }
            Token::Xor => {
                if let (Some(a), Some(b)) = (output.pop(), output.pop()) {
                    output.push(BTreeNode {
                        value: BoolEvalNodeValue::Operator(Operator::Xor),
                        right: Some(Box::new(a)),
                        left: Some(Box::new(b)),
                    })
                } else {
                    return Err(format!("{} requires two operands", tkn.to_string()));
                }
            }
            Token::Impl => {
                if let (Some(a), Some(b)) = (output.pop(), output.pop()) {
                    output.push(BTreeNode {
                        value: BoolEvalNodeValue::Operator(Operator::Impl),
                        right: Some(Box::new(a)),
                        left: Some(Box::new(b)),
                    })
                } else {
                    return Err(format!("{} requires two operands", tkn.to_string()));
                }
            }
            Token::Equi => {
                if let (Some(a), Some(b)) = (output.pop(), output.pop()) {
                    output.push(BTreeNode {
                        value: BoolEvalNodeValue::Operator(Operator::Equi),
                        right: Some(Box::new(a)),
                        left: Some(Box::new(b)),
                    })
                } else {
                    return Err(format!("{} requires two operands", tkn.to_string()));
                }
            }
        }
    }
    if output.len() != 1 {
        return Err(String::from("trailing operands"));
    }
    Ok((output.pop().unwrap(), letters))
}

mod tests {
    mod parse_ast {
        use super::super::*;

        #[test]
        fn valid() {
            assert_eq!(
                parse_ast(vec![Token::Letter('A'), Token::Letter('B'), Token::And]),
                Ok(BTreeNode {
                    value: BoolEvalNodeValue::Operator(Operator::And),
                    right: Some(Box::new(BTreeNode {
                        value: BoolEvalNodeValue::Operand(Operand::Immediate(false)),
                        right: None,
                        left: None,
                    })),
                    left: Some(Box::new(BTreeNode {
                        value: BoolEvalNodeValue::Operand(Operand::Immediate(true)),
                        right: None,
                        left: None,
                    })),
                })
            );
        }
    }
}
