#[macro_use]
mod macros;
mod error;

use std::fmt;
use scope::Scope;
pub use self::error::{EvalError, EvalErrorCode};

type EvalResult = Result<Expr, EvalError>;

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Number(f64),
    Bool(bool),
    String(String),
    Symbol(String),
    List(Vec<Expr>),
    Fn {
        params: Vec<Expr>,
        body: Vec<Expr>,
    },
    Macro {
        params: Vec<Expr>,
        body: Vec<Expr>,
    },
    Def {
        sym: String,
        expr: Box<Expr>,
    },
    Call {
        name: String,
        args: Vec<Expr>,
    },
}

impl Expr {
    pub fn eval(&self, scope: &mut Scope) -> EvalResult {
        match self {
            &Expr::Symbol(ref name) => {
                if let Some(e) = scope.get(name) {
                    Ok(e.clone())
                } else {
                    Expr::error(EvalErrorCode::UnknownError)
                }
            },
            &Expr::Def { .. } => {
                self.eval_def(scope)
            },
            &Expr::Call { .. } => {
                self.eval_call(scope)
            }
            e => {
                Ok(e.clone())
            },
        }
    }

    pub fn eval_quoted(&self, scope: &mut Scope) -> EvalResult {
        match self {
            &Expr::Call { .. } => {
                self.eval_call(scope)
            }
            e => {
                Ok(e.clone())
            },
        }
    }

    fn eval_def(&self, scope: &mut Scope) -> EvalResult {
        if let &Expr::Def { ref sym, ref expr } = self {
            let e = try!(expr.eval(scope));
            scope.insert(sym.clone(), e.clone());
            Ok(e)
        } else {
            Expr::error(EvalErrorCode::UnknownError)
        }
    }

    fn eval_call(&self, scope: &mut Scope) -> EvalResult {
        if let &Expr::Call { ref name, .. } = self {
            match &name[..] {
                "+" => {
                    self.eval_call_builtin_plus(scope)
                },
                "-" => {
                    self.eval_call_builtin_minus(scope)
                },
                "*" => {
                    self.eval_call_builtin_mul(scope)
                },
                "/" => {
                    self.eval_call_builtin_div(scope)
                },
                "<" => {
                    self.eval_call_builtin_lt(scope)
                },
                ">" => {
                    self.eval_call_builtin_gt(scope)
                },
                "=" => {
                    self.eval_call_builtin_eq(scope)
                },
                "quote" => {
                    self.eval_call_builtin_quote(scope)
                },
                "unquote" => {
                    self.eval_call_builtin_unquote(scope)
                },
                _ => {
                    self.eval_call_custom(scope)
                },
            }
        } else {
            Expr::error(EvalErrorCode::UnknownError)
        }
    }

    fn eval_call_builtin_plus(&self, scope: &mut Scope) -> EvalResult {
        if let &Expr::Call { ref args, .. } = self {
            let mut result = 0_f64;
            for a in args {
                if let Ok(Expr::Number(n)) = a.eval(scope) {
                    result += n;
                } else {
                    return Expr::error(EvalErrorCode::UnknownError)
                }
            }
            Ok(Expr::Number(result))
        } else {
            Expr::error(EvalErrorCode::UnknownError)
        }
    }

    fn eval_call_builtin_minus(&self, scope: &mut Scope) -> EvalResult {
        if let &Expr::Call { ref args, .. } = self {
            if args.len() >= 1 {
                if let Ok(Expr::Number(n)) = args[0].eval(scope) {
                    let mut result = if args.len() == 1 { -n } else { n };
                    for a in &args[1..] {
                        if let Ok(Expr::Number(n)) = a.eval(scope) {
                            result -= n
                        } else {
                            return Expr::error(EvalErrorCode::UnknownError)
                        }
                    }
                    Ok(Expr::Number(result))
                } else {
                    Expr::error(EvalErrorCode::UnknownError)
                }
            } else {
                Expr::error(EvalErrorCode::UnknownError)
            }
        } else {
            Expr::error(EvalErrorCode::UnknownError)
        }
    }

    fn eval_call_builtin_mul(&self, scope: &mut Scope) -> EvalResult {
        if let &Expr::Call { ref args, .. } = self {
            let mut result = 1_f64;
            for a in args {
                if let Ok(Expr::Number(n)) = a.eval(scope) {
                    result *= n
                } else {
                    return Expr::error(EvalErrorCode::UnknownError)
                }
            }
            Ok(Expr::Number(result))
        } else {
            Expr::error(EvalErrorCode::UnknownError)
        }
    }

    fn eval_call_builtin_div(&self, scope: &mut Scope) -> EvalResult {
        if let &Expr::Call { ref args, .. } = self {
            if args.len() >= 1 {
                if let Ok(Expr::Number(n)) = args[0].eval(scope) {
                    let mut result = if args.len() == 1 { 1. / n } else { n };
                    for a in &args[1..] {
                        if let Ok(Expr::Number(n)) = a.eval(scope) {
                            result /= n
                        } else {
                            return Expr::error(EvalErrorCode::UnknownError)
                        }
                    }
                    Ok(Expr::Number(result))
                } else {
                    Expr::error(EvalErrorCode::UnknownError)
                }
            } else {
                Expr::error(EvalErrorCode::UnknownError)
            }
        } else {
            Expr::error(EvalErrorCode::UnknownError)
        }
    }

    fn eval_call_builtin_lt(&self, scope: &mut Scope) -> EvalResult {
        if let &Expr::Call { ref args, .. } = self {
            if args.len() >= 1 {
                if let Ok(Expr::Number(n)) = args[0].eval(scope) {
                    let mut temp = n;
                    for a in &args[1..] {
                        if let Ok(Expr::Number(n)) = a.eval(scope) {
                            if temp < n {
                                temp = n
                            } else {
                                return Ok(Expr::Bool(false))
                            }
                        } else {
                            return Expr::error(EvalErrorCode::UnknownError)
                        }
                    }
                    Ok(Expr::Bool(true))
                } else {
                    return Expr::error(EvalErrorCode::UnknownError)
                }
            } else {
                Expr::error(EvalErrorCode::UnknownError)
            }
        } else {
            Expr::error(EvalErrorCode::UnknownError)
        }
    }

    fn eval_call_builtin_gt(&self, scope: &mut Scope) -> EvalResult {
        if let &Expr::Call { ref args, .. } = self {
            if args.len() >= 1 {
                if let Ok(Expr::Number(n)) = args[0].eval(scope) {
                    let mut temp = n;
                    for a in &args[1..] {
                        if let Ok(Expr::Number(n)) = a.eval(scope) {
                            if temp > n {
                                temp = n
                            } else {
                                return Ok(Expr::Bool(false))
                            }
                        } else {
                            return Expr::error(EvalErrorCode::UnknownError)
                        }
                    }
                    Ok(Expr::Bool(true))
                } else {
                    return Expr::error(EvalErrorCode::UnknownError)
                }
            } else {
                Expr::error(EvalErrorCode::UnknownError)
            }
        } else {
            Expr::error(EvalErrorCode::UnknownError)
        }
    }

    fn eval_call_builtin_eq(&self, scope: &mut Scope) -> EvalResult {
        if let &Expr::Call { ref args, .. } = self {
            if args.len() >= 1 {
                if let Ok(Expr::Number(n)) = args[0].eval(scope) {
                    let mut temp = n;
                    for a in &args[1..] {
                        if let Ok(Expr::Number(n)) = a.eval(scope) {
                            if temp == n {
                                temp = n
                            } else {
                                return Ok(Expr::Bool(false))
                            }
                        } else {
                            return Expr::error(EvalErrorCode::UnknownError)
                        }
                    }
                    Ok(Expr::Bool(true))
                } else {
                    return Expr::error(EvalErrorCode::UnknownError)
                }
            } else {
                Expr::error(EvalErrorCode::UnknownError)
            }
        } else {
            Expr::error(EvalErrorCode::UnknownError)
        }
    }

    fn eval_call_builtin_quote(&self, scope: &mut Scope) -> EvalResult {
        if let &Expr::Call { ref args, .. } = self {
            if args.len() == 1 {
                args[0].eval_quoted(scope)
            } else {
                Expr::error(EvalErrorCode::UnknownError)
            }
        } else {
            Expr::error(EvalErrorCode::UnknownError)
        }
    }

    fn eval_call_builtin_unquote(&self, scope: &mut Scope) -> EvalResult {
        if let &Expr::Call { ref args, .. } = self {
            if args.len() == 1 {
                args[0].eval(scope)
            } else {
                Expr::error(EvalErrorCode::UnknownError)
            }
        } else {
            Expr::error(EvalErrorCode::UnknownError)
        }
    }

    fn eval_call_custom(&self, scope: &mut Scope) -> EvalResult {
        if let &Expr::Call { ref name, ref args } = self {
            let func = match scope.get(name) {
                Some(e) => {
                    e.clone()
                },
                _ => {
                    return Expr::error(EvalErrorCode::UnknownError)
                }
            };
            match func {
                Expr::Fn { ref params, ref body } |
                Expr::Macro { ref params, ref body } => {
                    if args.len() != params.len() {
                        return Expr::error(EvalErrorCode::UnknownError)
                    }

                    let mut e_args = vec![];
                    for a in args {
                        e_args.push(try!(a.eval(scope)))
                    }

                    let ref mut fn_scope = Scope::new_chained(&scope);
                    for (p, a) in params.iter().zip(e_args.iter()) {
                        if let Expr::Symbol(ref s) = *p {
                            fn_scope.insert(s.clone(), a.clone());
                        } else {
                            return Expr::error(EvalErrorCode::UnknownError)
                        }
                    }

                    let mut result = e_list![];
                    for e in body {
                        result = try!(e.eval(fn_scope));
                    }

                    Ok(result)
                },
                _ => {
                    Expr::error(EvalErrorCode::UnknownError)
                }
            }
        } else {
            Expr::error(EvalErrorCode::UnknownError)
        }
    }

    fn error(code: EvalErrorCode) -> EvalResult {
        Err(EvalError::new(code))
    }
}


impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Expr::Number(n) => {
                write!(f, "{}", n)
            },
            &Expr::Bool(b) => {
                write!(f, "{}", b)
            },
            &Expr::Symbol(ref s) => {
                write!(f, "{}", s)
            },
            &Expr::String(ref s) => {
                write!(f, r#""{}""#, s)
            },
            &Expr::List(ref l) => {
                write!(f, "({})", l)
            },
            &Expr::Def { ref sym, ref expr } => {
                write!(f, "(def {} {})", sym, expr)
            },
            &Expr::Fn { ref params, ref body } => {
                write!(f, "(fn ({}) {})", params, body)
            },
            &Expr::Macro { ref params, ref body } => {
                write!(f, "(macro ({}) {})", params, body)
            },
            &Expr::Call { ref name, ref args } => {
                write!(f, "({} {})", name, args)
            },
        }
    }
}

impl fmt::Display for Vec<Expr> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut a = String::new();
        let i_last = self.len() - 1;
        for (i, s) in self.iter().enumerate() {
            if i < i_last {
                a.push_str(&format!("{} ", s))
            } else {
                a.push_str(&format!("{}", s))
            }
        }
        write!(f, "{}", a)
    }
}

#[cfg(test)]
mod tests {
    use scope::Scope;
    use super::error::EvalError;
    use super::error::EvalErrorCode::UnknownError;

    #[test]
    fn test_eval_number_to_itself() {
        let num = 10_f64;
        let ref mut scope = Scope::new();
        let expected_result = e_number!(num);
        let actual_result = e_number!(num).eval(scope);
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
        let expected_result = EvalError::new(UnknownError);
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
        let expected_result = EvalError::new(UnknownError);
        let expr = &e_call!["add", e_number!(1_f64)];
        let mut actual_result = expr.eval(scope);
        assert_eq!(expected_result, actual_result.err().unwrap());
        let expr = &e_call!["add",
                            e_number!(1_f64),
                            e_number!(1_f64),
                            e_number!(1_f64),
                            e_number!(1_f64)];
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
    fn test_format_list_with_nested_list_and_atoms() {
        let actual_input = e_def!["a", e_call!["+", e_number!(1_f64), e_number!(2_f64)]];
        let actual_result = format!("{}", actual_input);
        let expected_result = "(def a (+ 1 2))";
        assert_eq!(expected_result, actual_result);
    }
}
