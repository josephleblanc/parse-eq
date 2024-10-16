use crate::token::{
    Operator, Token,
    Token::{Op, UnOp},
    UnaryOperator,
};
use binary_tree_ds::TreeNode;

pub trait Negation {
    fn is_double_neg(&self) -> bool;
}

impl Negation for TreeNode<Token> {
    /// Checks whether the node is a double negative branch, e.g.
    ///     neg
    ///        \
    ///         neg
    ///           \
    ///           node
    fn is_double_neg(&self) -> bool {
        if let Some(ref right) = self.right {
            //if right.borrow().right.is_none() {
            //}
            return self.value == UnOp(UnaryOperator::Negation)
                && right.borrow().value == UnOp(UnaryOperator::Negation);
        } else {
            false
        }
    }
}
