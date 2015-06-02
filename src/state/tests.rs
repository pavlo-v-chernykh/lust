use super::State;
use super::error::EvalError::*;
use ast::nodes::Symbol;

#[test]
fn test_insert_to_and_get_from_root_state() {
    let mut state = State::new("user".to_string());
    let key = "rust-is-terrific".to_string();
    let val = n_number!(10.5);
    state.insert(Symbol::new(None, key.clone()), val.clone());
    assert_eq!(&val, state.get(&Symbol::new(None, key.clone())).unwrap());
}

#[test]
fn test_insert_to_and_get_from_child_state() {
    let root_state = State::new("user".to_string());
    let mut state = State::new_chained(&root_state);
    let key = "rust-is-terrific".to_string();
    let val = n_number!(10.5);
    state.insert(Symbol::new(None, key.clone()), val.clone());
    assert_eq!(&val, state.get(&Symbol::new(None, key.clone())).unwrap());
}

#[test]
fn test_insert_to_root_state_and_get_from_child_state() {
    let mut root_state = State::new("user".to_string());
    let key = "rust-is-terrific".to_string();
    let val = n_number!(10.5);
    root_state.insert(Symbol::new(None, key.clone()), val.clone());
    let state = State::new_chained(&root_state);
    assert_eq!(&val, state.get(&Symbol::new(None, key.clone())).unwrap());
}

#[test]
fn test_insert_to_child_state_and_get_none_from_root_state() {
    let root_state = State::new("user".to_string());
    let key = "rust-is-terrific".to_string();
    let val = n_number!(10.5);
    let mut state = State::new_chained(&root_state);
    state.insert(Symbol::new(None, key.clone()), val.clone());
    assert!(root_state.get(&Symbol::new(None, key.clone())).is_none());
}

#[test]
fn test_shadow_val_in_root_state() {
    let mut root_state = State::new("user".to_string());
    let key = "rust-is-terrific".to_string();
    let val_in_root = n_number!(10.5);
    root_state.insert(Symbol::new(None, key.clone()), val_in_root);
    let val_in_child = n_number!(0_f64);
    let mut state = State::new_chained(&root_state);
    state.insert(Symbol::new(None, key.clone()), val_in_child.clone());
    assert_eq!(&val_in_child, state.get(&Symbol::new(None, key.clone())).unwrap());
}

#[test]
fn test_insert_to_and_get_from_root_state_with_namespace() {
    let mut state = State::new("user".to_string());
    let key = "rust-is-terrific".to_string();
    let val = n_number!(10.5);
    state.insert(Symbol::new(None, key.clone()), val.clone());
    assert_eq!(&val, state.get(&Symbol::new(Some("user".to_string()), key.clone())).unwrap());
}

#[test]
fn test_insert_to_and_get_from_child_state_with_namespace() {
    let root_state = State::new("user".to_string());
    let mut state = State::new_chained(&root_state);
    let key = "rust-is-terrific".to_string();
    let val = n_number!(10.5);
    state.insert(Symbol::new(None, key.clone()), val.clone());
    assert_eq!(&val, state.get(&Symbol::new(Some("user_chained".to_string()), key.clone())).unwrap());
}

#[test]
fn test_insert_to_root_state_and_get_from_child_state_with_namespace() {
    let mut root_state = State::new("user".to_string());
    let key = "rust-is-terrific".to_string();
    let val = n_number!(10.5);
    root_state.insert(Symbol::new(None, key.clone()), val.clone());
    let state = State::new_chained(&root_state);
    assert_eq!(&val, state.get(&Symbol::new(Some("user".to_string()), key.clone())).unwrap());
}

#[test]
fn test_insert_to_child_state_and_get_none_from_root_state_with_namespace() {
    let root_state = State::new("user".to_string());
    let key = "rust-is-terrific".to_string();
    let val = n_number!(10.5);
    let mut state = State::new_chained(&root_state);
    state.insert(Symbol::new(None, key.clone()), val.clone());
    assert!(root_state.get(&Symbol::new(Some("user".to_string()), key.clone())).is_none());
}

#[test]
fn test_shadow_val_in_root_state_with_namespace() {
    let mut root_state = State::new("user".to_string());
    let key = "rust-is-terrific".to_string();
    let val_in_root = n_number!(10.5);
    root_state.insert(Symbol::new(None, key.clone()), val_in_root);
    let val_in_child = n_number!(0_f64);
    let mut state = State::new_chained(&root_state);
    state.insert(Symbol::new(None, key.clone()), val_in_child.clone());
    assert_eq!(&val_in_child, state.get(&Symbol::new(None, key.clone())).unwrap());
}

#[test]
fn test_expand_number() {
    let ref mut state = State::new("user".to_string());
    let num = 1_f64;
    assert_eq!(n_number!(num), state.expand(&n_number!(num)).ok().unwrap());
}

#[test]
fn test_expand_string() {
    let ref mut state = State::new("user".to_string());
    let s = "rust is wonderful";
    assert_eq!(n_string!(s), state.expand(&n_string!(s)).ok().unwrap());
}

#[test]
fn test_expand_symbol() {
    let ref mut state = State::new("user".to_string());
    let s = "+";
    assert_eq!(n_symbol!(s), state.expand(&n_symbol!(s)).ok().unwrap());
}

#[test]
fn test_expand_keyword() {
    let ref mut state = State::new("user".to_string());
    let s = ":+";
    assert_eq!(n_keyword!(s), state.expand(&n_keyword!(s)).ok().unwrap());
}

#[test]
fn test_expand_empty_list() {
    let ref mut state = State::new("user".to_string());
    assert_eq!(n_list![], state.expand(&n_list![]).ok().unwrap());
}

#[test]
fn test_expand_fn() {
    let ref mut state = State::new("user".to_string());
    let e = n_fn!([n_symbol!("a")],
                  [n_call!["+", vec![n_symbol!("a"), n_number!(1_f64)]]]);
    let n = n_list![vec![n_symbol!("fn"),
                         n_vec![vec![n_symbol!("a")]],
                         n_list![vec![n_symbol!("+"), n_symbol!("a"), n_number!(1_f64)]]]];
    assert_eq!(e, state.expand(&n).ok().unwrap());
}

#[test]
fn test_expand_macro() {
    let ref mut state = State::new("user".to_string());
    let e = n_macro!([n_symbol!("a")],
                     [n_call!["+", vec![n_symbol!("a"), n_number!(1_f64)]]]);
    let n = n_list![vec![n_symbol!("macro"),
                         n_vec![vec![n_symbol!("a")]],
                         n_list![vec![n_symbol!("+"), n_symbol!("a"), n_number!(1_f64)]]]];
    assert_eq!(e, state.expand(&n).ok().unwrap());
}

#[test]
fn test_expand_def() {
    let ref mut state = State::new("user".to_string());
    let e = n_def!["a", n_number![1_f64]];
    let n = n_list![vec![n_symbol!["def"], n_symbol!["a"], n_number![1_f64]]];
    assert_eq!(e, state.expand(&n).ok().unwrap());
}

#[test]
fn test_expand_call_fn() {
    let ref mut state = State::new("user".to_string());
    let e = n_call!["+", vec![n_symbol!["a"], n_number![1_f64]]];
    let n = n_list![vec![n_symbol!["+"], n_symbol!["a"], n_number![1_f64]]];
    assert_eq!(e, state.expand(&n).ok().unwrap());
}

#[test]
fn test_expand_call_macro() {
    let ref mut state = State::new("user".to_string());
    let m = n_def!["m", n_macro![[n_symbol!["a"], n_symbol!["b"]],
                                 [n_call!["+", vec![n_symbol!["a"], n_symbol!["b"]]]]]];
    state.eval(&m).ok().unwrap();
    let e = n_number![3.];
    let n = n_list![vec![n_symbol!["m"], n_number![1.], n_number![2.]]];
    assert_eq!(e, state.expand(&n).ok().unwrap());
}

#[test]
fn test_expand_quote() {
    let ref mut state = State::new("user".to_string());
    let n = n_list![vec![n_symbol!["quote"], n_symbol!["a"]]];
    assert_eq!(n_call!["quote", vec![n_symbol!["a"]]], state.expand(&n).ok().unwrap());
    let n = n_list![vec![n_symbol!["quote"],
                         n_list![vec![n_symbol!["+"], n_symbol!["a"], n_symbol!["b"]]]]];
    assert_eq!(n_call!["quote", vec![n_list![vec![n_symbol!["+"],
                                                  n_symbol!["a"],
                                                  n_symbol!["b"]]]]],
               state.expand(&n).ok().unwrap());
}

#[test]
fn test_expand_unquote() {
    let ref mut state = State::new("user".to_string());
    let n = n_list![vec![n_symbol!["quote"], n_list![vec![n_symbol!["a"],
                                                     n_list![vec![n_symbol!["unquote"],
                                                                  n_symbol!["b"]]]]]]];
    let expected_result = n_call!["quote", vec![n_list![vec![n_symbol!["a"],
                                                             n_call!["unquote",
                                                                     vec![n_symbol!["b"]]]]]]];
    assert_eq!(expected_result, state.expand(&n).ok().unwrap());
}

#[test]
fn test_expand_unquote_splicing() {
    let ref mut state = State::new("user".to_string());
    let n = n_list![vec![n_symbol!["quote"], n_list![vec![n_symbol!["a"],
                                                          n_list![vec![n_symbol!["unquote-splicing"],
                                                                       n_symbol!["b"]]]]]]];
    let expected_result = n_call!["quote", vec![n_list![vec![n_symbol!["a"],
                                                             n_call!["unquote-splicing",
                                                                     vec![n_symbol!["b"]]]]]]];
    assert_eq!(expected_result, state.expand(&n).ok().unwrap());
}

#[test]
fn test_expand_let() {
    let ref mut state = State::new("user".to_string());
    let n = n_list![vec![n_symbol!["let"], n_vec![vec![n_symbol!["a"],
                                                       n_list![vec![n_symbol!["+"],
                                                                    n_number![1.],
                                                                    n_number![2.]]],
                                                       n_symbol!["b"],
                                                       n_list![vec![n_symbol!["+"],
                                                                    n_symbol!["a"],
                                                                    n_number![3.]]]]],
                                      n_list![vec![n_symbol!["+"],
                                                   n_symbol!["a"],
                                                   n_symbol!["b"]]]]];
    let expected_result = n_let![[n_symbol!["a"], n_call!["+", vec![n_number![1.], n_number![2.]]],
                                  n_symbol!["b"], n_call!["+", vec![n_symbol!["a"],
                                                                    n_number![3.]]]],
                                 n_call!["+", vec![n_symbol!["a"], n_symbol!["b"]]]];
    assert_eq![expected_result, state.expand(&n).ok().unwrap()];
}

#[test]
fn test_eval_number_to_itself() {
    let num = 10_f64;
    let ref mut state = State::new("user".to_string());
    let expected_result = n_number!(num);
    let actual_result = state.eval(&n_number!(num));
    assert_eq!(expected_result, actual_result.ok().unwrap());
}

#[test]
fn test_eval_keyword_to_itself() {
    let keyword = ":keyword";
    let ref mut state = State::new("user".to_string());
    let expected_result = n_keyword!(keyword);
    let actual_result = state.eval(&n_keyword!(keyword));
    assert_eq!(expected_result, actual_result.ok().unwrap());
}

#[test]
fn test_eval_string_to_itself() {
    let s = "rust is awesome";
    let ref mut state = State::new("user".to_string());
    let expected_result = n_string!(s);
    let actual_result = state.eval(&n_string!(s));
    assert_eq!(expected_result, actual_result.ok().unwrap());
}

#[test]
fn test_eval_undefined_symbol_to_error() {
    let ref mut state = State::new("user".to_string());
    let expected_result = ResolveError("a".to_string());
    let actual_result = state.eval(&n_symbol!("a"));
    assert_eq!(expected_result, actual_result.err().unwrap());
}

#[test]
fn test_eval_true_to_matching_bool() {
    let ref mut state = State::new("user".to_string());
    let expected_result = n_bool!(true);
    let actual_result = state.eval(&n_symbol!("true"));
    assert_eq!(expected_result, actual_result.ok().unwrap());
}

#[test]
fn test_eval_false_to_matching_bool() {
    let ref mut state = State::new("user".to_string());
    let expected_result = n_bool!(false);
    let actual_result = state.eval(&n_symbol!("false"));
    assert_eq!(expected_result, actual_result.ok().unwrap());
}

#[test]
fn test_eval_nil_to_empty_list() {
    let ref mut state = State::new("user".to_string());
    let expected_result = n_list![];
    let actual_result = state.eval(&n_symbol!("nil"));
    assert_eq!(expected_result, actual_result.ok().unwrap());
}

#[test]
fn test_eval_def_special_form() {
    let num = 1_f64;
    let ref mut state = State::new("user".to_string());
    let expected_result = n_number!(num);
    let actual_input = &n_def!["a", n_number!(num)];
    let actual_result = state.eval(&actual_input);
    assert_eq!(expected_result, actual_result.ok().unwrap());
}

#[test]
fn test_eval_let_special_form() {
    let ref mut state = State::new("user".to_string());
    let let_expr = n_let![[n_symbol!["a"], n_call!["+", vec![n_number![1.], n_number![2.]]],
                           n_symbol!["b"], n_call!["+", vec![n_symbol!["a"], n_number![3.]]]],
                          n_call!["+", vec![n_symbol!["a"], n_symbol!["b"]]]];
    assert_eq!(n_number![9.], state.eval(&let_expr).ok().unwrap());
}

#[test]
fn test_eval_fn_special_form_and_call_defined_function() {
    let ref mut state = State::new("user".to_string());
    let expr = &n_def!["add-skip-and-sub",
                       n_fn![[n_symbol!("a"), n_symbol!("b")],
                             [n_call!["+", vec![n_symbol!("a"), n_symbol!("b")]],
                              n_call!["-", vec![n_symbol!("a"), n_symbol!("b")]]]]];
    state.eval(&expr).ok().unwrap();
    let expected_result = n_number!(-1_f64);
    let actual_input = n_call!["add-skip-and-sub", vec![n_number!(1_f64), n_number!(2_f64)]];
    let actual_result = state.eval(&actual_input);
    assert_eq!(expected_result, actual_result.ok().unwrap());
}

#[test]
fn test_eval_macro_special_form_and_call_defined_macro() {
    let ref mut state = State::new("user".to_string());
    let expr = &n_def!["add",
                       n_macro![[n_symbol!("a"), n_symbol!("b")],
                                [n_call!["+", vec![n_symbol!("a"), n_symbol!("b")]]]]];
    state.eval(&expr).ok().unwrap();
    let expected_result = n_number!(3.);
    let actual_input = n_call!["add", vec![n_number!(1_f64), n_number!(2_f64)]];
    let actual_result = state.eval(&actual_input);
    assert_eq!(expected_result, actual_result.ok().unwrap());
}

#[test]
fn test_eval_fn_and_get_error_when_call_defined_function_with_incorrect_number_of_args() {
    let ref mut state = State::new("user".to_string());
    let actual_input = n_def!["add",
                              n_fn![[n_symbol!("a"), n_symbol!("b")],
                                    [n_call!["+", vec![n_symbol!("a"), n_symbol!("b")]]]]];
    state.eval(&actual_input).ok().unwrap();
    let expected_result = IncorrectNumberOfArgumentsError(n_call!["add", vec![n_number![1_f64]]]);
    let expr = &n_call!["add", vec![n_number!(1_f64)]];
    let mut actual_result = state.eval(&expr);
    assert_eq!(expected_result, actual_result.err().unwrap());
    let expr = &n_call!["add", vec![
                        n_number!(1_f64),
                        n_number!(1_f64),
                        n_number!(1_f64),
                        n_number!(1_f64)]];
    let expected_result = IncorrectNumberOfArgumentsError(n_call!["add", vec![
                                                                  n_number![1_f64],
                                                                  n_number![1_f64],
                                                                  n_number![1_f64],
                                                                  n_number![1_f64]]]);
    actual_result = state.eval(&expr);
    assert_eq!(expected_result, actual_result.err().unwrap());
}

#[test]
fn test_eval_plus_builtin_fn() {
    let ref mut state = State::new("user".to_string());
    let expr = &n_def!["a", n_number!(1_f64)];
    state.eval(expr).ok().unwrap();
    let expr = &n_def!["b", n_number!(2_f64)];
    state.eval(expr).ok().unwrap();
    let nested_call = n_call!["+", vec![n_symbol!("a"), n_symbol!("b")]];
    let actual_input = &n_call!["+", vec![nested_call, n_number!(3_f64)]];
    assert_eq!(n_number!(6_f64), state.eval(&actual_input).ok().unwrap());
}

#[test]
fn test_eval_minus_builtin_fn() {
    let ref mut state = State::new("user".to_string());
    let actual_input = &n_call!["-", vec![n_number!(3_f64), n_number!(2_f64)]];
    let actual_result = state.eval(&actual_input);
    let expected_result = n_number!(1_f64);
    assert_eq!(expected_result, actual_result.ok().unwrap());
}

#[test]
fn test_eval_div_builtin_fn() {
    let ref mut state = State::new("user".to_string());
    let actual_input = &n_call!["/", vec![n_number!(3_f64), n_number!(2_f64)]];
    let actual_result = state.eval(&actual_input);
    let expected_result = n_number!(1.5);
    assert_eq!(expected_result, actual_result.ok().unwrap());
}

#[test]
fn test_eval_mul_builtin_fn() {
    let ref mut state = State::new("user".to_string());
    let actual_input = n_call!["*", vec![n_number!(3.5), n_number!(2_f64)]];
    let actual_result = state.eval(&actual_input);
    let expected_result = n_number!(7_f64);
    assert_eq!(expected_result, actual_result.ok().unwrap());
}

#[test]
fn test_eval_lt_builtin_fn_positive_case() {
    let ref mut state = State::new("user".to_string());
    let actual_input = &n_call!["<", vec![n_number!(1_f64), n_number!(2_f64), n_number!(3_f64)]];
    let actual_result = state.eval(&actual_input);
    let expected_result = n_bool!(true);
    assert_eq!(expected_result, actual_result.ok().unwrap());
}

#[test]
fn test_eval_lt_builtin_fn_negative_case() {
    let ref mut state = State::new("user".to_string());
    let actual_input = n_call!["<", vec![n_number!(3.5), n_number!(20_f64), n_number!(1_f64)]];
    let actual_result = state.eval(&actual_input);
    let expected_result = n_bool!(false);
    assert_eq!(expected_result, actual_result.ok().unwrap());
}

#[test]
fn test_eval_gt_builtin_fn_positive_case() {
    let ref mut state = State::new("user".to_string());
    let expr = &n_def!["a", n_number!(3_f64)];
    state.eval(&expr).ok().unwrap();
    let actual_input = &n_call![">", vec![n_symbol!("a"), n_number!(2_f64), n_number!(1_f64)]];
    let actual_result = state.eval(&actual_input);
    let expected_result = n_bool!(true);
    assert_eq!(expected_result, actual_result.ok().unwrap());
}

#[test]
fn test_eval_gt_builtin_fn_negative_case() {
    let ref mut state = State::new("user".to_string());
    let expr = &n_def!["a", n_number!(20_f64)];
    state.eval(&expr).ok().unwrap();
    let actual_input = &n_call![">", vec![n_number!(3.5), n_symbol!("a"), n_number!(1_f64)]];
    let actual_result = state.eval(&actual_input);
    let expected_result = n_bool!(false);
    assert_eq!(expected_result, actual_result.ok().unwrap());
}

#[test]
fn test_eval_quote_builtin_fn() {
    let ref mut state = State::new("user".to_string());
    let expr = &n_call!["quote", vec![n_list![vec![n_symbol!["+"],
                                                   n_symbol!["true"],
                                                   n_number![1.]]]]];
    let expected_result = n_list![vec![n_symbol!["+"], n_symbol!["true"], n_number![1.]]];
    assert_eq!(expected_result, state.eval(&expr).ok().unwrap());
}

#[test]
fn test_eval_unquote_builtin_fn() {
    let ref mut state = State::new("user".to_string());
    state.insert(Symbol::new(None, "a".to_string()), n_number![3.]);
    let expr = n_call!["quote", vec![n_list![vec![n_symbol!["+"],
                                                  n_call!["unquote", vec![n_symbol!["a"]]],
                                                  n_number![1.]]]]];
    let expected_result = n_list![vec![n_symbol!["+"],
                                       n_number![3.],
                                       n_number![1.]]];
    assert_eq!(expected_result, state.eval(&expr).ok().unwrap());
}

#[test]
fn test_eval_unquote_splicing_builtin_fn() {
    let ref mut state = State::new("user".to_string());
    state.insert(Symbol::new(None, "a".to_string()),
                 n_list![vec![n_number![1.], n_number![2.], n_number![3.]]]);
    let expr = n_call!["quote", vec![n_list![vec![n_symbol!["+"],
                                                  n_call!["unquote-splicing", vec![n_symbol!["a"]]],
                                                  n_number![1.]]]]];
    let expected_result = n_list![vec![n_symbol!["+"],
                                       n_number![1.],
                                       n_number![2.],
                                       n_number![3.],
                                       n_number![1.]]];
    assert_eq!(expected_result, state.eval(&expr).ok().unwrap());
}

#[test]
fn test_eval_eval_builtin_fn() {
    let ref mut state = State::new("user".to_string());
    let expr = n_call!["eval", vec![n_list![vec![n_symbol!["+"],
                                                 n_number![1.],
                                                 n_number![2.]]]]];
    let expected_result = n_number![3.];
    assert_eq!(expected_result, state.eval(&expr).ok().unwrap());
    let expr = n_call!["eval", vec![n_call!["quote", vec![n_list![vec![n_symbol!["+"],
                                                                  n_number![1.],
                                                                  n_number![2.]]]]]]];
    let expected_result = n_number![3.];
    assert_eq!(expected_result, state.eval(&expr).ok().unwrap());
}

#[test]
fn test_eval_eq_builtin_fn_positive_case() {
    let ref mut state = State::new("user".to_string());
    let expr = &n_def!["a", n_number!(3_f64)];
    state.eval(&expr).ok().unwrap();
    let actual_input = &n_call!["=", vec![n_symbol!("a"), n_number!(3_f64), n_number!(3_f64)]];
    let actual_result = state.eval(&actual_input);
    let expected_result = n_bool!(true);
    assert_eq!(expected_result, actual_result.ok().unwrap());
}

#[test]
fn test_eval_eq_builtin_fn_negative_case() {
    let ref mut state = State::new("user".to_string());
    let expr = &n_def!["a", n_number![1_f64]];
    state.eval(&expr).ok().unwrap();
    let actual_input = &n_call!["=", vec![n_number![3.5], n_number![20_f64], n_symbol!["a"]]];
    let actual_result = state.eval(&actual_input);
    let expected_result = n_bool![false];
    assert_eq!(expected_result, actual_result.ok().unwrap());
}

#[test]
fn test_eval_if_builtin_fn() {
    let ref mut state = State::new("user".to_string());
    let ref actual_input = n_call!["if", vec![n_call!["=", vec![n_number![3.], n_number![3.]]],
                                              n_call!["+", vec![n_number![3.], n_number![3.]]],
                                              n_call!["-", vec![n_number![3.], n_number![3.]]]]];
    let actual_result = state.eval(&actual_input);
    let expected_result = n_number![6.];
    assert_eq!(expected_result, actual_result.ok().unwrap());
    let ref actual_input = n_call!["if", vec![n_call!["<", vec![n_number![3.], n_number![3.]]],
                                              n_call!["+", vec![n_number![3.], n_number![3.]]],
                                              n_call!["-", vec![n_number![3.], n_number![3.]]]]];
    let actual_result = state.eval(&actual_input);
    let expected_result = n_number![0.];
    assert_eq!(expected_result, actual_result.ok().unwrap());
}
