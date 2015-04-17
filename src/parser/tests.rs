use super::Parser;

#[test]
fn test_parse_number() {
    let mut parser = Parser::new("1".chars());
    assert_eq!(e_number![1.],
               parser.next().unwrap().ok().unwrap())
}

#[test]
fn test_parse_string() {
    let s = "rust we're looking forward on beta";
    let input = format!(r#""{}""#, s);
    let mut parser = Parser::new(input.chars());
    assert_eq!(e_string![s],
               parser.next().unwrap().ok().unwrap())
}

#[test]
fn test_parse_symbol() {
    let s = "symbol";
    let mut parser = Parser::new(s.chars());
    assert_eq!(e_symbol![s],
               parser.next().unwrap().ok().unwrap())
}

#[test]
fn test_parse_ns_qualified_symbol() {
    let ns = "my-ns";
    let s = "symbol";
    let ns_qualified_symbol = format!("{}/{}", ns, s);
    let mut parser = Parser::new(ns_qualified_symbol.chars());
    assert_eq!(e_symbol![ns, s],
               parser.next().unwrap().ok().unwrap())
}

#[test]
fn test_parse_keyword() {
    let k = ":keyword";
    let mut parser = Parser::new(k.chars());
    assert_eq!(e_keyword![k],
               parser.next().unwrap().ok().unwrap())
}

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
fn test_parse_vec_expression() {
    let expected_result = e_vec![e_keyword!(":1"),
                                 e_keyword!(":2"),
                                 e_keyword!(":3")];
    let mut parser = Parser::new("[:1 :2 :3]".chars());
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

#[test]
fn test_parse_nested_vec_expressions() {
    let expected_result =  e_vec![e_number!(1.),
                                  e_number!(2.),
                                  e_list![e_symbol!("+"),
                                          e_number!(1_f64),
                                          e_number!(2_f64)],
                                  e_keyword![":k"]];
    let mut parser = Parser::new("[1 2 (+ 1 2) :k]".chars());
    let actual_result = parser.next().unwrap().ok().unwrap();
    assert_eq!(expected_result, actual_result);
}

#[test]
fn test_parse_quoted_list() {
    let expected_result =  e_list![e_symbol!["quote"],
                                   e_list![e_symbol!["+"],
                                           e_number![1.],
                                           e_number![2.]]];
    let mut parser = Parser::new("'(+ 1 2)".chars());
    let actual_result = parser.next().unwrap().ok().unwrap();
    assert_eq!(expected_result, actual_result);
}

#[test]
fn test_parse_unquoted_symbol() {
    let expected_result =  e_list![e_symbol!["unquote"],
                                   e_symbol!["symbol"]];
    let mut parser = Parser::new("~symbol".chars());
    let actual_result = parser.next().unwrap().ok().unwrap();
    assert_eq!(expected_result, actual_result);
}

#[test]
fn test_parse_unquoted_splicing_list() {
    let expected_result =  e_list![e_symbol!["unquote-splicing"],
                                   e_list![e_number![1.],
                                           e_number![2.],
                                           e_number![3.]]];
    let mut parser = Parser::new("~@(1 2 3)".chars());
    let actual_result = parser.next().unwrap().ok().unwrap();
    assert_eq!(expected_result, actual_result);
}
