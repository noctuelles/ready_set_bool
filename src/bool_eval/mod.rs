/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   mod.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: plouvel <plouvel@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/07/11 17:17:20 by plouvel           #+#    #+#             */
/*   Updated: 2024/07/11 18:14:45 by plouvel          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

mod lexer;
mod parser;

use crate::binary_tree::BTreeNode;

pub enum Operand {
    Immediate(bool),
    Variable(bool),
}

pub enum Operator {
    And,
    Or,
    Xor,
    Not,
    Impl,
    Equi,
}

pub enum BoolEvalNodeValue {
    Operand(Operand),
    Operator(Operator),
}

pub type ASTBoolEval = BTreeNode<BoolEvalNodeValue>;
