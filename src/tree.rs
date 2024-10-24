use crate::lexer::Ordering;
use crate::token::Priority;
use crate::token::Token;
use binary_tree_ds::*;
use std::cell::RefCell;
use std::rc::Rc;

pub type TreeNodeRef<T: Sized + Copy> = Rc<RefCell<TreeNode<T>>>;

#[derive(Debug, Clone, PartialEq)]
pub struct Tree {
    // TODO: Decide later whether this should be pub or not.
    pub(crate) root: TreeNodeRef<Token>,
}

impl Tree {
    pub fn new(root: TreeNodeRef<Token>) -> Self {
        Tree { root }
    }

    /// Construct a new binary tree representation of the expression from an in-order vec of
    /// tokens.
    /// Assumes the vec is well-formed, including appropriate number of parentheses.
    /// Algorithm mostly cribbed from this C++ implementation, modified to include unary operators:
    ///     https://leetcode.ca/2020-04-14-1597-Build-Binary-Expression-Tree-From-Infix-Expression/
    pub fn new_pre_from_in(in_order: Vec<Token>) -> Tree {
        let mut ops: Vec<Token> = vec![];
        let mut stack: Vec<TreeNodeRef<Token>> = vec![];

        for token in in_order {
            match token {
                Token::LParen => ops.push(token),

                Token::Number(n) => {
                    stack.push(TreeNode::new_rc(Token::Number(n), None, None));
                    if matches!(ops.last(), Some(Token::UnOp(_))) {
                        while matches!(ops.last(), Some(Token::UnOp(_))) {
                            Tree::combine(&mut ops, &mut stack);
                        }
                    }
                }
                Token::Var(v) => {
                    stack.push(TreeNode::new_rc(Token::Var(v), None, None));
                    if matches!(ops.last(), Some(Token::UnOp(_))) {
                        while matches!(ops.last(), Some(Token::UnOp(_))) {
                            Tree::combine(&mut ops, &mut stack);
                        }
                    }
                }
                Token::RParen => {
                    while let Some(stack_op) = ops.last() {
                        if stack_op != &Token::LParen {
                            Tree::combine(&mut ops, &mut stack);
                        } else {
                            ops.pop();
                            break;
                        }
                    }
                }
                Token::Op(op) => {
                    while !ops.is_empty() && ops.last().unwrap().priority() >= op.priority() {
                        Tree::combine(&mut ops, &mut stack);
                    }
                    ops.push(Token::Op(op));
                }
                Token::UnOp(un_op) => {
                    ops.push(Token::UnOp(un_op));
                }
            }
        }

        while stack.len() > 1 || !ops.is_empty() {
            Tree::combine(&mut ops, &mut stack);
        }

        Tree {
            root: stack.pop().unwrap(),
        }
    }

    pub fn save_typst_tree(&self, file: &'static str) -> std::io::Result<()> {
        let bin_tree_struct: binary_tree_ds::Tree<Token> =
            binary_tree_ds::Tree::new(self.root.clone());
        bin_tree_struct.save_typst(file)?;
        Ok(())
    }

    pub fn create_vec(&self, order: Ordering) -> Vec<Token> {
        let mut stack: Vec<Token> = vec![];
        push_into_order(&self.root, &mut stack, order);
        stack
    }

    fn combine(ops: &mut Vec<Token>, stack: &mut Vec<TreeNodeRef<Token>>) {
        let mut root = TreeNode::new(ops.pop().unwrap(), None, None);
        if matches!(root.value, Token::UnOp(_)) {
            root.right = Some(stack.pop().unwrap());
        } else {
            root.right = Some(stack.pop().unwrap());
            root.left = Some(stack.pop().unwrap());
        }
        stack.push(Rc::new(RefCell::new(root)));
    }

    pub fn get_root_clone(&self) -> TreeNode<Token> {
        self.root.borrow().clone()
    }
}

/// Recursive function used in method `create_vec` to take the tree and return a vector of the tree
/// in a given order. See `Tree::create_vec` for more.
/// Neither variables nor numbers should have parentheses surrounding them alone, and parentheses
/// should only be presented when neccesary.
///
/// Examples:
/// 5.000 -> 5.000, not (5.000)
/// x -> x, not (x)
///
/// The tree
///         *
///        / \
///       3   +
///          / \
///         2   4
/// becomes
/// 3 * (2 + 4)
///
/// The tree
///         +
///        / \
///       3   +
///          / \
///         2   4
/// becomes
/// 3 + 2 + 4, not 3 + (2 + 4)
fn push_into_order(node_ref: &TreeNodeRef<Token>, stack: &mut Vec<Token>, order: Ordering) {
    // Early return to avoid adding extraneous parentheses
    if node_ref.borrow().is_leaf() {
        stack.push(node_ref.borrow().value);
        return;
    }

    if order == Ordering::Pre {
        stack.push(node_ref.borrow().value);
    }
    if let Some(ref left) = node_ref.borrow().left {
        if !matches!(left.borrow().value, Token::Number(_))
            && !matches!(left.borrow().value, Token::Var(_))
            && node_ref.borrow().value.priority() > left.borrow().value.priority()
        {
            stack.push(Token::LParen);
            push_into_order(left, stack, order);
            stack.push(Token::RParen);
        } else {
            push_into_order(left, stack, order);
        }
    }
    if order == Ordering::In {
        stack.push(node_ref.borrow().value);
    }
    if let Some(ref right) = node_ref.borrow().right {
        if !matches!(right.borrow().value, Token::Number(_))
            && !matches!(right.borrow().value, Token::Var(_))
            && node_ref.borrow().value.priority() > right.borrow().value.priority()
        {
            stack.push(Token::LParen);
            push_into_order(right, stack, order);
            stack.push(Token::RParen);
        } else {
            push_into_order(right, stack, order);
        }
    }
    if order == Ordering::Post {
        stack.push(node_ref.borrow().value);
    }
}

///// Recursive function used in method `create_vec` to take the tree and return a vector of the tree
///// in a given order. See `Tree::create_vec` for more.
//fn push_into_order(
//    node_ref: &TreeNodeRef<Token>,
//    parent: Option<&TreeNodeRef<Token>>,
//    stack: &mut Vec<Token>,
//    order: Ordering,
//) {
//    // Early return to avoid adding extraneous parentheses
//    if node_ref.borrow().is_leaf() {
//        stack.push(node_ref.borrow().value);
//        return;
//    }
//
//    if order == Ordering::Pre {
//        stack.push(node_ref.borrow().value);
//    }
//    // handle left child
//    // if putting into `In` order, also insert a left parenthesis, but only if the parent operation
//    // is of higher priority, e.g. 2 * (1 + 3), and not when the operations are of the same
//    // priority or when the parent has lower priority, e.g. 2 + 1 + 3 or 2 * 1 + 3.
//    if let Some(ref left) = node_ref.borrow().left {
//        if let Some(parent_op) = parent {
//            if order == Ordering::In
//                && parent_op.borrow().value.priority() > left.borrow().value.priority()
//            {
//                stack.push(Token::LParen);
//            }
//        }
//        push_into_order(left, Some(node_ref), stack, order);
//    }
//    if order == Ordering::In {
//        stack.push(node_ref.borrow().value);
//    }
//    if let Some(ref right) = node_ref.borrow().right {
//        push_into_order(right, Some(node_ref), stack, order);
//        stack.push(Token::RParen);
//    }
//    if order == Ordering::Post {
//        stack.push(node_ref.borrow().value);
//    }
//}
