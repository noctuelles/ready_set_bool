/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   ast.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: plouvel <plouvel@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/07/12 11:50:27 by plouvel           #+#    #+#             */
/*   Updated: 2024/07/12 14:18:37 by plouvel          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::{cell::Cell, collections::BTreeMap, rc::Rc};

use crate::binary_tree::BTreeNode;
use crate::bool_eval::lexer::Token;

#[derive(Debug)]
pub enum Operand {
    Immediate(bool),
    Variable(char, Rc<Cell<bool>>),
}

#[derive(Debug)]
pub enum Operator {
    And,
    Or,
    Xor,
    Not,
    Impl,
    Equi,
}

#[derive(Debug)]
pub enum BoolEvalNodeValue {
    Operand(Operand),
    Operator(Operator),
}

type BoolEvalNode = BTreeNode<BoolEvalNodeValue>;
type BoolEvalVars = BTreeMap<char, Rc<Cell<bool>>>;

pub struct ASTBoolEval {
    root: BoolEvalNode,
    variables: BoolEvalVars,
}

impl TryFrom<&[Token]> for ASTBoolEval {
    type Error = String;

    fn try_from(tkns: &[Token]) -> Result<Self, Self::Error> {
        let mut output = Vec::new();
        let mut variables = BTreeMap::new();

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
                        *c,
                        variables
                            .entry(*c)
                            .or_insert(Rc::new(Cell::new(false)))
                            .clone(),
                    )),
                    right: None,
                    left: None,
                }),
                /* Two Operand Operators */
                Token::And | Token::Or | Token::Xor | Token::Equi | Token::Impl => {
                    if let (Some(a), Some(b)) = (output.pop(), output.pop()) {
                        output.push(BTreeNode {
                            value: match tkn {
                                Token::And => BoolEvalNodeValue::Operator(Operator::And),
                                Token::Or => BoolEvalNodeValue::Operator(Operator::Or),
                                Token::Xor => BoolEvalNodeValue::Operator(Operator::Xor),
                                Token::Equi => BoolEvalNodeValue::Operator(Operator::Equi),
                                Token::Impl => BoolEvalNodeValue::Operator(Operator::Impl),
                                _ => unreachable!(),
                            },
                            right: Some(Box::new(a)),
                            left: Some(Box::new(b)),
                        })
                    } else {
                        return Err(format!("{} requires two operands", tkn.to_string()));
                    }
                }
                Token::Not => {
                    if let Some(a) = output.pop() {
                        output.push(BTreeNode {
                            value: BoolEvalNodeValue::Operator(Operator::Not),
                            right: Some(Box::new(a)),
                            left: None,
                        })
                    } else {
                        return Err(format!("{} requires one operand", tkn.to_string()));
                    }
                }
            }
        }
        if output.len() != 1 {
            return Err("trailing operands".to_string());
        }
        Ok(Self::new(output.pop().unwrap(), variables))
    }
}

impl ASTBoolEval {
    pub fn new(root: BoolEvalNode, variables: BTreeMap<char, Rc<Cell<bool>>>) -> Self {
        Self { root, variables }
    }

    pub fn print_formula(&self) {
        println!();
        self.root.print_formula();
        println!();
    }

    pub fn print_truth_table(&self) {
        if self.variables.is_empty() {
            return;
        }

        let saved_values = self
            .variables
            .iter()
            .map(|(_, v)| v.get())
            .collect::<Vec<bool>>();
        println!();
        for c in self.variables.keys() {
            print!("| {c} ");
        }
        println!("| = |");
        for _ in self.variables.keys() {
            print!("|---");
        }
        println!("|---|");
        let nperms = 2usize.pow(self.variables.len() as u32);
        for i in 0..nperms {
            for (j, val) in self.variables.values().enumerate() {
                val.set(i >> j & 1 == 1);
                print!("| {} ", val.get() as u8);
            }
            println!("| {} |", self.eval() as u8);
        }
        for (i, val) in self.variables.values().enumerate() {
            val.set(saved_values[i]);
        }
    }

    pub fn eval(&self) -> bool {
        self.root.eval()
    }
}

impl BoolEvalNode {
    pub fn print_formula(&self) {
        match &self.value {
            BoolEvalNodeValue::Operand(Operand::Immediate(b)) => {
                if *b {
                    print!("⊤");
                } else {
                    print!("⊥");
                }
            }
            BoolEvalNodeValue::Operand(Operand::Variable(c, _)) => print!("{}", *c),
            BoolEvalNodeValue::Operator(Operator::And) => {
                print!("(");
                self.left.as_ref().unwrap().print_formula();
                print!(" ⋀ ");
                self.right.as_ref().unwrap().print_formula();
                print!(")");
            }
            BoolEvalNodeValue::Operator(Operator::Or) => {
                print!("(");
                self.left.as_ref().unwrap().print_formula();
                print!(" ⋁ ");
                self.right.as_ref().unwrap().print_formula();
                print!(")");
            }
            BoolEvalNodeValue::Operator(Operator::Xor) => {
                print!("(");
                self.left.as_ref().unwrap().print_formula();
                print!(" ⊕ ");
                self.right.as_ref().unwrap().print_formula();
                print!(")");
            }
            BoolEvalNodeValue::Operator(Operator::Not) => {
                print!("¬");
                print!("(");
                self.right.as_ref().unwrap().print_formula();
                print!(")");
            }
            BoolEvalNodeValue::Operator(Operator::Impl) => {
                print!("(");
                self.left.as_ref().unwrap().print_formula();
                print!(" ⇒ ");
                self.right.as_ref().unwrap().print_formula();
                print!(")");
            }
            BoolEvalNodeValue::Operator(Operator::Equi) => {
                print!("(");
                self.left.as_ref().unwrap().print_formula();
                print!(" ⇔ ");
                self.right.as_ref().unwrap().print_formula();
                print!(")");
            }
        }
    }

    pub fn eval(&self) -> bool {
        match &self.value {
            BoolEvalNodeValue::Operand(Operand::Immediate(b)) => b.clone(),
            BoolEvalNodeValue::Operand(Operand::Variable(_, b)) => b.get(),
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
