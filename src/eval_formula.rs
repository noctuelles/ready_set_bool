/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   eval_formula.rs                                    :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: plouvel <plouvel@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/07/10 12:30:20 by plouvel           #+#    #+#             */
/*   Updated: 2024/07/10 16:37:01 by plouvel          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#[derive(Debug, PartialEq)]
enum Token {
    True,
    False,
    Not,
    And,
    Or,
    Xor,
    Impl,
    Equi,
}

fn char_to_token(c: char) -> Option<Token> {
    match c {
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

fn apply_operator(tkn: &Token,) -> bool {
    match tkn {
        Token::
    }
}

fn lex(formula: &str) -> Result<Vec<Token>, bool> {
    let mut tkns: Vec<Token> = Vec::new();

    for c in formula.chars() {
        match char_to_token(c) {
            Some(tkn) => tkns.push(tkn),
            None => return Err(false),
        };
    }
    Ok(tkns)
}

fn eval_formula(formula: &str) -> bool {
    let tkns = lex(formula);

    match tkns {
        Ok(tkns) => {
            println!("{}", tkns.len());
            true
        }
        Err(err) => err,
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_lex_wrong_formula() {
        assert!(lex("1&0p").is_err());
        assert!(lex("abc").is_err());
        assert!(lex("10&1|9").is_err());
    }

    #[test]
    fn test_lex_ok_formula() {
        assert_eq!(
            lex("10&1|").unwrap(),
            vec![
                Token::True,
                Token::False,
                Token::And,
                Token::True,
                Token::Or
            ]
        );
        assert_eq!(
            lex("1&0|1").unwrap(),
            vec![
                Token::True,
                Token::And,
                Token::False,
                Token::Or,
                Token::True
            ]
        );
        assert_eq!(
            lex("=>&|^!!").unwrap(),
            vec![
                Token::Equi,
                Token::Impl,
                Token::And,
                Token::Or,
                Token::Xor,
                Token::Not,
                Token::Not
            ]
        )
    }
}
