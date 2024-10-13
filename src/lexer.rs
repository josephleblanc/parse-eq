use crate::token::Operator::*;
use crate::token::Token;
use crate::token::Token::*;
use crate::token::UnaryOperator;
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
    pub fn new_inorder(s: &str) -> Result<Self, Box<dyn Error>> {
        // Note: Each character which is processed into a struct (e.g. '+', 'x', 'y'), must be
        // listed among the split chars here.
        let split_chars = ['+', '-', '/', '*', '(', ')', 'x', 'y', 'z'];
        let mut mid_split = s
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
                        "y" => Some(Var(Variable::Y)),
                        "z" => Some(Var(Variable::Z)),
                        "sin" => Some(UnOp(UnaryOperator::Sine)),
                        "cos" => Some(UnOp(UnaryOperator::Cosine)),
                        "tan" => Some(UnOp(UnaryOperator::Tangent)),
                        _ => None,
                    }
                }
            })
            .enumerate()
            .peekable();
        let mut list: Vec<Token> = vec![];
        while let Some((i, mut token)) = mid_split.next() {
            if let Some((_, peeked)) = mid_split.peek() {
                // Turn subtraction '-' to negation if first token and the next token is a valid
                // target for negation.
                if i == 0 && token == Op(Subtract)
                    || (matches!(peeked, LParen)
                        && matches!(peeked, Var(_))
                        && matches!(peeked, UnOp(_))
                        && matches!(peeked, Number(_)))
                {
                    token = UnOp(UnaryOperator::Negation);
                } else if *peeked == Op(Subtract)
                    && (matches!(token, Op(_)) || matches!(token, UnOp(_)))
                {
                    // Turn subtraction '-' to negation if it immediately follows a regular
                    // (binary) operation.
                    list.push(token);
                    mid_split.next();
                    token = UnOp(UnaryOperator::Negation);
                }
            }
            list.push(token);
        }
        //let list: Vec<Token> = mid_split.collect();
        Ok(Lexer {
            list,
            ordering: Ordering::In,
        })
    }

    /// Consuming function that creates a pre-order list of tokens from an in-order list of tokens.
    /// Currently of limited use as it does not handle parentheses.
    // TODO: Decide whether to scrap this function or not. It would be more usefully handled by
    // the tree struct in tree.rs.
    ///
    /// e.g.
    /// "3 * 2.0 + x"
    /// => [
    ///     Number(3.0),
    ///     Op(Multiply),
    ///     Number(2.0),
    ///     Op(Plus),
    ///     Var(Variable::X)
    /// ]
    /// becomes
    /// [
    ///     Op(Multiply),
    ///     Number(3.0),
    ///     Op(Plus),
    ///     Number(2.0),
    ///     Var(Variable::X)
    /// ]
    pub fn in_to_pre(&mut self) {
        if self.ordering != Ordering::In {
            panic!("The ordering must be in-order to use in_to_pre to change ordering");
        }
        //let mut in_order: Vec<Token> = self.list.into_iter().rev().collect();
        let mut pre_order: Vec<Token> = vec![];
        let mut stack: Vec<Token> = vec![];
        for token in self.list.iter() {
            match token {
                Op(op) => {
                    pre_order.push(Token::Op(*op));
                    while let Some(num_var) = stack.pop() {
                        pre_order.push(num_var);
                    }
                }
                UnOp(un_op) => {
                    pre_order.push(Token::UnOp(*un_op));
                    pre_order.push(stack.pop().unwrap());
                }
                Number(n) => stack.push(Token::Number(*n)),
                Var(v) => stack.push(Token::Var(*v)),
                LParen => (),
                RParen => (),
            }
        }
        if let Some(last_token) = stack.pop() {
            pre_order.push(last_token);
        }

        self.list = pre_order;
        self.ordering = Ordering::Pre;
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
