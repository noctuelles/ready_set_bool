/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   eval_formula.rs                                    :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: plouvel <plouvel@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/07/10 12:30:20 by plouvel           #+#    #+#             */
/*   Updated: 2024/07/11 18:00:36 by plouvel          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

// use crate::binary_tree::BTreeNode;
// use std::fmt::{self};

// fn parse(tkns: Vec<Token>) -> Result<BTreeNode<Token>, String> {
//     let mut output: Vec<BTreeNode<Token>> = Vec::new();

//     for tkn in tkns {
//         match tkn {
//             /* Operands */
//             Token::True | Token::False => output.push(BTreeNode {
//                 value: tkn,
//                 right: None,
//                 left: None,
//             }),
//             /* Operators */
//             Token::Not => {
//                 if let Some(a) = output.pop() {
//                     output.push(BTreeNode {
//                         value: tkn,
//                         right: Some(Box::new(a)),
//                         left: None,
//                     })
//                 } else {
//                     return Err(format!("{} requires one operand", tkn.to_string()));
//                 }
//             }
//             Token::And | Token::Or | Token::Xor | Token::Equi | Token::Impl => {
//                 if let (Some(a), Some(b)) = (output.pop(), output.pop()) {
//                     output.push(BTreeNode {
//                         value: tkn,
//                         right: Some(Box::new(a)),
//                         left: Some(Box::new(b)),
//                     })
//                 } else {
//                     return Err(format!("{} requires two operands", tkn.to_string()));
//                 }
//             }
//         }
//     }
//     if output.len() != 1 {
//         return Err(String::from("trailing operands"));
//     }
//     Ok(output.pop().unwrap())
// }

// fn eval(node: &BTreeNode<Token>) -> bool {
//     let mut p = false;
//     let mut q = false;

//     if let Some(node) = node.right.as_ref() {
//         p = eval(node);
//     }
//     if let Some(node) = node.left.as_ref() {
//         q = eval(node);
//     }

//     match node.value {
//         Token::True => true,
//         Token::False => false,
//         Token::Not => !p,
//         Token::Or => p | q,
//         Token::And => p & q,
//         Token::Xor => p ^ q,
//         Token::Equi => p == q,
//         Token::Impl => !p | q,
//     }
// }

// pub fn eval_formula(formula: &str) -> bool {
//     match lex(formula) {
//         Ok(tkns) => match parse(tkns) {
//             Ok(ast_root) => eval(&ast_root),
//             Err(message) => {
//                 println!("an error occured during parsing : {}.", message);
//                 false
//             }
//         },
//         Err(message) => {
//             println!("an error occured during lexing : {}.", message);
//             false
//         }
//     }
// }

// mod tests {
//     use super::*;

//     #[test]
//     fn test_lex_wrong_formula() {
//         assert!(lex("1&0p").is_err());
//         assert!(lex("abc").is_err());
//         assert!(lex("10&1|9").is_err());
//     }

//     #[test]
//     fn test_lex_ok_formula() {
//         assert_eq!(
//             lex("10&1|").unwrap(),
//             vec![
//                 Token::True,
//                 Token::False,
//                 Token::And,
//                 Token::True,
//                 Token::Or
//             ]
//         );
//         assert_eq!(
//             lex("1&0|1").unwrap(),
//             vec![
//                 Token::True,
//                 Token::And,
//                 Token::False,
//                 Token::Or,
//                 Token::True
//             ]
//         );
//         assert_eq!(
//             lex("=>&|^!!").unwrap(),
//             vec![
//                 Token::Equi,
//                 Token::Impl,
//                 Token::And,
//                 Token::Or,
//                 Token::Xor,
//                 Token::Not,
//                 Token::Not
//             ]
//         )
//     }
// }
