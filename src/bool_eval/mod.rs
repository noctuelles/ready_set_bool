/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   mod.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: plouvel <plouvel@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/07/11 17:17:20 by plouvel           #+#    #+#             */
/*   Updated: 2024/07/11 19:23:14 by plouvel          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

mod lexer;
mod parser;

use crate::binary_tree::BTreeNode;

pub enum Operand<'a> {
    Immediate(bool),
    Variable(&'a bool),
}

pub enum Operator {
    And,
    Or,
    Xor,
    Not,
    Impl,
    Equi,
}

pub enum BoolEvalNodeValue<'a> {
    Operand(Operand<'a>),
    Operator(Operator),
}

pub type ASTBoolEval<'a> = BTreeNode<BoolEvalNodeValue<'a>>;

impl<'a> BTreeNode<BoolEvalNodeValue<'a>> {
    pub fn new(value: BoolEvalNodeValue<'a>) -> Self {
        BTreeNode::<BoolEvalNodeValue<'a>> {
            value,
            left: None,
            right: None,
        }
    }

    pub fn eval(&self) -> bool {
        match &self.value {
            BoolEvalNodeValue::Operand(Operand::Immediate(b)) => *b,
            BoolEvalNodeValue::Operand(Operand::Variable(b)) => **b,
            BoolEvalNodeValue::Operator(Operator::And) => {
                self.left.as_ref().unwrap().eval() && self.right.as_ref().unwrap().eval()
            }
            BoolEvalNodeValue::Operator(Operator::Or) => {
                self.left.as_ref().unwrap().eval() || self.right.as_ref().unwrap().eval()
            }
            BoolEvalNodeValue::Operator(Operator::Xor) => {
                self.left.as_ref().unwrap().eval() ^ self.right.as_ref().unwrap().eval()
            }
            BoolEvalNodeValue::Operator(Operator::Not) => !self.right.as_ref().unwrap().eval(),
            BoolEvalNodeValue::Operator(Operator::Impl) => {
                !self.left.as_ref().unwrap().eval() || self.right.as_ref().unwrap().eval()
            }
            BoolEvalNodeValue::Operator(Operator::Equi) => {
                self.left.as_ref().unwrap().eval() == self.right.as_ref().unwrap().eval()
            }
        }
    }
}
