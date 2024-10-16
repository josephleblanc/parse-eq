//use std::cell::RefCell;
//use std::rc::Rc;

use parse_eq::eq_props::negation::Negation;
use std::fs::File;
use std::io::Read;

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
fn tree_unary_simple() {
    use binary_tree_ds::TreeNode;
    use parse_eq::lexer::Lexer;
    use parse_eq::token::Token::*;
    use parse_eq::token::UnaryOperator::*;
    use parse_eq::tree::Tree;

    let one = TreeNode::new_rc(Number(1.0), None, None);
    let sine_one = TreeNode::new_rc(UnOp(Sine), None, Some(one));

    let check_tree = Tree::new(sine_one);

    let lexer = Lexer::new_inorder("sin 1").unwrap();
    let in_order = lexer.list;

    let tree: Tree = Tree::new_pre_from_in(in_order);
    assert_eq!(check_tree, tree);
}

#[test]
fn tree_unary_with_var() {
    use binary_tree_ds::TreeNode;
    use parse_eq::lexer::Lexer;
    use parse_eq::token::Operator::*;
    use parse_eq::token::Token::*;
    use parse_eq::token::UnaryOperator::*;
    use parse_eq::token::Variable;
    use parse_eq::tree::Tree;

    let one = TreeNode::new_rc(Number(1.0), None, None);
    let x = TreeNode::new_rc(Var(Variable::X), None, None);
    let sine_x = TreeNode::new_rc(UnOp(Sine), None, Some(x));

    let sin_x_plus_one = TreeNode::new_rc(Op(Add), Some(sine_x), Some(one));

    let check_tree = Tree::new(sin_x_plus_one);

    let lexer = Lexer::new_inorder("sin x + 1").unwrap();
    let in_order = lexer.list;

    let tree: Tree = Tree::new_pre_from_in(in_order);
    assert_eq!(check_tree, tree);
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

#[test]
fn tree_unary_parens() {
    use binary_tree_ds::TreeNode;
    use parse_eq::lexer::Lexer;
    use parse_eq::token::Operator::*;
    use parse_eq::token::Token::*;
    use parse_eq::token::UnaryOperator::*;
    use parse_eq::token::Variable;
    use parse_eq::tree::Tree;

    let one = TreeNode::new_rc(Number(1.0), None, None);
    let x = TreeNode::new_rc(Var(Variable::X), None, None);
    let sin_x = TreeNode::new_rc(UnOp(Sine), None, Some(x));

    let sin_x_plus_one = TreeNode::new_rc(Op(Add), Some(sin_x), Some(one));
    let check_tree = Tree::new(sin_x_plus_one);

    let lexer = Lexer::new_inorder("( sin x ) + 1").unwrap();
    let in_order = lexer.list;

    let tree: Tree = Tree::new_pre_from_in(in_order);
    assert_eq!(tree, check_tree);

    let lexer2 = Lexer::new_inorder("(sin x) + 1").unwrap();
    let in_order2 = lexer2.list;

    let tree2: Tree = Tree::new_pre_from_in(in_order2);
    assert_eq!(tree2, check_tree);

    let lexer3 = Lexer::new_inorder("sin x + (1)").unwrap();
    let in_order3 = lexer3.list;

    let tree3: Tree = Tree::new_pre_from_in(in_order3);
    assert_eq!(tree3, check_tree);

    let lexer4 = Lexer::new_inorder("(sin x + 1)").unwrap();
    let in_order4 = lexer4.list;

    let tree4: Tree = Tree::new_pre_from_in(in_order4);
    assert_eq!(tree4, check_tree);

    let lexer5 = Lexer::new_inorder("(sin (x) + 1)").unwrap();
    let in_order5 = lexer5.list;

    let tree5: Tree = Tree::new_pre_from_in(in_order5);
    assert_eq!(tree5, check_tree);
}

#[test]
fn tree_unary_complex() {
    use binary_tree_ds::TreeNode;
    use parse_eq::lexer::Lexer;
    use parse_eq::token::Operator::*;
    use parse_eq::token::Token::*;
    use parse_eq::token::UnaryOperator::*;
    use parse_eq::token::Variable;
    use parse_eq::tree::Tree;

    let one = TreeNode::new_rc(Number(1.0), None, None);
    let two = TreeNode::new_rc(Number(2.0), None, None);
    let five = TreeNode::new_rc(Number(5.0), None, None);
    let x = TreeNode::new_rc(Var(Variable::X), None, None);
    let sin_x = TreeNode::new_rc(UnOp(Sine), None, Some(x));

    let five_sine_x = TreeNode::new_rc(Op(Multiply), Some(five), Some(sin_x));
    let minus_two = TreeNode::new_rc(Op(Subtract), Some(two), Some(five_sine_x));
    let plus_one = TreeNode::new_rc(Op(Add), Some(minus_two), Some(one));
    let check_tree = Tree::new(plus_one);

    let lexer = Lexer::new_inorder("2 - (5 * sin x ) + 1").unwrap();
    let in_order = lexer.list;

    let tree: Tree = Tree::new_pre_from_in(in_order);

    assert_eq!(check_tree, tree);
}

#[test]
fn tree_unary_negation() {
    use binary_tree_ds::TreeNode;
    use parse_eq::lexer::Lexer;
    use parse_eq::token::Operator::*;
    use parse_eq::token::Token::*;
    use parse_eq::token::UnaryOperator::*;
    use parse_eq::token::Variable;
    use parse_eq::tree::Tree;

    let one = TreeNode::new_rc(Number(1.0), None, None);
    let two = TreeNode::new_rc(Number(2.0), None, None);
    let five = TreeNode::new_rc(Number(5.0), None, None);
    let x = TreeNode::new_rc(Var(Variable::X), None, None);

    let neg_x = TreeNode::new_rc(UnOp(Negation), None, Some(x));
    let sin_neg_x = TreeNode::new_rc(UnOp(Sine), None, Some(neg_x));
    let neg_two = TreeNode::new_rc(UnOp(Negation), None, Some(two));

    let neg_sin_neg_x = TreeNode::new_rc(UnOp(Negation), None, Some(sin_neg_x));
    let five_neg_sine_neg_x = TreeNode::new_rc(Op(Multiply), Some(five), Some(neg_sin_neg_x));
    let neg_two_minus = TreeNode::new_rc(Op(Subtract), Some(neg_two), Some(five_neg_sine_neg_x));
    let plus_one = TreeNode::new_rc(Op(Add), Some(neg_two_minus), Some(one));
    let check_tree = Tree::new(plus_one);

    let lexer = Lexer::new_inorder("-2 - (5 * -sin -x ) + 1").unwrap();
    let in_order = lexer.list;
    println!("lexer.list: {:?}", in_order);

    let tree: Tree = Tree::new_pre_from_in(in_order);
    tree.save_typst_tree("typst_test2.typ").unwrap();

    assert_eq!(check_tree, tree);
}

#[test]
fn tree_save_typst() {
    use parse_eq::lexer::Lexer;
    use parse_eq::tree::Tree;

    // Output tested above
    // 2 * ( 5 * 3 + 4 / ( 1 + 6 ) )

    let lexer = Lexer::new_inorder("-2 * ( 5 * 3 + 4 / ( 1 + -sin -(x/2) ) )").unwrap();
    let in_order = lexer.list;

    let tree: Tree = Tree::new_pre_from_in(in_order);
    tree.save_typst_tree("typst_test.typ").unwrap();

    let mut file = File::open("./typst_test.typ").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    assert_eq!(
        contents,
        r#"
#let data = (
([\*], ([-], [2.000]), ([+], ([\*], [5.000], [3.000]), ([/], [4.000], ([+], [1.000], ([-], ([sin], ([-], ([/], [x], [2.000]))))))))
)

#import "@preview/cetz:0.1.2": canvas, draw, tree

#canvas(length: 1cm, {
  import draw: *

  set-style(content: (padding: .2),
    fill: gray.lighten(70%),
    stroke: gray.lighten(70%))

  tree.tree(data, spread: 2.5, grow: 1.5, draw-node: (node, _) => {
    circle((), radius: .45, stroke: none)
    content((), node.content)
  }, draw-edge: (from, to, _) => {
    line((a: from, number: .6, abs: true, b: to),
         (a: to, number: .6, abs: true, b: from), mark: (end: ">"))
  }, name: "tree")
})
"#
    );
}

#[test]
fn tree_create_vec() {
    use parse_eq::lexer::Lexer;
    use parse_eq::lexer::Ordering;
    use parse_eq::tree::Tree;

    let lexer = Lexer::new_inorder("-2 * ( 5 * 3 + 4 / ( 1 + -sin -(x/2) ) )").unwrap();
    let in_order = lexer.list;

    let tree: Tree = Tree::new_pre_from_in(in_order);
    let in_order_vec = tree.create_vec(Ordering::In);

    let check_lexer = Lexer::new_inorder("-2 * ( 5 * 3 + 4 / ( 1 + -sin -(x/2) ) )").unwrap();
    let check_in_order = check_lexer.list;

    for (check_val, test_val) in check_in_order.iter().zip(in_order_vec.iter()) {
        println!("check: {}\t test: {}", check_val, test_val);
    }
    assert_eq!(check_in_order, in_order_vec);
}

#[test]
fn tree_detect_negation() {
    use parse_eq::lexer::Lexer;
    use parse_eq::lexer::Ordering;
    use parse_eq::token::{
        Operator::*,
        Token,
        Token::{LParen, Number, Op, RParen, UnOp, Var},
        UnaryOperator, Variable,
    };
    use parse_eq::tree::Tree;

    // 1. Double negated number
    let lexer = Lexer::new_inorder("- ( -2 )").unwrap();
    let in_order = lexer.list;
    let check_vec: Vec<Token> = vec![
        UnOp(UnaryOperator::Negation),
        LParen,
        UnOp(UnaryOperator::Negation),
        Number(2.0),
        RParen,
    ];
    assert_eq!(check_vec, in_order);
    let tree: Tree = Tree::new_pre_from_in(in_order.clone());
    let root = tree.get_root_clone();
    assert!(root.is_double_neg());

    // 2. Single negation number
    let lexer = Lexer::new_inorder("( -2 )").unwrap();
    let in_order = lexer.list;
    let check_vec: Vec<Token> = vec![LParen, UnOp(UnaryOperator::Negation), Number(2.0), RParen];
    assert_eq!(check_vec, in_order);
    let tree: Tree = Tree::new_pre_from_in(in_order.clone());
    let root = tree.get_root_clone();
    assert!(!root.is_double_neg());

    // 3. Double negation variable
    let lexer = Lexer::new_inorder("- ( -x )").unwrap();
    let in_order = lexer.list;
    let check_vec: Vec<Token> = vec![
        UnOp(UnaryOperator::Negation),
        LParen,
        UnOp(UnaryOperator::Negation),
        Var(Variable::X),
        RParen,
    ];
    assert_eq!(check_vec, in_order);
    let tree: Tree = Tree::new_pre_from_in(in_order.clone());
    let root = tree.get_root_clone();
    assert!(root.is_double_neg());

    // 4. Double negation with addition operation
    let lexer = Lexer::new_inorder("1 + - ( -x )").unwrap();
    let in_order = lexer.list;
    let check_vec: Vec<Token> = vec![
        Number(1.0),
        Op(Add),
        UnOp(UnaryOperator::Negation),
        LParen,
        UnOp(UnaryOperator::Negation),
        Var(Variable::X),
        RParen,
    ];
    assert_eq!(check_vec, in_order);
    let tree: Tree = Tree::new_pre_from_in(in_order.clone());

    let root = tree.get_root_clone();
    assert!(root.right.unwrap().borrow().is_double_neg());

    // 5. Double negation with subtract operation
    let lexer = Lexer::new_inorder("1 - - ( -x )").unwrap();
    let in_order = lexer.list;
    let check_vec: Vec<Token> = vec![
        Number(1.0),
        Op(Subtract),
        UnOp(UnaryOperator::Negation),
        LParen,
        UnOp(UnaryOperator::Negation),
        Var(Variable::X),
        RParen,
    ];
    assert_eq!(check_vec, in_order);
    let tree: Tree = Tree::new_pre_from_in(in_order.clone());

    let root = tree.get_root_clone();
    assert!(root.right.unwrap().borrow().is_double_neg());
}
