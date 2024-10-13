#[cfg(test)]
#[test]
fn split_nums_simple() {
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
    assert_eq!(
        vec![Token::Number(1.0)],
        Lexer::new_inorder("1").unwrap().list
    );
    assert_eq!(
        vec![Token::Number(1.0)],
        Lexer::new_inorder(" 1").unwrap().list
    );
    assert_eq!(
        vec![Token::Number(1.0)],
        Lexer::new_inorder("1 ").unwrap().list
    );
    assert_eq!(
        vec![Token::Number(12.0)],
        Lexer::new_inorder("12").unwrap().list
    );
    assert_eq!(
        vec![Token::Number(12.0)],
        Lexer::new_inorder(" 12").unwrap().list
    );
    assert_eq!(
        vec![Token::Number(12.0)],
        Lexer::new_inorder("12 ").unwrap().list
    );
}

#[test]
fn lexer_decimal() {
    use parse_eq::lexer::Lexer;
    use parse_eq::token::Token;
    assert_eq!(
        vec![Token::Number(1.0)],
        Lexer::new_inorder("1.0").unwrap().list
    );
    assert_eq!(
        vec![Token::Number(1.1)],
        Lexer::new_inorder("1.1").unwrap().list
    );
    assert_eq!(
        vec![Token::Number(0.1)],
        Lexer::new_inorder("0.1").unwrap().list
    );
}

#[test]
fn lexer_token_simple() {
    use parse_eq::lexer::Lexer;
    use parse_eq::token::Token;
    assert_eq!(vec![Token::LParen], Lexer::new_inorder("(").unwrap().list);
    assert_eq!(vec![Token::RParen], Lexer::new_inorder(")").unwrap().list);
}

#[test]
fn lexer_operator_simple() {
    use parse_eq::lexer::Lexer;
    use parse_eq::token::Operator::*;
    use parse_eq::token::Token;
    assert_eq!(vec![Token::Op(Add)], Lexer::new_inorder("+").unwrap().list);
    assert_eq!(
        vec![Token::Op(Subtract)],
        Lexer::new_inorder("-").unwrap().list
    );
    assert_eq!(
        vec![Token::Op(Multiply)],
        Lexer::new_inorder("*").unwrap().list
    );
    assert_eq!(
        vec![Token::Op(Divide)],
        Lexer::new_inorder("/").unwrap().list
    );
}

#[test]
fn lexer_variable_simple() {
    use parse_eq::lexer::Lexer;
    use parse_eq::token::Token;
    use parse_eq::token::Variable;
    assert_eq!(
        vec![Token::Var(Variable::X)],
        Lexer::new_inorder("x").unwrap().list
    );
    assert_eq!(
        vec![Token::Var(Variable::Y)],
        Lexer::new_inorder("y").unwrap().list
    );
    assert_eq!(
        vec![Token::Var(Variable::Z)],
        Lexer::new_inorder("z").unwrap().list
    );
    assert_ne!(
        vec![Token::Var(Variable::X)],
        Lexer::new_inorder("y").unwrap().list
    );
}

#[test]
fn lexer_variable_parens() {
    use parse_eq::lexer::Lexer;
    use parse_eq::token::Token;
    use parse_eq::token::Variable;
    assert_eq!(
        vec![Token::LParen, Token::Var(Variable::X), Token::RParen],
        Lexer::new_inorder("(x)").unwrap().list
    );
    assert_eq!(
        vec![
            Token::LParen,
            Token::LParen,
            Token::Var(Variable::X),
            Token::RParen,
            Token::RParen
        ],
        Lexer::new_inorder("((x))").unwrap().list
    );
    assert_eq!(
        vec![
            Token::LParen,
            Token::LParen,
            Token::LParen,
            Token::LParen,
            Token::Var(Variable::Y),
            Token::RParen,
            Token::RParen,
            Token::RParen,
            Token::RParen
        ],
        Lexer::new_inorder("(( ((y)) ) )").unwrap().list
    );
}

#[test]
fn lexer_variable_operator() {
    use parse_eq::lexer::Lexer;
    use parse_eq::token::Operator::*;
    use parse_eq::token::Token::*;
    use parse_eq::token::Variable;
    assert_eq!(
        vec![Var(Variable::X), Op(Add), Number(1.0)],
        Lexer::new_inorder("x+1").unwrap().list
    );
    assert_eq!(
        vec![LParen, Var(Variable::X), RParen, Op(Add), Number(1.0)],
        Lexer::new_inorder("(x)+1").unwrap().list
    );
    assert_eq!(
        vec![Var(Variable::X), Op(Add), LParen, Number(1.0), RParen],
        Lexer::new_inorder("x+(1)").unwrap().list
    );
    assert_eq!(
        vec![LParen, Var(Variable::X), Op(Add), Number(1.0), RParen],
        Lexer::new_inorder("(x+1)").unwrap().list
    );
}

#[test]
fn lexer_operator() {
    use parse_eq::lexer::Lexer;
    use parse_eq::token::Operator::*;
    use parse_eq::token::Token::*;

    let add = vec![Number(1.0), Op(Add), Number(2.0)];
    let subtract = vec![Number(1.0), Op(Subtract), Number(2.0)];
    let multiply = vec![Number(1.0), Op(Multiply), Number(2.0)];
    let divide = vec![Number(1.0), Op(Divide), Number(2.0)];

    assert_eq!(add, Lexer::new_inorder("1+2").unwrap().list);
    assert_eq!(subtract, Lexer::new_inorder("1-2").unwrap().list);
    assert_eq!(multiply, Lexer::new_inorder("1*2").unwrap().list);
    assert_eq!(divide, Lexer::new_inorder("1/2").unwrap().list);
}

#[test]
fn lexer_operator_parentheses() {
    use parse_eq::lexer::Lexer;
    use parse_eq::token::Operator::*;
    use parse_eq::token::Token::*;

    let add = vec![LParen, Number(1.0), Op(Add), Number(2.0), RParen];
    let subtract = vec![LParen, Number(1.0), Op(Subtract), Number(2.0), RParen];
    let multiply = vec![LParen, Number(1.0), Op(Multiply), Number(2.0), RParen];
    let divide = vec![LParen, Number(1.0), Op(Divide), Number(2.0), RParen];

    assert_eq!(add, Lexer::new_inorder("(1+2)").unwrap().list);
    assert_eq!(subtract, Lexer::new_inorder("(1-2)").unwrap().list);
    assert_eq!(multiply, Lexer::new_inorder("(1*2)").unwrap().list);
    assert_eq!(divide, Lexer::new_inorder("(1/2)").unwrap().list);
}

#[test]
fn lexer_operator_parentheses_whitespace() {
    use parse_eq::lexer::Lexer;
    use parse_eq::token::Operator::*;
    use parse_eq::token::Token::*;

    let add = vec![LParen, Number(1.0), Op(Add), Number(2.0), RParen];

    assert_eq!(add, Lexer::new_inorder("(1+2)").unwrap().list);
    assert_eq!(add, Lexer::new_inorder(" (1+2)").unwrap().list);
    assert_eq!(add, Lexer::new_inorder("(1+2) ").unwrap().list);
    assert_eq!(add, Lexer::new_inorder(" (1+2) ").unwrap().list);
    assert_eq!(add, Lexer::new_inorder(" ( 1+2) ").unwrap().list);
    assert_eq!(add, Lexer::new_inorder(" ( 1 +2) ").unwrap().list);
    assert_eq!(add, Lexer::new_inorder(" ( 1 + 2) ").unwrap().list);
    assert_eq!(add, Lexer::new_inorder(" ( 1 + 2 ) ").unwrap().list);
    assert_eq!(add, Lexer::new_inorder("       ( 1 + 2 ) ").unwrap().list);
    assert_eq!(
        add,
        Lexer::new_inorder("       (         1 + 2 ) ")
            .unwrap()
            .list
    );
    assert_eq!(
        add,
        Lexer::new_inorder("       (         1       + 2 ) ")
            .unwrap()
            .list
    );
}

#[test]
fn lexer_unop_trig() {
    use parse_eq::lexer::Lexer;
    use parse_eq::token::Token::*;
    use parse_eq::token::UnaryOperator::*;

    let sine = vec![UnOp(Sine)];
    assert_eq!(sine, Lexer::new_inorder("sin").unwrap().list);
    let cosine = vec![UnOp(Cosine)];
    assert_eq!(cosine, Lexer::new_inorder("cos").unwrap().list);
    let tangent = vec![UnOp(Tangent)];
    assert_eq!(tangent, Lexer::new_inorder("tan").unwrap().list);
}

#[test]
fn lexer_unop_negation() {
    use parse_eq::lexer::Lexer;
    use parse_eq::token::Operator::*;
    use parse_eq::token::Token::*;
    use parse_eq::token::UnaryOperator::*;
    use parse_eq::token::Variable;

    let neg_one = vec![UnOp(Negation), Number(1.0)];
    assert_eq!(neg_one, Lexer::new_inorder("-1").unwrap().list);

    let double_neg = vec![UnOp(Negation), Number(1.0), Op(Add), Number(2.0)];
    assert_eq!(double_neg, Lexer::new_inorder("-1 + 2").unwrap().list);

    let trip_neg = vec![
        UnOp(Negation),
        Number(1.0),
        Op(Add),
        UnOp(Negation),
        Number(2.0),
    ];
    assert_eq!(trip_neg, Lexer::new_inorder("-1 + -2").unwrap().list);

    let trip_neg_var = vec![
        UnOp(Negation),
        Number(1.0),
        Op(Add),
        UnOp(Negation),
        Var(Variable::X),
    ];
    assert_eq!(trip_neg_var, Lexer::new_inorder("-1 + -x").unwrap().list);

    let check_vec = vec![
        UnOp(Negation),
        Number(2.0),
        Op(Subtract),
        LParen,
        Number(5.0),
        Op(Multiply),
        UnOp(Negation),
        UnOp(Sine),
        UnOp(Negation),
        Var(Variable::X),
        RParen,
        Op(Add),
        Number(1.0),
    ];
    let lexer = Lexer::new_inorder("-2 - (5 * -sin -x ) + 1").unwrap();
    assert_eq!(check_vec, lexer.list);
}
