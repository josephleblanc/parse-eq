// This is where we parse the string into a vec of tokens.
//
// The goal is to take a string and return a vec which may be iterated over to create an iterator of
// Stmt, a type we define in ast.rs.
// An example of how this should be used:
//
// let input_string = "(1 + 13x)/2";
// tokens: Vec<Token> = Token::lexer(input_string);
// expr: Vec<Stmt> = Stmt::from_tokens(tokens);
//
// Once we have a way to turn the string into a Vec<Token>, and a way to turn Vec<Token> into
// Vec<Stmt>, we can implement Iterator for Token and Expr, as iterators are probably a better way
// to go about this.

use crate::lexer::Operator::*;
use crate::lexer::Token::*;
use std::error::Error;

#[derive(Debug, Copy, Clone, PartialEq)]
/// Tokens are the first internal representation of the input string.
pub enum Token {
    // Left paren
    LParen,
    // Right paren
    RParen,
    // Operators, e.g. +, -, /
    Op(Operator),
    // Numbers, e.g. 1.23, 2800000.0, e, pi
    Number(f32),
    // Variables, e.g. x, y, z
    Var(Variable),
}

impl TryFrom<Token> for f32 {
    type Error = &'static str;
    fn try_from(token: Token) -> Result<Self, Self::Error> {
        match token {
            Token::Number(float) => Ok(float),
            _ => Err("Invalid token cannot be parsed into a float."),
        }
    }
}
impl TryFrom<&Token> for f32 {
    type Error = &'static str;
    fn try_from(token: &Token) -> Result<Self, Self::Error> {
        match token {
            Token::Number(float) => Ok(*float),
            _ => Err("Invalid token cannot be parsed into a float."),
        }
    }
}

impl Token {
    pub fn is_op(&self) -> bool {
        matches!(self, Op(_))
    }

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
    pub fn lexer(s: &str) -> Result<Vec<Token>, Box<dyn Error>> {
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

#[derive(Debug, Copy, Clone, PartialEq)]
// This is placeholder for now - we could change the approach to something else,
// e.g. We could get rid of this type and just have Token::Var(u8), where Var(0) is the first
// variable (maybe x), and Var(1) is the second variable in the expression "( 2x - y ) / 4x"
pub enum Variable {
    X,
    Y,
    Z,
    // more here
}

#[derive(Debug, Copy, Clone, PartialEq)]
// An enum for the different operator types our parser can handle.
// This enum is subject to change, as it may be better to have the operators split into binary
// operators (e.g. Multiply, Divide) and unary (e.g. Sine, Cosine).
pub enum Operator {
    Multiply,
    Divide,
    Add,
    Subtract,
    //Sine,
    //Cosine,
    //Tangent,
    //ArcSine,
    //ArcCosine,
    //ArcTangent,
    //Exponent,
    //Logarithm,
    // more here
}

impl Operator {
    pub fn precedence(&self) -> Precedence {
        match self {
            Multiply => Precedence {
                precedence: 2,
                is_left: false,
            },
            Divide => Precedence {
                precedence: 2,
                is_left: true,
            },
            Add => Precedence {
                precedence: 1,
                is_left: false,
            },
            Subtract => Precedence {
                precedence: 1,
                is_left: true,
            },
            //Sine => Precedence { precedence: 0, is_left: false },
            //Cosine => Precedence { precedence: 0, is_left: false },
            //Tangent => Precedence { precedence: 0, is_left: false },
            //ArcSine => Precedence { precedence: 0, is_left: false },
            //ArcCosine => Precedence { precedence: 0, is_left: false },
            //ArcTangent => Precedence { precedence: 0, is_left: false },
            //Exponent => Precedence { precedence: 0, is_left: false },
            //Logarithm => Precedence { precedence: 0, is_left: false },
        }
    }
}

pub struct Precedence {
    precedence: u8,
    is_left: bool,
}
