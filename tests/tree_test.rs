//use std::cell::RefCell;
//use std::rc::Rc;

#[cfg(test)]
#[test]
fn tree_simple() {
    use binary_tree_ds::TreeNode;
    use parse_eq::lexer::Lexer;
    use parse_eq::token::Operator::*;
    use parse_eq::token::Token::*;
    use parse_eq::tree::Tree;

    let left_node = TreeNode::new_rc(Number(1.0), None, None);
    let right_node = TreeNode::new_rc(Number(2.0), None, None);
    let check_tree: Tree = Tree::new(TreeNode::new_rc(
        Op(Divide),
        Some(left_node),
        Some(right_node),
    ));
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
    let one = TreeNode::new_rc(Number(1.0), None, None);
    let two = TreeNode::new_rc(Number(2.0), None, None);

    let add = TreeNode::new_rc(Op(Add), Some(one), Some(two));
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
    let one = TreeNode::new_rc(Number(1.0), None, None);
    let two = TreeNode::new_rc(Number(2.0), None, None);
    let three = TreeNode::new_rc(Number(3.0), None, None);
    let four = TreeNode::new_rc(Number(4.0), None, None);
    let five = TreeNode::new_rc(Number(5.0), None, None);
    let six = TreeNode::new_rc(Number(6.0), None, None);

    let lower_plus = TreeNode::new_rc(Op(Add), Some(one), Some(six));
    let division = TreeNode::new_rc(Op(Divide), Some(four), Some(lower_plus));
    let lower_mult = TreeNode::new_rc(Op(Multiply), Some(five), Some(three));
    let upper_plus = TreeNode::new_rc(Op(Add), Some(lower_mult), Some(division));
    let upper_mult = TreeNode::new_rc(Op(Multiply), Some(two), Some(upper_plus));
    let check_tree: Tree = Tree::new(upper_mult);

    let lexer = Lexer::new_inorder("2 * ( 5 * 3 + 4 / ( 1 + 6 ) )").unwrap();
    let in_order = lexer.list;

    let tree: Tree = Tree::new_pre_from_in(in_order);
    assert_eq!(tree, check_tree);
}

#[test]
fn tree_parens_many() {
    use binary_tree_ds::TreeNode;
    use parse_eq::lexer::Lexer;
    use parse_eq::token::Operator::*;
    use parse_eq::token::Token::*;
    use parse_eq::tree::Tree;

    // 2 * ( 5 * 3 + 4 / ( 1 + 6 ) )
    let one = TreeNode::new_rc(Number(1.0), None, None);
    let two = TreeNode::new_rc(Number(2.0), None, None);
    let three = TreeNode::new_rc(Number(3.0), None, None);
    let four = TreeNode::new_rc(Number(4.0), None, None);
    let five = TreeNode::new_rc(Number(5.0), None, None);
    let six = TreeNode::new_rc(Number(6.0), None, None);

    let lower_plus = TreeNode::new_rc(Op(Add), Some(one), Some(six));
    let division = TreeNode::new_rc(Op(Divide), Some(four), Some(lower_plus));
    let lower_mult = TreeNode::new_rc(Op(Multiply), Some(five), Some(three));
    let upper_plus = TreeNode::new_rc(Op(Add), Some(lower_mult), Some(division));
    let upper_mult = TreeNode::new_rc(Op(Multiply), Some(two), Some(upper_plus));
    let check_tree: Tree = Tree::new(upper_mult);

    let lexer = Lexer::new_inorder("( ( 2 * ( 5 * 3 + 4 / ( 1 + 6 ) ) ) )").unwrap();
    let in_order = lexer.list;

    let tree: Tree = Tree::new_pre_from_in(in_order);
    assert_eq!(tree, check_tree);
}

#[test]
fn tree_parens_many_more() {
    use binary_tree_ds::TreeNode;
    use parse_eq::lexer::Lexer;
    use parse_eq::token::Operator::*;
    use parse_eq::token::Token::*;
    use parse_eq::tree::Tree;

    // 2 * ( 5 * 3 + 4 / ( 1 + 6 ) )
    let one = TreeNode::new_rc(Number(1.0), None, None);
    let two = TreeNode::new_rc(Number(2.0), None, None);
    let three = TreeNode::new_rc(Number(3.0), None, None);
    let four = TreeNode::new_rc(Number(4.0), None, None);
    let five = TreeNode::new_rc(Number(5.0), None, None);
    let six = TreeNode::new_rc(Number(6.0), None, None);

    let lower_plus = TreeNode::new_rc(Op(Add), Some(one), Some(six));
    let division = TreeNode::new_rc(Op(Divide), Some(four), Some(lower_plus));
    let lower_mult = TreeNode::new_rc(Op(Multiply), Some(five), Some(three));
    let upper_plus = TreeNode::new_rc(Op(Add), Some(lower_mult), Some(division));
    let upper_mult = TreeNode::new_rc(Op(Multiply), Some(two), Some(upper_plus));
    let check_tree: Tree = Tree::new(upper_mult);

    let lexer = Lexer::new_inorder("( ( 2 * ( ( 5 * 3 ) + 4 / ( ( 1 ) + ( 6 ) ) ) ) )").unwrap();
    let in_order = lexer.list;

    let tree: Tree = Tree::new_pre_from_in(in_order);
    assert_eq!(tree, check_tree);
}

#[test]
fn tree_var_simple() {
    use binary_tree_ds::TreeNode;
    use parse_eq::lexer::Lexer;
    use parse_eq::token::Operator::*;
    use parse_eq::token::Token::*;
    use parse_eq::token::Variable;
    use parse_eq::tree::Tree;

    let one = TreeNode::new_rc(Number(1.0), None, None);
    let x = TreeNode::new_rc(Var(Variable::X), None, None);

    let x_plus_one = TreeNode::new_rc(Op(Add), Some(x), Some(one));
    let check_tree = Tree::new(x_plus_one);

    let lexer = Lexer::new_inorder("x + 1").unwrap();
    let in_order = lexer.list;

    let tree: Tree = Tree::new_pre_from_in(in_order);
    assert_eq!(tree, check_tree);
}

#[test]
fn tree_var_parens() {
    use binary_tree_ds::TreeNode;
    use parse_eq::lexer::Lexer;
    use parse_eq::token::Operator::*;
    use parse_eq::token::Token::*;
    use parse_eq::token::Variable;
    use parse_eq::tree::Tree;

    let one = TreeNode::new_rc(Number(1.0), None, None);
    let x = TreeNode::new_rc(Var(Variable::X), None, None);

    let x_plus_one = TreeNode::new_rc(Op(Add), Some(x), Some(one));
    let check_tree = Tree::new(x_plus_one);

    let lexer = Lexer::new_inorder("( x ) + 1").unwrap();
    let in_order = lexer.list;

    let tree: Tree = Tree::new_pre_from_in(in_order);
    assert_eq!(tree, check_tree);

    let lexer2 = Lexer::new_inorder("(x) + 1").unwrap();
    let in_order2 = lexer2.list;

    let tree2: Tree = Tree::new_pre_from_in(in_order2);
    assert_eq!(tree2, check_tree);

    let lexer3 = Lexer::new_inorder("x + (1)").unwrap();
    let in_order3 = lexer3.list;

    let tree3: Tree = Tree::new_pre_from_in(in_order3);
    assert_eq!(tree3, check_tree);

    let lexer3 = Lexer::new_inorder("(x + 1)").unwrap();
    let in_order3 = lexer3.list;

    let tree3: Tree = Tree::new_pre_from_in(in_order3);
    assert_eq!(tree3, check_tree);
}
