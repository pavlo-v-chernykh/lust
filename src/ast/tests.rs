use scope::Scope;
use ast::error::EvalError::*;

#[test]
fn test_expand_number() {
    let ref mut scope = Scope::new_std();
    let num = 1_f64;
    assert_eq!(e_number!(num), e_number!(num).expand(scope).ok().unwrap());
}

#[test]
fn test_expand_string() {
    let ref mut scope = Scope::new_std();
    let s = "rust is wonderful";
    assert_eq!(e_string!(s), e_string!(s).expand(scope).ok().unwrap());
}

#[test]
fn test_expand_symbol() {
    let ref mut scope = Scope::new_std();
    let s = "+";
    assert_eq!(e_symbol!(s), e_symbol!(s).expand(scope).ok().unwrap());
}

#[test]
fn test_expand_keyword() {
    let ref mut scope = Scope::new_std();
    let s = ":+";
    assert_eq!(e_keyword!(s), e_keyword!(s).expand(scope).ok().unwrap());
}

#[test]
fn test_expand_empty_list() {
    let ref mut scope = Scope::new_std();
    assert_eq!(e_list![], e_list![].expand(scope).ok().unwrap());
}

#[test]
fn test_expand_fn() {
    let ref mut scope = Scope::new_std();
    let e = e_fn!([e_symbol!("a")],
                  [e_call!["+", e_symbol!("a"), e_number!(1_f64)]]);
    let n = e_list![e_symbol!("fn"),
                    e_vec![e_symbol!("a")],
                    e_list![e_symbol!("+"), e_symbol!("a"), e_number!(1_f64)]];
    assert_eq!(e, n.expand(scope).ok().unwrap());
}

#[test]
fn test_expand_macro() {
    let ref mut scope = Scope::new_std();
    let e = e_macro!([e_symbol!("a")],
                     [e_call!["+", e_symbol!("a"), e_number!(1_f64)]]);
    let n = e_list![e_symbol!("macro"),
                    e_vec![e_symbol!("a")],
                    e_list![e_symbol!("+"), e_symbol!("a"), e_number!(1_f64)]];
    assert_eq!(e, n.expand(scope).ok().unwrap());
}

#[test]
fn test_expand_def() {
    let ref mut scope = Scope::new_std();
    let e = e_def!["a", e_number![1_f64]];
    let n = e_list![e_symbol!["def"], e_symbol!["a"], e_number![1_f64]];
    assert_eq!(e, n.expand(scope).ok().unwrap());
}

#[test]
fn test_expand_call_fn() {
    let ref mut scope = Scope::new_std();
    let e = e_call!["+", e_symbol!["a"], e_number![1_f64]];
    let n = e_list![e_symbol!["+"], e_symbol!["a"], e_number![1_f64]];
    assert_eq!(e, n.expand(scope).ok().unwrap());
}

#[test]
fn test_expand_call_macro() {
    let ref mut scope = Scope::new_std();
    let m = e_def!["m", e_macro![[e_symbol!["a"], e_symbol!["b"]],
                                 [e_call!["+", e_symbol!["a"], e_symbol!["b"]]]]];
    m.eval(scope).ok().unwrap();
    let e = e_number![3.];
    let n = e_list![e_symbol!["m"], e_number![1.], e_number![2.]];
    assert_eq!(e, n.expand(scope).ok().unwrap());
}

#[test]
fn test_expand_quote() {
    let ref mut scope = Scope::new_std();
    let n = e_list![e_symbol!["quote"], e_symbol!["a"]];
    assert_eq!(e_call!["quote", e_symbol!["a"]], n.expand(scope).ok().unwrap());
    let n = e_list![e_symbol!["quote"],
                    e_list![e_symbol!["+"], e_symbol!["a"], e_symbol!["b"]]];
    assert_eq!(e_call!["quote", e_list![e_symbol!["+"], e_symbol!["a"], e_symbol!["b"]]],
               n.expand(scope).ok().unwrap());
}

#[test]
fn test_expand_unquote() {
    let ref mut scope = Scope::new_std();
    let n = e_list![e_symbol!["quote"], e_list![e_symbol!["a"],
                                                e_list![e_symbol!["unquote"], e_symbol!["b"]]]];
    let expected_result = e_call!["quote", e_list![e_symbol!["a"],
                                           e_call!["unquote", e_symbol!["b"]]]];
    assert_eq!(expected_result, n.expand(scope).ok().unwrap());
}

#[test]
fn test_eval_number_to_itself() {
    let num = 10_f64;
    let ref mut scope = Scope::new();
    let expected_result = e_number!(num);
    let actual_result = e_number!(num).eval(scope);
    assert_eq!(expected_result, actual_result.ok().unwrap());
}

#[test]
fn test_eval_keyword_to_itself() {
    let keyword = ":keyword";
    let ref mut scope = Scope::new();
    let expected_result = e_keyword!(keyword);
    let actual_result = e_keyword!(keyword).eval(scope);
    assert_eq!(expected_result, actual_result.ok().unwrap());
}

#[test]
fn test_eval_string_to_itself() {
    let s = "rust is awesome";
    let ref mut scope = Scope::new();
    let expected_result = e_string!(s);
    let actual_result = e_string!(s).eval(scope);
    assert_eq!(expected_result, actual_result.ok().unwrap());
}

#[test]
fn test_eval_undefined_symbol_to_error() {
    let ref mut scope = Scope::new();
    let expected_result = ResolveError("a".to_string());
    let actual_result = e_symbol!("a").eval(scope);
    assert_eq!(expected_result, actual_result.err().unwrap());
}

#[test]
fn test_eval_true_to_matching_bool() {
    let ref mut scope = Scope::new_std();
    let expected_result = e_bool!(true);
    let actual_result = e_symbol!("true").eval(scope);
    assert_eq!(expected_result, actual_result.ok().unwrap());
}

#[test]
fn test_eval_false_to_matching_bool() {
    let ref mut scope = Scope::new_std();
    let expected_result = e_bool!(false);
    let actual_result = e_symbol!("false").eval(scope);
    assert_eq!(expected_result, actual_result.ok().unwrap());
}

#[test]
fn test_eval_nil_to_empty_list() {
    let ref mut scope = Scope::new_std();
    let expected_result = e_list![];
    let actual_result = e_symbol!("nil").eval(scope);
    assert_eq!(expected_result, actual_result.ok().unwrap());
}

#[test]
fn test_eval_def_special_form() {
    let num = 1_f64;
    let ref mut scope = Scope::new();
    let expected_result = e_number!(num);
    let actual_input = &e_def!["a", e_number!(num)];
    let actual_result = actual_input.eval(scope);
    assert_eq!(expected_result, actual_result.ok().unwrap());
}

#[test]
fn test_eval_fn_special_form_and_call_defined_function() {
    let ref mut scope = Scope::new();
    let expr = &e_def!["add-skip-and-sub",
                       e_fn![[e_symbol!("a"), e_symbol!("b")],
                             [e_call!["+", e_symbol!("a"), e_symbol!("b")],
                              e_call!["-", e_symbol!("a"), e_symbol!("b")]]]];
    expr.eval(scope).ok().unwrap();
    let expected_result = e_number!(-1_f64);
    let actual_input = e_call!["add-skip-and-sub", e_number!(1_f64), e_number!(2_f64)];
    let actual_result = actual_input.eval(scope);
    assert_eq!(expected_result, actual_result.ok().unwrap());
}

#[test]
fn test_eval_macro_special_form_and_call_defined_macro() {
    let ref mut scope = Scope::new();
    let expr = &e_def!["add",
                       e_macro![[e_symbol!("a"), e_symbol!("b")],
                                [e_call!["+", e_symbol!("a"), e_symbol!("b")]]]];
    expr.eval(scope).ok().unwrap();
    let expected_result = e_number!(3.);
    let actual_input = e_call!["add", e_number!(1_f64), e_number!(2_f64)];
    let actual_result = actual_input.eval(scope);
    assert_eq!(expected_result, actual_result.ok().unwrap());
}

#[test]
fn test_eval_fn_and_get_error_when_call_defined_function_with_incorrect_number_of_args() {
    let ref mut scope = Scope::new();
    let actual_input = e_def!["add",
                              e_fn![[e_symbol!("a"), e_symbol!("b")],
                                    [e_call!["+", e_symbol!("a"), e_symbol!("b")]]]];
    actual_input.eval(scope).ok().unwrap();
    let expected_result = IncorrectNumberOfArgumentsError(e_call!["add", e_number![1_f64]]);
    let expr = &e_call!["add", e_number!(1_f64)];
    let mut actual_result = expr.eval(scope);
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
    actual_result = expr.eval(scope);
    assert_eq!(expected_result, actual_result.err().unwrap());
}

#[test]
fn test_eval_plus_builtin_fn() {
    let ref mut scope = Scope::new();
    let expr = &e_def!["a", e_number!(1_f64)];
    expr.eval(scope).ok().unwrap();
    let expr = &e_def!["b", e_number!(2_f64)];
    expr.eval(scope).ok().unwrap();
    let nested_call = e_call!["+", e_symbol!("a"), e_symbol!("b")];
    let actual_input = &e_call!["+", nested_call, e_number!(3_f64)];
    assert_eq!(e_number!(6_f64), actual_input.eval(scope).ok().unwrap());
}

#[test]
fn test_eval_minus_builtin_fn() {
    let ref mut scope = Scope::new();
    let actual_input = &e_call!["-", e_number!(3_f64), e_number!(2_f64)];
    let actual_result = actual_input.eval(scope);
    let expected_result = e_number!(1_f64);
    assert_eq!(expected_result, actual_result.ok().unwrap());
}

#[test]
fn test_eval_div_builtin_fn() {
    let ref mut scope = Scope::new();
    let actual_input = &e_call!["/", e_number!(3_f64), e_number!(2_f64)];
    let actual_result = actual_input.eval(scope);
    let expected_result = e_number!(1.5);
    assert_eq!(expected_result, actual_result.ok().unwrap());
}

#[test]
fn test_eval_mul_builtin_fn() {
    let ref mut scope = Scope::new();
    let actual_input = e_call!["*", e_number!(3.5), e_number!(2_f64)];
    let actual_result = actual_input.eval(scope);
    let expected_result = e_number!(7_f64);
    assert_eq!(expected_result, actual_result.ok().unwrap());
}

#[test]
fn test_eval_lt_builtin_fn_positive_case() {
    let ref mut scope = Scope::new();
    let actual_input = &e_call!["<", e_number!(1_f64), e_number!(2_f64), e_number!(3_f64)];
    let actual_result = actual_input.eval(scope);
    let expected_result = e_bool!(true);
    assert_eq!(expected_result, actual_result.ok().unwrap());
}

#[test]
fn test_eval_lt_builtin_fn_negative_case() {
    let ref mut scope = Scope::new();
    let actual_input = e_call!["<", e_number!(3.5), e_number!(20_f64), e_number!(1_f64)];
    let actual_result = actual_input.eval(scope);
    let expected_result = e_bool!(false);
    assert_eq!(expected_result, actual_result.ok().unwrap());
}

#[test]
fn test_eval_gt_builtin_fn_positive_case() {
    let ref mut scope = Scope::new();
    let expr = &e_def!["a", e_number!(3_f64)];
    expr.eval(scope).ok().unwrap();
    let actual_input = &e_call![">", e_symbol!("a"), e_number!(2_f64), e_number!(1_f64)];
    let actual_result = actual_input.eval(scope);
    let expected_result = e_bool!(true);
    assert_eq!(expected_result, actual_result.ok().unwrap());
}

#[test]
fn test_eval_gt_builtin_fn_negative_case() {
    let ref mut scope = Scope::new();
    let expr = &e_def!["a", e_number!(20_f64)];
    expr.eval(scope).ok().unwrap();
    let actual_input = &e_call![">", e_number!(3.5), e_symbol!("a"), e_number!(1_f64)];
    let actual_result = actual_input.eval(scope);
    let expected_result = e_bool!(false);
    assert_eq!(expected_result, actual_result.ok().unwrap());
}

#[test]
fn test_eval_quote_builtin_fn() {
    let ref mut scope = Scope::new();
    let expr = &e_call!["quote", e_list![e_symbol!["+"], e_symbol!["true"], e_number![1.]]];
    let expected_result = e_list![e_symbol!["+"], e_symbol!["true"], e_number![1.]];
    assert_eq!(expected_result, expr.eval(scope).ok().unwrap());
}

#[test]
fn test_eval_unquote_builtin_fn() {
    let ref mut scope = Scope::new();
    scope.insert("a".to_string(), e_number![3.]);
    let expr = e_call!["quote", e_list![e_symbol!["+"],
                                        e_call!["unquote", e_symbol!["a"]], e_number![1.]]];
    let expected_result = e_list![e_symbol!["+"], e_number![3.], e_number![1.]];
    assert_eq!(expected_result, expr.eval(scope).ok().unwrap());
}

#[test]
fn test_eval_eval_builtin_fn() {
    let ref mut scope = Scope::new();
    let expr = e_call!["eval", e_list![e_symbol!["+"], e_number![1.], e_number![2.]]];
    let expected_result = e_number![3.];
    assert_eq!(expected_result, expr.eval(scope).ok().unwrap());
    let expr = e_call!["eval", e_call!["quote", e_list![e_symbol!["+"],
                                                        e_number![1.],
                                                        e_number![2.]]]];
    let expected_result = e_number![3.];
    assert_eq!(expected_result, expr.eval(scope).ok().unwrap());
}

#[test]
fn test_eval_eq_builtin_fn_positive_case() {
    let ref mut scope = Scope::new();
    let expr = &e_def!["a", e_number!(3_f64)];
    expr.eval(scope).ok().unwrap();
    let actual_input = &e_call!["=", e_symbol!("a"), e_number!(3_f64), e_number!(3_f64)];
    let actual_result = actual_input.eval(scope);
    let expected_result = e_bool!(true);
    assert_eq!(expected_result, actual_result.ok().unwrap());
}

#[test]
fn test_eval_eq_builtin_fn_negative_case() {
    let ref mut scope = Scope::new();
    let expr = &e_def!["a", e_number![1_f64]];
    expr.eval(scope).ok().unwrap();
    let actual_input = &e_call!["=", e_number![3.5], e_number![20_f64], e_symbol!["a"]];
    let actual_result = actual_input.eval(scope);
    let expected_result = e_bool![false];
    assert_eq!(expected_result, actual_result.ok().unwrap());
}

#[test]
fn test_eval_if_builtin_fn() {
    let ref mut scope = Scope::new();
    let ref actual_input = e_call!["if", e_call!["=", e_number![3.], e_number![3.]],
                                         e_call!["+", e_number![3.], e_number![3.]],
                                         e_call!["-", e_number![3.], e_number![3.]]];
    let actual_result = actual_input.eval(scope);
    let expected_result = e_number![6.];
    assert_eq!(expected_result, actual_result.ok().unwrap());
    let ref actual_input = e_call!["if", e_call!["<", e_number![3.], e_number![3.]],
                                         e_call!["+", e_number![3.], e_number![3.]],
                                         e_call!["-", e_number![3.], e_number![3.]]];
    let actual_result = actual_input.eval(scope);
    let expected_result = e_number![0.];
    assert_eq!(expected_result, actual_result.ok().unwrap());
}


#[test]
fn test_format_list_with_nested_list_and_atoms() {
    let actual_input = e_def!["a", e_call!["+", e_number!(1_f64), e_number!(2_f64)]];
    let actual_result = format!("{}", actual_input);
    let expected_result = "(def a (+ 1 2))";
    assert_eq!(expected_result, actual_result);
}

#[test]
fn test_format_call_expr_without_args() {
    assert_eq!(format!("{}", e_call!["+",]), "(+)");
}
