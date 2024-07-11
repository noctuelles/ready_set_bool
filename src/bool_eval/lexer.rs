/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lexer.rs                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: plouvel <plouvel@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/07/11 17:42:21 by plouvel           #+#    #+#             */
/*   Updated: 2024/07/11 18:03:20 by plouvel          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#[derive(Debug, PartialEq)]
pub enum Token {
    Letter(char),
    True,
    False,
    Not,
    And,
    Or,
    Xor,
    Impl,
    Equi,
}

fn c_to_tkn(c: char) -> Option<Token> {
    match c {
        'A'..='Z' => Some(Token::Letter(c)),
        '1' => Some(Token::True),
        '0' => Some(Token::False),
        '!' => Some(Token::Not),
        '&' => Some(Token::And),
        '|' => Some(Token::Or),
        '^' => Some(Token::Xor),
        '>' => Some(Token::Impl),
        '=' => Some(Token::Equi),
        _ => None,
    }
}

pub fn lex_rpn(rpn_formula: &str) -> Result<Vec<Token>, String> {
    let mut tkns = Vec::new();

    for c in rpn_formula.chars() {
        if c.is_whitespace() {
            continue;
        }
        if let Some(tkn) = c_to_tkn(c) {
            tkns.push(tkn);
        } else {
            return Err(format!("'{c}' is not a valid character"));
        }
    }
    Ok(tkns)
}

mod tests {
    mod lex_rpn {

        #[test]
        fn valid() {
            use super::super::*;

            assert_eq!(
                lex_rpn("10&"),
                Ok(vec![Token::True, Token::False, Token::And])
            );
            assert_eq!(
                lex_rpn("10|"),
                Ok(vec![Token::True, Token::False, Token::Or])
            );
            assert_eq!(
                lex_rpn("11>"),
                Ok(vec![Token::True, Token::True, Token::Impl])
            );
            assert_eq!(
                lex_rpn("10="),
                Ok(vec![Token::True, Token::False, Token::Equi])
            );
            assert_eq!(
                lex_rpn("1 0 1 1  \t | | ="),
                Ok(vec![
                    Token::True,
                    Token::False,
                    Token::True,
                    Token::True,
                    Token::Or,
                    Token::Or,
                    Token::Equi
                ])
            );
            assert_eq!(
                lex_rpn("AB&C|"),
                Ok(vec![
                    Token::Letter('A'),
                    Token::Letter('B'),
                    Token::And,
                    Token::Letter('C'),
                    Token::Or
                ])
            )
        }

        #[test]
        fn invalid() {
            use super::super::*;

            assert!(lex_rpn("11&0/").is_err());
            assert!(lex_rpn("ab&").is_err());
            assert!(lex_rpn("AB&C!|d").is_err());
        }
    }
}
