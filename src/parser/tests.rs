use super::Parser;

#[test]
fn test_parse_list_expression() {
    let expected_result = e_list![e_symbol!("def"),
                                  e_symbol!("a"),
                                  e_number!(1_f64)];
    let mut parser = Parser::new("(def a 1)".chars());
    let actual_result = parser.next().unwrap().ok().unwrap();
    assert_eq!(expected_result, actual_result);
}

#[test]
fn test_parse_nested_list_expressions() {
    let expected_result =  e_list![e_symbol!("def"),
                                   e_symbol!("a"),
                                   e_list![e_symbol!("+"),
                                           e_number!(1_f64),
                                           e_number!(2_f64)]];
    let mut parser = Parser::new("(def a (+ 1 2))".chars());
    let actual_result = parser.next().unwrap().ok().unwrap();
    assert_eq!(expected_result, actual_result);
}
