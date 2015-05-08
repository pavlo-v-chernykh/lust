#[macro_use]
extern crate lust;

use lust::{Parser, State};

#[test]
fn test_macro_expansion() {
    let ref mut state = State::new("user".to_string());
    let input = "(def m (macro [a] '(+ 1 ~a)))";
    let expr = state.eval(&Parser::new(input.chars())
                                  .next().unwrap().ok().unwrap())
                    .ok().unwrap();
    assert_eq!(expr, n_macro![[n_symbol!["a"]],
                              [n_call!["quote",
                                       n_list![n_symbol!["+"],
                                               n_number![1.],
                                               n_call!["unquote",
                                                       n_symbol!["a"]]]]]]);
    let input = "(def f (fn [b] (m b)))";
    let expr = state.eval(&Parser::new(input.chars())
                                  .next().unwrap().ok().unwrap())
                    .ok().unwrap();
    assert_eq!(expr, n_fn![[n_symbol!["b"]],
                           [n_call!["+",
                                    n_number![1.],
                                    n_symbol!["b"]]]]);
}

#[test]
fn test_namespaced_syntax_quoted_macro_expansion() {
    let ref mut state = State::new("user".to_string());
    let switch_to_namespace_other = "(in-ns 'other)";
    state.eval(&Parser::new(switch_to_namespace_other.chars())
                       .next().unwrap().ok().unwrap())
         .ok().unwrap();
    let define_symbol_a = "(def a 1)";
    state.eval(&Parser::new(define_symbol_a.chars())
                       .next().unwrap().ok().unwrap())
         .ok().unwrap();
    let define_macro_m = "(def m (macro [b] `(+ a ~b)))";
    state.eval(&Parser::new(define_macro_m.chars())
                       .next().unwrap().ok().unwrap())
         .ok().unwrap();
    let switch_to_namespace_user = "(in-ns 'user)";
    state.eval(&Parser::new(switch_to_namespace_user.chars())
                       .next().unwrap().ok().unwrap())
         .ok().unwrap();
    let call_macro_as_function = "(other/m 3)";
    let result = state.eval(&Parser::new(call_macro_as_function.chars())
                                    .next().unwrap().ok().unwrap())
                      .ok().unwrap();
    assert_eq!(result, n_number![4.]);
}

#[test]
fn test_features() {
    let ref mut state = State::new("user".to_string());
    let input = include_str!("./test.ls");
    for expr in Parser::new(input.chars()) {
        state.eval(&expr.unwrap_or_else(|e| panic!("{}", e)))
             .unwrap_or_else(|e| panic!("{}", e));
    }
}
