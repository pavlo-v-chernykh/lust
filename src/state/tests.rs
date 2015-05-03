use super::State;
use super::error::EvalError::*;

#[test]
fn test_insert_to_and_get_from_root_state() {
    let mut state = State::new("user".to_string());
    let key = "rust-is-terrific".to_string();
    let val = e_number!(10.5);
    state.insert(key.clone(), val.clone());
    assert_eq!(&val, state.get(None, &key).unwrap());
}

#[test]
fn test_insert_to_and_get_from_child_state() {
    let root_state = State::new("user".to_string());
    let mut state = State::new_chained(&root_state);
    let key = "rust-is-terrific".to_string();
    let val = e_number!(10.5);
    state.insert(key.clone(), val.clone());
    assert_eq!(&val, state.get(None, &key).unwrap());
}

#[test]
fn test_insert_to_root_state_and_get_from_child_state() {
    let mut root_state = State::new("user".to_string());
    let key = "rust-is-terrific".to_string();
    let val = e_number!(10.5);
    root_state.insert(key.clone(), val.clone());
    let state = State::new_chained(&root_state);
    assert_eq!(&val, state.get(None, &key).unwrap());
}

#[test]
fn test_insert_to_child_state_and_get_none_from_root_state() {
    let root_state = State::new("user".to_string());
    let key = "rust-is-terrific".to_string();
    let val = e_number!(10.5);
    let mut state = State::new_chained(&root_state);
    state.insert(key.clone(), val.clone());
    assert!(root_state.get(None, &key).is_none());
}

#[test]
fn test_shadow_val_in_root_state() {
    let mut root_state = State::new("user".to_string());
    let key = "rust-is-terrific".to_string();
    let val_in_root = e_number!(10.5);
    root_state.insert(key.clone(), val_in_root);
    let val_in_child = e_number!(0_f64);
    let mut state = State::new_chained(&root_state);
    state.insert(key.clone(), val_in_child.clone());
    assert_eq!(&val_in_child, state.get(None, &key).unwrap());
}

#[test]
fn test_insert_to_and_get_from_root_state_with_namespace() {
    let mut state = State::new("user".to_string());
    let key = "rust-is-terrific".to_string();
    let val = e_number!(10.5);
    state.insert(key.clone(), val.clone());
    assert_eq!(&val, state.get(Some("user".to_string()).as_ref(), &key).unwrap());
}

#[test]
fn test_insert_to_and_get_from_child_state_with_namespace() {
    let root_state = State::new("user".to_string());
    let mut state = State::new_chained(&root_state);
    let key = "rust-is-terrific".to_string();
    let val = e_number!(10.5);
    state.insert(key.clone(), val.clone());
    assert_eq!(&val, state.get(Some("user_chained".to_string()).as_ref(), &key).unwrap());
}

#[test]
fn test_insert_to_root_state_and_get_from_child_state_with_namespace() {
    let mut root_state = State::new("user".to_string());
    let key = "rust-is-terrific".to_string();
    let val = e_number!(10.5);
    root_state.insert(key.clone(), val.clone());
    let state = State::new_chained(&root_state);
    assert_eq!(&val, state.get(Some("user".to_string()).as_ref(), &key).unwrap());
}

#[test]
fn test_insert_to_child_state_and_get_none_from_root_state_with_namespace() {
    let root_state = State::new("user".to_string());
    let key = "rust-is-terrific".to_string();
    let val = e_number!(10.5);
    let mut state = State::new_chained(&root_state);
    state.insert(key.clone(), val.clone());
    assert!(root_state.get(Some("user".to_string()).as_ref(), &key).is_none());
}

#[test]
fn test_shadow_val_in_root_state_with_namespace() {
    let mut root_state = State::new("user".to_string());
    let key = "rust-is-terrific".to_string();
    let val_in_root = e_number!(10.5);
    root_state.insert(key.clone(), val_in_root);
    let val_in_child = e_number!(0_f64);
    let mut state = State::new_chained(&root_state);
    state.insert(key.clone(), val_in_child.clone());
    assert_eq!(&val_in_child, state.get(None, &key).unwrap());
}

#[test]
fn test_expand_number() {
    let ref mut state = State::new("user".to_string());
    let num = 1_f64;
    assert_eq!(e_number!(num), state.expand(&e_number!(num)).ok().unwrap());
}

#[test]
fn test_expand_string() {
    let ref mut state = State::new("user".to_string());
    let s = "rust is wonderful";
    assert_eq!(e_string!(s), state.expand(&e_string!(s)).ok().unwrap());
}

#[test]
fn test_expand_symbol() {
    let ref mut state = State::new("user".to_string());
    let s = "+";
    assert_eq!(e_symbol!(s), state.expand(&e_symbol!(s)).ok().unwrap());
}

#[test]
fn test_expand_keyword() {
    let ref mut state = State::new("user".to_string());
    let s = ":+";
    assert_eq!(e_keyword!(s), state.expand(&e_keyword!(s)).ok().unwrap());
}

#[test]
fn test_expand_empty_list() {
    let ref mut state = State::new("user".to_string());
    assert_eq!(e_list![], state.expand(&e_list![]).ok().unwrap());
}

#[test]
fn test_expand_fn() {
    let ref mut state = State::new("user".to_string());
    let e = e_fn!([e_symbol!("a")],
                  [e_call!["+", e_symbol!("a"), e_number!(1_f64)]]);
    let n = e_list![e_symbol!("fn"),
                    e_vec![e_symbol!("a")],
                    e_list![e_symbol!("+"), e_symbol!("a"), e_number!(1_f64)]];
    assert_eq!(e, state.expand(&n).ok().unwrap());
}

#[test]
fn test_expand_macro() {
    let ref mut state = State::new("user".to_string());
    let e = e_macro!([e_symbol!("a")],
                     [e_call!["+", e_symbol!("a"), e_number!(1_f64)]]);
    let n = e_list![e_symbol!("macro"),
                    e_vec![e_symbol!("a")],
                    e_list![e_symbol!("+"), e_symbol!("a"), e_number!(1_f64)]];
    assert_eq!(e, state.expand(&n).ok().unwrap());
}

#[test]
fn test_expand_def() {
    let ref mut state = State::new("user".to_string());
    let e = e_def!["a", e_number![1_f64]];
    let n = e_list![e_symbol!["def"], e_symbol!["a"], e_number![1_f64]];
    assert_eq!(e, state.expand(&n).ok().unwrap());
}

#[test]
fn test_expand_call_fn() {
    let ref mut state = State::new("user".to_string());
    let e = e_call!["+", e_symbol!["a"], e_number![1_f64]];
    let n = e_list![e_symbol!["+"], e_symbol!["a"], e_number![1_f64]];
    assert_eq!(e, state.expand(&n).ok().unwrap());
}

#[test]
fn test_expand_call_macro() {
    let ref mut state = State::new("user".to_string());
    let m = e_def!["m", e_macro![[e_symbol!["a"], e_symbol!["b"]],
                                 [e_call!["+", e_symbol!["a"], e_symbol!["b"]]]]];
    state.eval(&m).ok().unwrap();
    let e = e_number![3.];
    let n = e_list![e_symbol!["m"], e_number![1.], e_number![2.]];
    assert_eq!(e, state.expand(&n).ok().unwrap());
}

#[test]
fn test_expand_quote() {
    let ref mut state = State::new("user".to_string());
    let n = e_list![e_symbol!["quote"], e_symbol!["a"]];
    assert_eq!(e_call!["quote", e_symbol!["a"]], state.expand(&n).ok().unwrap());
    let n = e_list![e_symbol!["quote"],
                    e_list![e_symbol!["+"], e_symbol!["a"], e_symbol!["b"]]];
    assert_eq!(e_call!["quote", e_list![e_symbol!["+"], e_symbol!["a"], e_symbol!["b"]]],
               state.expand(&n).ok().unwrap());
}

#[test]
fn test_expand_unquote() {
    let ref mut state = State::new("user".to_string());
    let n = e_list![e_symbol!["quote"], e_list![e_symbol!["a"],
                                                e_list![e_symbol!["unquote"], e_symbol!["b"]]]];
    let expected_result = e_call!["quote", e_list![e_symbol!["a"],
                                           e_call!["unquote", e_symbol!["b"]]]];
    assert_eq!(expected_result, state.expand(&n).ok().unwrap());
}

#[test]
fn test_expand_unquote_splicing() {
    let ref mut state = State::new("user".to_string());
    let n = e_list![e_symbol!["quote"], e_list![e_symbol!["a"],
                                                e_list![e_symbol!["unquote-splicing"],
                                                        e_symbol!["b"]]]];
    let expected_result = e_call!["quote", e_list![e_symbol!["a"],
                                           e_call!["unquote-splicing", e_symbol!["b"]]]];
    assert_eq!(expected_result, state.expand(&n).ok().unwrap());
}

#[test]
fn test_expand_let() {
    let ref mut state = State::new("user".to_string());
    let n = e_list![e_symbol!["let"], e_vec![e_symbol!["a"], e_list![e_symbol!["+"],
                                                                     e_number![1.],
                                                                     e_number![2.]],
                                             e_symbol!["b"], e_list![e_symbol!["+"],
                                                                     e_symbol!["a"],
                                                                     e_number![3.]]],
                                      e_list![e_symbol!["+"], e_symbol!["a"], e_symbol!["b"]]];
    let expected_result = e_let![[e_symbol!["a"], e_call!["+", e_number![1.], e_number![2.]],
                                  e_symbol!["b"], e_call!["+", e_symbol!["a"], e_number![3.]]],
                                 e_call!["+", e_symbol!["a"], e_symbol!["b"]]];
    assert_eq![expected_result, state.expand(&n).ok().unwrap()];
}

#[test]
fn test_eval_number_to_itself() {
    let num = 10_f64;
    let ref mut state = State::new("user".to_string());
    let expected_result = e_number!(num);
    let actual_result = state.eval(&e_number!(num));
    assert_eq!(expected_result, actual_result.ok().unwrap());
}

#[test]
fn test_eval_keyword_to_itself() {
    let keyword = ":keyword";
    let ref mut state = State::new("user".to_string());
    let expected_result = e_keyword!(keyword);
    let actual_result = state.eval(&e_keyword!(keyword));
    assert_eq!(expected_result, actual_result.ok().unwrap());
}

#[test]
fn test_eval_string_to_itself() {
    let s = "rust is awesome";
    let ref mut state = State::new("user".to_string());
    let expected_result = e_string!(s);
    let actual_result = state.eval(&e_string!(s));
    assert_eq!(expected_result, actual_result.ok().unwrap());
}

#[test]
fn test_eval_undefined_symbol_to_error() {
    let ref mut state = State::new("user".to_string());
    let expected_result = ResolveError("a".to_string());
    let actual_result = state.eval(&e_symbol!("a"));
    assert_eq!(expected_result, actual_result.err().unwrap());
}

#[test]
fn test_eval_true_to_matching_bool() {
    let ref mut state = State::new("user".to_string());
    let expected_result = e_bool!(true);
    let actual_result = state.eval(&e_symbol!("true"));
    assert_eq!(expected_result, actual_result.ok().unwrap());
}

#[test]
fn test_eval_false_to_matching_bool() {
    let ref mut state = State::new("user".to_string());
    let expected_result = e_bool!(false);
    let actual_result = state.eval(&e_symbol!("false"));
    assert_eq!(expected_result, actual_result.ok().unwrap());
}

#[test]
fn test_eval_nil_to_empty_list() {
    let ref mut state = State::new("user".to_string());
    let expected_result = e_list![];
    let actual_result = state.eval(&e_symbol!("nil"));
    assert_eq!(expected_result, actual_result.ok().unwrap());
}

#[test]
fn test_eval_def_special_form() {
    let num = 1_f64;
    let ref mut state = State::new("user".to_string());
    let expected_result = e_number!(num);
    let actual_input = &e_def!["a", e_number!(num)];
    let actual_result = state.eval(&actual_input);
    assert_eq!(expected_result, actual_result.ok().unwrap());
}

#[test]
fn test_eval_let_special_form() {
    let ref mut state = State::new("user".to_string());
    let let_expr = e_let![[e_symbol!["a"], e_call!["+", e_number![1.], e_number![2.]],
                           e_symbol!["b"], e_call!["+", e_symbol!["a"], e_number![3.]]],
                          e_call!["+", e_symbol!["a"], e_symbol!["b"]]];
    assert_eq!(e_number![9.], state.eval(&let_expr).ok().unwrap());
}

#[test]
fn test_eval_fn_special_form_and_call_defined_function() {
    let ref mut state = State::new("user".to_string());
    let expr = &e_def!["add-skip-and-sub",
                       e_fn![[e_symbol!("a"), e_symbol!("b")],
                             [e_call!["+", e_symbol!("a"), e_symbol!("b")],
                              e_call!["-", e_symbol!("a"), e_symbol!("b")]]]];
    state.eval(&expr).ok().unwrap();
    let expected_result = e_number!(-1_f64);
    let actual_input = e_call!["add-skip-and-sub", e_number!(1_f64), e_number!(2_f64)];
    let actual_result = state.eval(&actual_input);
    assert_eq!(expected_result, actual_result.ok().unwrap());
}

#[test]
fn test_eval_macro_special_form_and_call_defined_macro() {
    let ref mut state = State::new("user".to_string());
    let expr = &e_def!["add",
                       e_macro![[e_symbol!("a"), e_symbol!("b")],
                                [e_call!["+", e_symbol!("a"), e_symbol!("b")]]]];
    state.eval(&expr).ok().unwrap();
    let expected_result = e_number!(3.);
    let actual_input = e_call!["add", e_number!(1_f64), e_number!(2_f64)];
    let actual_result = state.eval(&actual_input);
    assert_eq!(expected_result, actual_result.ok().unwrap());
}

#[test]
fn test_eval_fn_and_get_error_when_call_defined_function_with_incorrect_number_of_args() {
    let ref mut state = State::new("user".to_string());
    let actual_input = e_def!["add",
                              e_fn![[e_symbol!("a"), e_symbol!("b")],
                                    [e_call!["+", e_symbol!("a"), e_symbol!("b")]]]];
    state.eval(&actual_input).ok().unwrap();
    let expected_result = IncorrectNumberOfArgumentsError(e_call!["add", e_number![1_f64]]);
    let expr = &e_call!["add", e_number!(1_f64)];
    let mut actual_result = state.eval(&expr);
    assert_eq!(expected_result, actual_result.err().unwrap());
    let expr = &e_call!["add",
                        e_number!(1_f64),
                        e_number!(1_f64),
                        e_number!(1_f64),
                        e_number!(1_f64)];
    let expected_result = IncorrectNumberOfArgumentsError(e_call!["add",
                                                                  e_number![1_f64],
                                                                  e_number![1_f64],
                                                                  e_number![1_f64],
                                                                  e_number![1_f64]]);
    actual_result = state.eval(&expr);
    assert_eq!(expected_result, actual_result.err().unwrap());
}

#[test]
fn test_eval_plus_builtin_fn() {
    let ref mut state = State::new("user".to_string());
    let expr = &e_def!["a", e_number!(1_f64)];
    state.eval(expr).ok().unwrap();
    let expr = &e_def!["b", e_number!(2_f64)];
    state.eval(expr).ok().unwrap();
    let nested_call = e_call!["+", e_symbol!("a"), e_symbol!("b")];
    let actual_input = &e_call!["+", nested_call, e_number!(3_f64)];
    assert_eq!(e_number!(6_f64), state.eval(&actual_input).ok().unwrap());
}

#[test]
fn test_eval_minus_builtin_fn() {
    let ref mut state = State::new("user".to_string());
    let actual_input = &e_call!["-", e_number!(3_f64), e_number!(2_f64)];
    let actual_result = state.eval(&actual_input);
    let expected_result = e_number!(1_f64);
    assert_eq!(expected_result, actual_result.ok().unwrap());
}

#[test]
fn test_eval_div_builtin_fn() {
    let ref mut state = State::new("user".to_string());
    let actual_input = &e_call!["/", e_number!(3_f64), e_number!(2_f64)];
    let actual_result = state.eval(&actual_input);
    let expected_result = e_number!(1.5);
    assert_eq!(expected_result, actual_result.ok().unwrap());
}

#[test]
fn test_eval_mul_builtin_fn() {
    let ref mut state = State::new("user".to_string());
    let actual_input = e_call!["*", e_number!(3.5), e_number!(2_f64)];
    let actual_result = state.eval(&actual_input);
    let expected_result = e_number!(7_f64);
    assert_eq!(expected_result, actual_result.ok().unwrap());
}

#[test]
fn test_eval_lt_builtin_fn_positive_case() {
    let ref mut state = State::new("user".to_string());
    let actual_input = &e_call!["<", e_number!(1_f64), e_number!(2_f64), e_number!(3_f64)];
    let actual_result = state.eval(&actual_input);
    let expected_result = e_bool!(true);
    assert_eq!(expected_result, actual_result.ok().unwrap());
}

#[test]
fn test_eval_lt_builtin_fn_negative_case() {
    let ref mut state = State::new("user".to_string());
    let actual_input = e_call!["<", e_number!(3.5), e_number!(20_f64), e_number!(1_f64)];
    let actual_result = state.eval(&actual_input);
    let expected_result = e_bool!(false);
    assert_eq!(expected_result, actual_result.ok().unwrap());
}

#[test]
fn test_eval_gt_builtin_fn_positive_case() {
    let ref mut state = State::new("user".to_string());
    let expr = &e_def!["a", e_number!(3_f64)];
    state.eval(&expr).ok().unwrap();
    let actual_input = &e_call![">", e_symbol!("a"), e_number!(2_f64), e_number!(1_f64)];
    let actual_result = state.eval(&actual_input);
    let expected_result = e_bool!(true);
    assert_eq!(expected_result, actual_result.ok().unwrap());
}

#[test]
fn test_eval_gt_builtin_fn_negative_case() {
    let ref mut state = State::new("user".to_string());
    let expr = &e_def!["a", e_number!(20_f64)];
    state.eval(&expr).ok().unwrap();
    let actual_input = &e_call![">", e_number!(3.5), e_symbol!("a"), e_number!(1_f64)];
    let actual_result = state.eval(&actual_input);
    let expected_result = e_bool!(false);
    assert_eq!(expected_result, actual_result.ok().unwrap());
}

#[test]
fn test_eval_quote_builtin_fn() {
    let ref mut state = State::new("user".to_string());
    let expr = &e_call!["quote", e_list![e_symbol!["+"], e_symbol!["true"], e_number![1.]]];
    let expected_result = e_list![e_symbol!["+"], e_symbol!["true"], e_number![1.]];
    assert_eq!(expected_result, state.eval(&expr).ok().unwrap());
}

#[test]
fn test_eval_unquote_builtin_fn() {
    let ref mut state = State::new("user".to_string());
    state.insert("a".to_string(), e_number![3.]);
    let expr = e_call!["quote", e_list![e_symbol!["+"],
                                        e_call!["unquote", e_symbol!["a"]], e_number![1.]]];
    let expected_result = e_list![e_symbol!["+"], e_number![3.], e_number![1.]];
    assert_eq!(expected_result, state.eval(&expr).ok().unwrap());
}

#[test]
fn test_eval_unquote_splicing_builtin_fn() {
    let ref mut state = State::new("user".to_string());
    state.insert("a".to_string(), e_list![e_number![1.], e_number![2.], e_number![3.]]);
    let expr = e_call!["quote", e_list![e_symbol!["+"],
                                        e_call!["unquote-splicing", e_symbol!["a"]],
                                        e_number![1.]]];
    let expected_result = e_list![e_symbol!["+"],
                                  e_number![1.],
                                  e_number![2.],
                                  e_number![3.],
                                  e_number![1.]];
    assert_eq!(expected_result, state.eval(&expr).ok().unwrap());
}

#[test]
fn test_eval_eval_builtin_fn() {
    let ref mut state = State::new("user".to_string());
    let expr = e_call!["eval", e_list![e_symbol!["+"], e_number![1.], e_number![2.]]];
    let expected_result = e_number![3.];
    assert_eq!(expected_result, state.eval(&expr).ok().unwrap());
    let expr = e_call!["eval", e_call!["quote", e_list![e_symbol!["+"],
                                                        e_number![1.],
                                                        e_number![2.]]]];
    let expected_result = e_number![3.];
    assert_eq!(expected_result, state.eval(&expr).ok().unwrap());
}

#[test]
fn test_eval_eq_builtin_fn_positive_case() {
    let ref mut state = State::new("user".to_string());
    let expr = &e_def!["a", e_number!(3_f64)];
    state.eval(&expr).ok().unwrap();
    let actual_input = &e_call!["=", e_symbol!("a"), e_number!(3_f64), e_number!(3_f64)];
    let actual_result = state.eval(&actual_input);
    let expected_result = e_bool!(true);
    assert_eq!(expected_result, actual_result.ok().unwrap());
}

#[test]
fn test_eval_eq_builtin_fn_negative_case() {
    let ref mut state = State::new("user".to_string());
    let expr = &e_def!["a", e_number![1_f64]];
    state.eval(&expr).ok().unwrap();
    let actual_input = &e_call!["=", e_number![3.5], e_number![20_f64], e_symbol!["a"]];
    let actual_result = state.eval(&actual_input);
    let expected_result = e_bool![false];
    assert_eq!(expected_result, actual_result.ok().unwrap());
}

#[test]
fn test_eval_if_builtin_fn() {
    let ref mut state = State::new("user".to_string());
    let ref actual_input = e_call!["if", e_call!["=", e_number![3.], e_number![3.]],
                                         e_call!["+", e_number![3.], e_number![3.]],
                                         e_call!["-", e_number![3.], e_number![3.]]];
    let actual_result = state.eval(&actual_input);
    let expected_result = e_number![6.];
    assert_eq!(expected_result, actual_result.ok().unwrap());
    let ref actual_input = e_call!["if", e_call!["<", e_number![3.], e_number![3.]],
                                         e_call!["+", e_number![3.], e_number![3.]],
                                         e_call!["-", e_number![3.], e_number![3.]]];
    let actual_result = state.eval(&actual_input);
    let expected_result = e_number![0.];
    assert_eq!(expected_result, actual_result.ok().unwrap());
}
