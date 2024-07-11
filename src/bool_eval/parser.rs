/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   parser.rs                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: plouvel <plouvel@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/07/11 17:59:19 by plouvel           #+#    #+#             */
/*   Updated: 2024/07/11 18:16:50 by plouvel          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::lexer::*;
use super::*;
use crate::binary_tree::BTreeNode;

pub fn parse_ast(tkns: Vec<Token>) -> Result<ASTBoolEval, String> {
    let mut output: Vec<ASTBoolEval> = Vec::new();

    for tkn in tkns {
        match tkn {
            /* Operands */
            Token::True | Token::False => output.push(BTreeNode {
                value: BoolEvalNodeValue::Operand(Operand::Immediate(true)),
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
            Token::And | Token::Or | Token::Xor | Token::Equi | Token::Impl => {
                if let (Some(a), Some(b)) = (output.pop(), output.pop()) {
                    output.push(BTreeNode {
                        value: tkn,
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
    Ok(output.pop().unwrap())
}
