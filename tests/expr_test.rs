#[cfg(test)]
#[test]
fn simple() {
    use parse_eq::lexer::Lexer;
    use parse_eq::token::Operator::*;
    use parse_eq::token::Token::*;
    let add_pre = vec![Op(Add), Number(1.0), Number(2.0)];

    let mut lexer = Lexer::new_inorder("1+2").unwrap();
    lexer.in_to_pre();
    assert_eq!(add_pre, lexer.list);
}

#[test]
fn multiply_and_add() {
    use parse_eq::lexer::Lexer;
    use parse_eq::token::Operator::*;
    use parse_eq::token::Token::*;
    let add_mult_pre = vec![Op(Add), Number(1.0), Op(Multiply), Number(2.0), Number(3.0)];

    let mut lexer = Lexer::new_inorder("1+2*3").unwrap();
    lexer.in_to_pre();
    assert_eq!(add_mult_pre, lexer.list);
}
