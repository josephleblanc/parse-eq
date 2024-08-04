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
