// Visitor Pattern code from:
// https://rust-unofficial.github.io/patterns/patterns/behavioural/visitor.html?highlight=ref#discussion
// another example:
// https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=1e93173b55bdad5e2908e32823611202

use crate::lexer::Token;

pub struct Expr(Vec<Token>);

impl Expr {
    /// Evaluates a statement from tokens. This version relies on post-fix (reverse polish) notation
    // We may wish to change this to prefix at some point.
    pub fn eval(&self) -> () /* Stmt::Expr */ {
        let mut stack: Vec<f32> = vec![];
        // TODO: make this work
        // This doesn't actually work - it needs to have a while loop.
        for token in self.0.iter() {
            match token {
                // TODO: Cover all cases of operator
                Token::Number(n) => stack.push(*n),
                Token::Op(crate::lexer::Operator::Multiply) => {
                    let new = stack.pop().unwrap() * stack.pop().unwrap();
                    stack.push(new)
                },
                // more here
                _ => todo!()
                // TODO: Add error handling
            }
        }
    }
    // TODO: Add a way to assign values to variables and evaluate with those values.
}

impl From<Vec< Token >> for Expr {
    fn from(value: Vec<Token>) -> Self {
        Self(value)
    }
}
