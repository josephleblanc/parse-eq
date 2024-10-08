use std::cell::RefCell;
use std::rc::Rc;

#[cfg(test)]
#[test]
fn tree_simple() {
    use binary_tree_ds::TreeNode;
    use parse_eq::lexer::Lexer;
    use parse_eq::token::Operator::*;
    use parse_eq::token::Token::*;
    use parse_eq::tree::Tree;

    let left_node = Rc::new(RefCell::new(TreeNode::new(Number(1.0), None, None)));
    let right_node = Rc::new(RefCell::new(TreeNode::new(Number(2.0), None, None)));
    let check_tree: Tree = Tree::new(Rc::new(RefCell::new(TreeNode::new(
        Op(Divide),
        Some(left_node),
        Some(right_node),
    ))));
    let lexer = Lexer::new_inorder("1/2").unwrap();
    let in_order = lexer.list;

    let tree: Tree = Tree::new_pre_from_in(in_order);
    assert_eq!(tree, check_tree);
}

//#[test]
//fn simple() {
//    use parse_eq::lexer::Lexer;
//    use parse_eq::token::Operator::*;
//    use parse_eq::token::Token::*;
//
//    let add_pre = vec![Op(Add), Number(1.0), Number(2.0)];
//    let lexer = Lexer::new_inorder("1+2").unwrap();
//    let in_order = lexer.list;
//
//    use parse_eq::tree::Tree;
//    let tree: Tree = Tree::new_pre_from_in(in_order);
//}
