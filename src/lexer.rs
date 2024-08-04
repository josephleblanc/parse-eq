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
        let split_chars = [
            '+', '-', '/', '*', '(', ')'
        ];
        let mut mid_split = s.split_whitespace()
            .flat_map(|split| split.split_inclusive(split_chars))
            .flat_map(split_nums);
        todo!()
    }
    // TODO: Add error types for token that carry more information about the type of error
    // encountered, e.g. "Divide by zero", "Operator not followed by a number or variable", etc.
}

// Split numbers from variables, e.g. 132x becomes ['132', 'x'], or (132) becomes ['(', '132', ')']
// Helper function used in Token::lexer
// note: it may be better to handle this functionality through the split_chars variable in 
// Token::lexer, depending on how many variables we should account for before hand.
fn split_nums(s: &str) -> Result< Vec< &str >, Box<dyn Error>> {
    let mut split_indicies: Vec<usize> = vec![];
    if s.len() > 1 {
        let s_iter = s.chars().zip(s.chars().skip(1)).enumerate();
        for (i, ( current, next) ) in s_iter {
            match (current.is_numeric(), next.is_numeric()) {
                (false, true) => split_indicies.push(i),
                (true, false) => split_indicies.push(i),
                _ => ()
            }
        }
        // TODO: make the split in middle through .split_at() recursive
        let mut splits_vec: Vec<&str> = vec![];
        splits_vec.push(s.split_at(split_indicies[0]).0);
        for split_i in &split_indicies[..] {
            splits_vec.push(s.split_at(split_indicies[*split_i]).1);
        }
        Ok(splits_vec)

    } else if s.len() <= 1 { 
        return Ok( vec![s] )
    } else {
        return Err("Malformed input string for split_nums. Input should be numbers or variables".into());
    }
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
