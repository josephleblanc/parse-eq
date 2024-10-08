use crate::token::Priority;
use crate::token::Token;
use binary_tree_ds::*;
use std::cell::RefCell;
use std::rc::Rc;

type TreeNodeRef<T: Sized + Copy> = Rc<RefCell<TreeNode<T>>>;

#[derive(Debug, Clone, PartialEq)]
pub struct Tree {
    root: TreeNodeRef<Token>,
}

impl Tree {
    pub fn new(root: TreeNodeRef<Token>) -> Self {
        Tree { root }
    }

    /// Construct a new binary tree representation of the expression from an in-order vec of
    /// tokens.
    /// Assumes the vec is well-formed, including appropriate number of parentheses.
    /// Algorithm mostly cribbed from this C++ implementation:
    ///     https://leetcode.ca/2020-04-14-1597-Build-Binary-Expression-Tree-From-Infix-Expression/
    pub fn new_pre_from_in(in_order: Vec<Token>) -> Tree {
        let mut ops: Vec<Token> = vec![];
        let mut stack: Vec<TreeNodeRef<Token>> = vec![];

        for token in in_order {
            match token {
                Token::LParen => ops.push(token),

                Token::Number(n) => stack.push(Rc::new(RefCell::new(TreeNode::new(
                    Token::Number(n),
                    None,
                    None,
                )))),
                Token::Var(v) => stack.push(Rc::new(RefCell::new(TreeNode::new(
                    Token::Var(v),
                    None,
                    None,
                )))),

                Token::RParen => {
                    while let Some(stack_op) = ops.last() {
                        if stack_op != &Token::LParen {
                            Tree::combine(&mut ops, &mut stack);
                        }
                    }
                }
                Token::Op(op) => {
                    while !ops.is_empty() && ops.last().unwrap().priority() >= op.priority() {
                        Tree::combine(&mut ops, &mut stack);
                    }
                    ops.push(Token::Op(op));
                }
            }
        }

        while stack.len() > 1 {
            Tree::combine(&mut ops, &mut stack);
        }

        Tree {
            root: stack.pop().unwrap(),
        }
    }

    fn combine(ops: &mut Vec<Token>, stack: &mut Vec<TreeNodeRef<Token>>) {
        let mut root = TreeNode::new(ops.pop().unwrap(), None, None);
        root.right = Some(stack.pop().unwrap());
        root.left = Some(stack.pop().unwrap());
        stack.push(Rc::new(RefCell::new(root)));
    }
}
