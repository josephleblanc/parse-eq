use crate::token::{
    Operator, Token,
    Token::{Op, UnOp},
    UnaryOperator,
};
use binary_tree_ds::TreeNode;

pub trait Negation {
    fn is_negation(&self) -> bool;
}

impl Negation for TreeNode<Token> {
    fn is_negation(&self) -> bool {
        if let Some(ref right) = self.right {
            return self.value == UnOp(UnaryOperator::Negation)
                && right.borrow().value == UnOp(UnaryOperator::Negation);
        } else {
            false
        }
    }
}
