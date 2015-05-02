#[macro_use]
extern crate lust;

use lust::{Parser, State};

#[test]
fn test_macro_expansion() {
    let ref mut state = State::new("user".to_string());
    let input = "(def m (macro [a] '(+ 1 ~a)))";
    let expr = Parser::new(input.chars())
        .next().unwrap().ok().unwrap()
        .eval(state).ok().unwrap();
    assert_eq!(expr, e_macro![[e_symbol!["a"]],
                              [e_call!["quote",
                                       e_list![e_symbol!["+"],
                                               e_number![1.],
                                               e_call!["unquote",
                                                       e_symbol!["a"]]]]]]);
    let input = "(def f (fn [b] (m b)))";
    let expr = Parser::new(input.chars())
        .next().unwrap().ok().unwrap()
        .eval(state).ok().unwrap();
    assert_eq!(expr, e_fn![[e_symbol!["b"]],
                           [e_call!["+",
                                    e_number![1.],
                                    e_symbol!["b"]]]]);
}

#[test]
fn test_namespaced_syntax_quoted_macro_expansion() {
    let ref mut state = State::new("user".to_string());
    let switch_to_namespace_other = "(in-ns 'other)";
    Parser::new(switch_to_namespace_other.chars())
        .next().unwrap().ok().unwrap()
        .eval(state).ok().unwrap();
    let define_symbol_a = "(def a 1)";
    Parser::new(define_symbol_a.chars())
        .next().unwrap().ok().unwrap()
        .eval(state).ok().unwrap();
    let define_macro_m = "(def m (macro [b] `(+ a ~b)))";
    Parser::new(define_macro_m.chars())
        .next().unwrap().ok().unwrap()
        .eval(state).ok().unwrap();
    let switch_to_namespace_user = "(in-ns 'user)";
    Parser::new(switch_to_namespace_user.chars())
        .next().unwrap().ok().unwrap()
        .eval(state).ok().unwrap();
    let call_macro_as_function = "(other/m 3)";
    let result = Parser::new(call_macro_as_function.chars())
        .next().unwrap().ok().unwrap()
        .eval(state).ok().unwrap();
    assert_eq!(result, e_number![4.]);
}

#[test]
fn test_features() {
    let ref mut state = State::new("user".to_string());
    let input = include_str!("./test.ls");
    for expr in Parser::new(input.chars()) {
        expr.unwrap_or_else(|e| panic!("{}", e))
            .eval(state)
            .unwrap_or_else(|e| panic!("{}", e));
    }
}
