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
                        println!("running RParen while loop");
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
                        println!("running Op while loop");
                        Tree::combine(&mut ops, &mut stack);
                    }
                    ops.push(Token::Op(op));
                }
                Token::UnOp(un_op) => {
                    while !ops.is_empty() && ops.last().unwrap().priority() >= un_op.priority() {
                        println!("running UnOp while loop");
                        Tree::combine(&mut ops, &mut stack);
                    }
                    ops.push(Token::UnOp(un_op));
                }
            }
        }

        while stack.len() > 1 {
            println!("running end stack while loop");
            Tree::combine(&mut ops, &mut stack);
        }

        Tree {
            root: stack.pop().unwrap(),
        }
    }

    pub fn save_typst(&self, file: &'static str) -> std::io::Result<()> {
        let bin_tree_struct: binary_tree_ds::Tree<Token> =
            binary_tree_ds::Tree::new(self.root.clone());
        bin_tree_struct.save_typst(file)?;
        Ok(())
    }

    fn combine(ops: &mut Vec<Token>, stack: &mut Vec<TreeNodeRef<Token>>) {
        println!("Running combine");
        let mut root = TreeNode::new(ops.pop().unwrap(), None, None);
        if matches!(root.value, Token::UnOp(_)) {
            root.right = Some(stack.pop().unwrap());
        } else {
            root.right = Some(stack.pop().unwrap());
            root.left = Some(stack.pop().unwrap());
        }
        stack.push(Rc::new(RefCell::new(root)));
    }
}
