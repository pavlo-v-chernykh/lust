use super::Parser;

#[test]
fn test_parse_number() {
    let mut parser = Parser::new("1".chars());
    assert_eq!(n_number![1.],
               parser.next().unwrap().ok().unwrap())
}

#[test]
fn test_parse_string() {
    let s = "rust we're looking forward on beta";
    let input = format!(r#""{}""#, s);
    let mut parser = Parser::new(input.chars());
    assert_eq!(n_string![s],
               parser.next().unwrap().ok().unwrap())
}

#[test]
fn test_parse_symbol() {
    let s = "symbol";
    let mut parser = Parser::new(s.chars());
    assert_eq!(n_symbol![s],
               parser.next().unwrap().ok().unwrap())
}

#[test]
fn test_parse_ns_qualified_symbol() {
    let ns = "my-ns";
    let s = "symbol";
    let ns_qualified_symbol = format!("{}/{}", ns, s);
    let mut parser = Parser::new(ns_qualified_symbol.chars());
    assert_eq!(n_symbol![Some(ns.to_string()), s],
               parser.next().unwrap().ok().unwrap())
}

#[test]
fn test_parse_keyword() {
    let mut parser = Parser::new(":keyword".chars());
    assert_eq!(n_keyword!["keyword"],
               parser.next().unwrap().ok().unwrap())
}

#[test]
fn test_parse_ns_qualified_keyword() {
    let mut parser = Parser::new(":my-ns/keyword".chars());
    assert_eq!(n_keyword![Some("my-ns".to_string()), "keyword"],
               parser.next().unwrap().ok().unwrap())
}

#[test]
fn test_parse_list_expression() {
    let expected_result = n_list![n_symbol!("def"),
                                  n_symbol!("a"),
                                  n_number!(1_f64)];
    let mut parser = Parser::new("(def a 1)".chars());
    let actual_result = parser.next().unwrap().ok().unwrap();
    assert_eq!(expected_result, actual_result);
}

#[test]
fn test_parse_vec_expression() {
    let expected_result = n_vec![n_keyword!("1"),
                                 n_keyword!("2"),
                                 n_keyword!("3")];
    let mut parser = Parser::new("[:1 :2 :3]".chars());
    let actual_result = parser.next().unwrap().ok().unwrap();
    assert_eq!(expected_result, actual_result);
}

#[test]
fn test_parse_nested_list_expressions() {
    let expected_result =  n_list![n_symbol!("def"),
                                   n_symbol!("a"),
                                   n_list![n_symbol!("+"),
                                           n_number!(1_f64),
                                           n_number!(2_f64)]];
    let mut parser = Parser::new("(def a (+ 1 2))".chars());
    let actual_result = parser.next().unwrap().ok().unwrap();
    assert_eq!(expected_result, actual_result);
}

#[test]
fn test_parse_nested_vec_expressions() {
    let expected_result =  n_vec![n_number!(1.),
                                  n_number!(2.),
                                  n_list![n_symbol!("+"),
                                          n_number!(1_f64),
                                          n_number!(2_f64)],
                                  n_keyword!["k"]];
    let mut parser = Parser::new("[1 2 (+ 1 2) :k]".chars());
    let actual_result = parser.next().unwrap().ok().unwrap();
    assert_eq!(expected_result, actual_result);
}

#[test]
fn test_parse_quoted_list() {
    let expected_result =  n_list![n_symbol!["quote"],
                                   n_list![n_symbol!["+"],
                                           n_number![1.],
                                           n_number![2.]]];
    let mut parser = Parser::new("'(+ 1 2)".chars());
    let actual_result = parser.next().unwrap().ok().unwrap();
    assert_eq!(expected_result, actual_result);
}

#[test]
fn test_parse_unquoted_symbol() {
    let expected_result =  n_list![n_symbol!["unquote"],
                                   n_symbol!["symbol"]];
    let mut parser = Parser::new("~symbol".chars());
    let actual_result = parser.next().unwrap().ok().unwrap();
    assert_eq!(expected_result, actual_result);
}

#[test]
fn test_parse_unquoted_splicing_list() {
    let expected_result =  n_list![n_symbol!["unquote-splicing"],
                                   n_list![n_number![1.],
                                           n_number![2.],
                                           n_number![3.]]];
    let mut parser = Parser::new("~@(1 2 3)".chars());
    let actual_result = parser.next().unwrap().ok().unwrap();
    assert_eq!(expected_result, actual_result);
}

#[test]
fn test_parse_syntax_quoted_list() {
    let expected_result = n_list![n_symbol!["syntax-quote"],
                                  n_list![n_symbol!["+"],
                                          n_number![1.],
                                          n_number![2.]]];
    let mut parser = Parser::new("`(+ 1 2)".chars());
    assert_eq!(expected_result, parser.next().unwrap().ok().unwrap());
}
