#[cfg(test)]
#[test]
fn split_nums_simple() {
    // TODO: add lexer tests here
    use parse_eq::lexer::split_nums;
    assert_eq!(vec!["1"], split_nums("1").unwrap());
    assert_eq!(vec!["12"], split_nums("12").unwrap());
}

#[test]
fn split_nums_vars_simple() {
    use parse_eq::lexer::split_nums;
    assert_eq!(vec!["1", "x"], split_nums("1x").unwrap());
}

#[test]
fn split_nums_vars() {
    use parse_eq::lexer::split_nums;
    assert_eq!(vec!["12", "x"], split_nums("12x").unwrap());
    assert_eq!(vec!["123", "x"], split_nums("123x").unwrap());
    assert_eq!(vec!["1234", "x"], split_nums("1234x").unwrap());
    assert_eq!(vec!["12.3", "x"], split_nums("12.3x").unwrap());
}

#[test]
fn lexer_whitespace() {
    use parse_eq::lexer::Lexer;
    use parse_eq::token::Token;
    assert_eq!(vec![Token::Number(1.0)], Lexer::new_inorder("1").unwrap());
    assert_eq!(vec![Token::Number(1.0)], Lexer::new_inorder(" 1").unwrap());
    assert_eq!(vec![Token::Number(1.0)], Lexer::new_inorder("1 ").unwrap());
    assert_eq!(vec![Token::Number(12.0)], Lexer::new_inorder("12").unwrap());
    assert_eq!(
        vec![Token::Number(12.0)],
        Lexer::new_inorder(" 12").unwrap()
    );
    assert_eq!(
        vec![Token::Number(12.0)],
        Lexer::new_inorder("12 ").unwrap()
    );
}

#[test]
fn lexer_decimal() {
    use parse_eq::lexer::Lexer;
    use parse_eq::token::Token;
    assert_eq!(vec![Token::Number(1.0)], Lexer::new_inorder("1.0").unwrap());
    assert_eq!(vec![Token::Number(1.1)], Lexer::new_inorder("1.1").unwrap());
    assert_eq!(vec![Token::Number(0.1)], Lexer::new_inorder("0.1").unwrap());
}

#[test]
fn lexer_token_simple() {
    use parse_eq::lexer::Lexer;
    use parse_eq::token::Token;
    assert_eq!(vec![Token::LParen], Lexer::new_inorder("(").unwrap());
    assert_eq!(vec![Token::RParen], Lexer::new_inorder(")").unwrap());
}

#[test]
fn lexer_operator_simple() {
    use parse_eq::lexer::Lexer;
    use parse_eq::token::Operator::*;
    use parse_eq::token::Token;
    assert_eq!(vec![Token::Op(Add)], Lexer::new_inorder("+").unwrap());
    assert_eq!(vec![Token::Op(Subtract)], Lexer::new_inorder("-").unwrap());
    assert_eq!(vec![Token::Op(Multiply)], Lexer::new_inorder("*").unwrap());
    assert_eq!(vec![Token::Op(Divide)], Lexer::new_inorder("/").unwrap());
}

#[test]
fn lexer_operator() {
    use parse_eq::lexer::Lexer;
    use parse_eq::token::Operator::*;
    use parse_eq::token::Token;
    use parse_eq::token::Token::*;

    let add = vec![Number(1.0), Op(Add), Number(2.0)];
    let subtract = vec![Number(1.0), Op(Subtract), Number(2.0)];
    let multiply = vec![Number(1.0), Op(Multiply), Number(2.0)];
    let divide = vec![Number(1.0), Op(Divide), Number(2.0)];

    assert_eq!(add, Lexer::new_inorder("1+2").unwrap());
    assert_eq!(subtract, Lexer::new_inorder("1-2").unwrap());
    assert_eq!(multiply, Lexer::new_inorder("1*2").unwrap());
    assert_eq!(divide, Lexer::new_inorder("1/2").unwrap());
}

#[test]
fn lexer_operator_parentheses() {
    use parse_eq::lexer::Lexer;
    use parse_eq::token::Operator::*;
    use parse_eq::token::Token;
    use parse_eq::token::Token::*;

    let add = vec![LParen, Number(1.0), Op(Add), Number(2.0), RParen];
    let subtract = vec![LParen, Number(1.0), Op(Subtract), Number(2.0), RParen];
    let multiply = vec![LParen, Number(1.0), Op(Multiply), Number(2.0), RParen];
    let divide = vec![LParen, Number(1.0), Op(Divide), Number(2.0), RParen];

    assert_eq!(add, Lexer::new_inorder("(1+2)").unwrap());
    assert_eq!(subtract, Lexer::new_inorder("(1-2)").unwrap());
    assert_eq!(multiply, Lexer::new_inorder("(1*2)").unwrap());
    assert_eq!(divide, Lexer::new_inorder("(1/2)").unwrap());
}

#[test]
fn lexer_operator_parentheses_whitespace() {
    use parse_eq::lexer::Lexer;
    use parse_eq::token::Operator::*;
    use parse_eq::token::Token;
    use parse_eq::token::Token::*;

    let add = vec![LParen, Number(1.0), Op(Add), Number(2.0), RParen];

    assert_eq!(add, Lexer::new_inorder("(1+2)").unwrap());
    assert_eq!(add, Lexer::new_inorder(" (1+2)").unwrap());
    assert_eq!(add, Lexer::new_inorder("(1+2) ").unwrap());
    assert_eq!(add, Lexer::new_inorder(" (1+2) ").unwrap());
    assert_eq!(add, Lexer::new_inorder(" ( 1+2) ").unwrap());
    assert_eq!(add, Lexer::new_inorder(" ( 1 +2) ").unwrap());
    assert_eq!(add, Lexer::new_inorder(" ( 1 + 2) ").unwrap());
    assert_eq!(add, Lexer::new_inorder(" ( 1 + 2 ) ").unwrap());
    assert_eq!(add, Lexer::new_inorder("       ( 1 + 2 ) ").unwrap());
    assert_eq!(
        add,
        Lexer::new_inorder("       (         1 + 2 ) ").unwrap()
    );
    assert_eq!(
        add,
        Lexer::new_inorder("       (         1       + 2 ) ").unwrap()
    );
}
