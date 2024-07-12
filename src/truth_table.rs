/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   truth_table.rs                                     :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: plouvel <plouvel@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/07/12 11:47:20 by plouvel           #+#    #+#             */
/*   Updated: 2024/07/12 17:51:07 by plouvel          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::bool_eval::ast::*;
use crate::bool_eval::lexer::*;

pub fn print_truth_table(formula: &str) {
    match lex_rpn(formula) {
        Ok(tkns) => match ASTBoolEval::try_from(tkns.as_ref()) {
            Ok(ast) => {
                println!(
                    "Reverse Polish Notation representation : {}",
                    ast.get_rpn_repr()
                );
                println!("Infix Notation representation : {}", ast.get_infix_repr());
                println!("Truth table:");
                ast.print_truth_table();
            }
            Err(msg) => println!("parse error: {msg}"),
        },
        Err(msg) => println!("lexing error: {msg}"),
    }
}
