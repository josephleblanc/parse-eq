use crate::lexer::Token;
use binary_tree_ds::*;
use std::cell::RefCell;
use std::rc::Rc;

type TreeNodeRef<T: Sized + Copy> = Rc<RefCell<TreeNode<T>>>;

pub struct Tree {
    root: TreeNodeRef<Token>,
}
