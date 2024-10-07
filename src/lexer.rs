use crate::token::Operator::*;
use crate::token::Token;
use crate::token::Token::*;
use crate::token::Variable;
use std::error::Error;

pub struct Lexer {
    pub list: Vec<Token>,
    pub ordering: Ordering,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Ordering {
    In,
    Pre,
    Post,
}

impl Lexer {
    /// Takes input string and returns tokens.
    /// e.g.
    /// "(3 * 2.0 + x) / 8" => [
    ///                          LParen,
    ///                          Number(3.0),
    ///                          Op(Multiply),
    ///                          Number(2.0),
    ///                          Op(Plus),
    ///                          Var(Variable::X)
    ///                          RParen,
    ///                          Op(Divide),
    ///                          Number(8.0)
    ///                      ]
    pub fn new_inorder(s: &str) -> Result<Vec<Token>, Box<dyn Error>> {
        let split_chars = ['+', '-', '/', '*', '(', ')'];
        let mut mid_split: Vec<Token> = s
            .split_whitespace()
            .flat_map(|split| split.split_inclusive(split_chars))
            .flat_map(split_nums)
            .flatten()
            .filter_map(|split| {
                if split.chars().nth(0)?.is_numeric() {
                    Some(Number(
                        split.parse::<f32>().expect("Could not parse number"),
                    ))
                } else {
                    match split {
                        "(" => Some(LParen),
                        ")" => Some(RParen),
                        "+" => Some(Op(Add)),
                        "-" => Some(Op(Subtract)),
                        "*" => Some(Op(Multiply)),
                        "/" => Some(Op(Divide)),
                        "x" => Some(Var(Variable::X)),
                        _ => None,
                    }
                }
            })
            .collect();
        Ok(mid_split)
    }
    // TODO: Add error types for token that carry more information about the type of error
    // encountered, e.g. "Divide by zero", "Operator not followed by a number or variable", etc.
}

// Split numbers from variables, e.g. 132x becomes ['132', 'x'], or (132) becomes ['(', '132', ')']
// Helper function used in Token::lexer
// note: it may be better to handle this functionality through the split_chars variable in
// Token::lexer, depending on how many variables we should account for before hand.
pub fn split_nums(s: &str) -> Result<Vec<&str>, Box<dyn Error>> {
    let mut split_indicies: Vec<usize> = vec![];
    if s.len() > 1 {
        let s_iter = s.chars().zip(s.chars().skip(1)).enumerate();
        for (i, (current, next)) in s_iter {
            match (
                current.is_numeric() || current == '.',
                next.is_numeric() || next == '.',
            ) {
                (false, true) => split_indicies.push(i),
                (true, false) => split_indicies.push(i + 1),
                _ => (),
            }
        }
        // TODO: try making the split in middle through .split_at() recursive
        let mut splits_vec: Vec<&str> = vec![];
        if !split_indicies.is_empty() {
            // note: split_at starts at 1 while indexing starts at 0, that is why +1 below
            splits_vec.push(s.split_at(split_indicies[0]).0);
            for split_i in &split_indicies[..] {
                let second_split = s.split_at(*split_i).1;
                if !second_split.is_empty() {
                    splits_vec.push(second_split);
                }
            }
        } else {
            splits_vec.push(s);
        }
        Ok(splits_vec)
    } else if s.len() <= 1 {
        return Ok(vec![s]);
    } else {
        return Err(
            "Malformed input string for split_nums. Input should be numbers or variables".into(),
        );
    }
}
