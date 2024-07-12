/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   truth_table.rs                                     :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: plouvel <plouvel@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/07/12 11:47:20 by plouvel           #+#    #+#             */
/*   Updated: 2024/07/12 14:16:26 by plouvel          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::bool_eval::ast::*;
use crate::bool_eval::lexer::*;

pub fn print_truth_table(formula: &str) {
    match lex_rpn(formula) {
        Ok(tkns) => match ASTBoolEval::try_from(tkns.as_ref()) {
            Ok(ast) => {
                ast.print_formula();
                ast.print_truth_table();
            }
            Err(msg) => println!("parse error: {msg}"),
        },
        Err(msg) => println!("lexing error: {msg}"),
    }
}
