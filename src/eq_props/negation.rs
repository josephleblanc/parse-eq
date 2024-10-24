use crate::token::{Token, Token::UnOp, UnaryOperator};
use crate::tree::TreeNodeRef;
use binary_tree_ds::TreeNode;
use binary_tree_ds::TreeNodeProperties;
use uuid::Uuid;

pub trait Negation {
    fn is_double_neg(&self) -> bool;
    fn neg_id_first(&self) -> Option<Uuid>;
    //fn do_double_neg(&mut self) -> Result<(), &'static str>;
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
            println!(
                "self.value: {:?}, right.borrow().value: {:?}",
                self.value,
                right.borrow().value
            );
            return self.value == UnOp(UnaryOperator::Negation)
                && right.borrow().value == UnOp(UnaryOperator::Negation);
        } else {
            false
        }
    }

    fn neg_id_first(&self) -> Option<uuid::Uuid> {
        if self.is_double_neg() {
            return Some(self.get_id());
        }
        if let Some(ref right) = self.right {
            if right.borrow().is_double_neg() {
                return Some(right.borrow().get_id());
            } else {
                return right.borrow().neg_id_first();
            }
        }
        None
    }
}

impl crate::tree::Tree {
    pub fn get_by_id(&self, id: Uuid) -> Option<TreeNodeRef<Token>> {
        self.root.get_by_id(id)
    }
    pub fn do_double_neg(&mut self, id: Uuid) -> Result<(), &'static str> {
        use std::rc::Rc;

        let mut target_node: Option<TreeNodeRef<Token>>;
        println!("strong rc for root: {}", Rc::strong_count(&self.root));
        if self.root.clone().borrow().cmp_id(id) {
            todo!()
        }
        if let Some(node) = self.get_by_id(id) {
            todo!()
        }
        Err("Attempted to apply 'do_double_neg' to an invalid node")
    }

    pub fn neg_id_first(&self) -> Option<Uuid> {
        self.root.borrow().neg_id_first()
    }
}
