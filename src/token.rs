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

use crate::token::Operator::*;
use crate::token::Token::*;
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