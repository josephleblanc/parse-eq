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

use std::error::Error;


#[derive(Debug, Copy, Clone)]
/// Tokens are the first internal representation of the input string.
pub(crate) enum Token {
    // Left paren
    LParen,
    // Right paren
    RParen,
    // Operators, e.g. +, -, /
    Op(Operator),
    // Numbers, e.g. 1.23, 2800000.0, e, pi
    Number(f32),
    // Variables, e.g. x, y, z
    Var(Variable)
}

impl Token {

/// Takes input string and returns tokens.
    fn lexer(s: &str) -> Result< Vec<Token>, Box<dyn Error>> {
        // TODO: Convert string (or whatever input type we want) into Vec of tokens.
        // e.g. 
        // "(3 * 2.0 + x) / 8" => [
        //                          LParen, 
        //                          Number(3.0), 
        //                          Op(Multiply), 
        //                          Number(2.0), 
        //                          Op(Plus),
        //                          Var(Variable::X)
        //                          RParen, 
        //                          Op(Divide),
        //                          Number(8.0)
        //                      ]
        // Even better would be returning prefix notation, as it is easier to build into a tree:
        // "(3 * 2.0 + x) / 8" => [
        //                          Divide,
        //                          LParen, 
        //                          Multiply, 
        //                          Number(3.0), 
        //                          Number(2.0), 
        //                          Plus,
        //                          Var(Variable::X)
        //                          RParen, 
        //                          Number(8.0)
        //                      ]
        todo!()
    }
    // TODO: Add error types for token that carry more information about the type of error
    // encountered, e.g. "Divide by zero", "Operator not followed by a number or variable", etc.
}

#[derive(Debug, Copy, Clone)]
// This is placeholder for now - we could change the approach to something else,
// e.g. We could get rid of this type and just have Token::Var(u8), where Var(0) is the first
// variable (maybe x), and Var(1) is the second variable in the expression "( 2x - y ) / 4x"
pub(crate) enum Variable {
    X,
    Y,
    Z,
    // more here
}

#[derive(Debug, Copy, Clone)]
// An enum for the different operator types our parser can handle.
// This enum is subject to change, as it may be better to have the operators split into binary
// operators (e.g. Multiply, Divide) and unary (e.g. Sine, Cosine).
pub(crate) enum Operator{
    Multiply,
    Divide,
    Add,
    Subtract,
    Sine,
    Cosine,
    Tangent,
    ArcSine,
    ArcCosine,
    ArcTangent,
    Exponent,
    Logarithm,
    // more here
}
