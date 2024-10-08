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

#[test]
fn tree_parens_simple() {
    use binary_tree_ds::TreeNode;
    use parse_eq::lexer::Lexer;
    use parse_eq::token::Operator::*;
    use parse_eq::token::Token::*;
    use parse_eq::tree::Tree;

    // 2 * ( 5 * 3 + 4 / ( 1 + 6 ) )
    let one = Rc::new(RefCell::new(TreeNode::new(Number(1.0), None, None)));
    let two = Rc::new(RefCell::new(TreeNode::new(Number(2.0), None, None)));

    let add = Rc::new(RefCell::new(TreeNode::new(Op(Add), Some(one), Some(two))));
    let check_tree: Tree = Tree::new(add);

    let lexer = Lexer::new_inorder("(1 + 2)").unwrap();
    let in_order = lexer.list;

    let check_in_order = vec![LParen, Number(1.0), Op(Add), Number(2.0), RParen];
    assert_eq!(in_order, check_in_order);

    let tree: Tree = Tree::new_pre_from_in(in_order);
    assert_eq!(tree, check_tree);
}

#[test]
fn tree_parens() {
    use binary_tree_ds::TreeNode;
    use parse_eq::lexer::Lexer;
    use parse_eq::token::Operator::*;
    use parse_eq::token::Token::*;
    use parse_eq::tree::Tree;

    // 2 * ( 5 * 3 + 4 / ( 1 + 6 ) )
    let one = Rc::new(RefCell::new(TreeNode::new(Number(1.0), None, None)));
    let two = Rc::new(RefCell::new(TreeNode::new(Number(2.0), None, None)));
    let three = Rc::new(RefCell::new(TreeNode::new(Number(3.0), None, None)));
    let four = Rc::new(RefCell::new(TreeNode::new(Number(4.0), None, None)));
    let five = Rc::new(RefCell::new(TreeNode::new(Number(5.0), None, None)));
    let six = Rc::new(RefCell::new(TreeNode::new(Number(6.0), None, None)));

    let lower_plus = Rc::new(RefCell::new(TreeNode::new(Op(Add), Some(one), Some(six))));
    let division = Rc::new(RefCell::new(TreeNode::new(
        Op(Divide),
        Some(four),
        Some(lower_plus),
    )));
    let lower_mult = Rc::new(RefCell::new(TreeNode::new(
        Op(Multiply),
        Some(five),
        Some(three),
    )));
    let upper_plus = Rc::new(RefCell::new(TreeNode::new(
        Op(Add),
        Some(lower_mult),
        Some(division),
    )));
    let upper_mult = Rc::new(RefCell::new(TreeNode::new(
        Op(Multiply),
        Some(two),
        Some(upper_plus),
    )));
    let check_tree: Tree = Tree::new(upper_mult);

    let lexer = Lexer::new_inorder("2 * ( 5 * 3 + 4 / ( 1 + 6 ) )").unwrap();
    let in_order = lexer.list;

    let tree: Tree = Tree::new_pre_from_in(in_order);
    assert_eq!(tree, check_tree);
}
