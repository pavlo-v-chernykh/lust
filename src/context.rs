use std::collections::HashMap;
use ast::Expr;
use val::Val;

macro_rules! try_number {
    ($e:expr) => (match $e {
        Ok($crate::val::Val::Number(n)) => {
            n
        },
        _ => {
            return Err(EvalError::EvalError)
        }
    })
}

#[derive(Debug, PartialEq)]
enum EvalError{
    EvalError,
    IncorrectSpecialForm,
    IncorrectNumberOfArguments,
    IncorrectTypeOfArgument
}

type EvalResult = Result<Val, EvalError>;

pub struct Context {
    env: HashMap<String, Val>,
}

impl Context {
    pub fn new() -> Context {
        let mut ctx = Context {
            env: HashMap::new(),
        };
        ctx.env.insert("nil".to_string(), v_list![]);
        ctx.env.insert("true".to_string(), v_bool!(true));
        ctx.env.insert("false".to_string(), v_bool!(false));
        ctx
    }

    pub fn eval(&mut self, s: &Expr) -> EvalResult {
        match *s {
            Expr::Number(n) => {
                Ok(Val::Number(n))
            },
            Expr::Symbol(ref name) => {
                if let Some(v) = self.env.get(name) {
                    Ok(v.clone())
                } else {
                    Err(EvalError::EvalError)
                }
            },
            Expr::String(ref s) => {
                Ok(Val::String(s.clone()))
            },
            Expr::List(ref l) => {
                if let Expr::Symbol(ref n) = l[0] {
                    match &n[..] {
                        "def" => {
                            self.eval_def(s)
                        },
                        "fn" => {
                            self.eval_fn(s)
                        },
                        "+" => {
                            self.eval_plus(s)
                        },
                        "-" => {
                            self.eval_minus(s)
                        },
                        "/" => {
                            self.eval_div(s)
                        },
                        "*" => {
                            self.eval_mul(s)
                        },
                        "<" => {
                            self.eval_lt(s)
                        },
                        ">" => {
                            self.eval_gt(s)
                        },
                        "=" => {
                            self.eval_eq(s)
                        },
                        _ => {
                            self.eval_call(s)
                        },
                    }
                } else {
                    Err(EvalError::EvalError)
                }
            }
        }
    }

    fn eval_eq(&mut self, s: &Expr) -> EvalResult {
        match *s {
            Expr::List(ref l) => {
                if let Expr::Symbol(ref n) = l[0] {
                    match &n[..] {
                        "=" => {
                            if l.len() > 2 {
                                let mut a = try_number!(self.eval(&l[1]));
                                for e in &l[2..] {
                                    let n = try_number!(self.eval(e));
                                    if a == n {
                                        a = n
                                    } else {
                                        return Ok(Val::Bool(false))
                                    }
                                }
                                Ok(Val::Bool(true))
                            } else {
                                Err(EvalError::EvalError)
                            }
                        },
                        _ => {
                            Err(EvalError::EvalError)
                        }
                    }
                } else {
                    Err(EvalError::EvalError)
                }
            },
            _ => {
                Err(EvalError::EvalError)
            }
        }
    }

    fn eval_gt(&mut self, s: &Expr) -> EvalResult {
        match *s {
            Expr::List(ref l) => {
                if let Expr::Symbol(ref n) = l[0] {
                    match &n[..] {
                        ">" => {
                            if l.len() > 2 {
                                let mut a = try_number!(self.eval(&l[1]));
                                for e in &l[2..] {
                                    let n = try_number!(self.eval(e));
                                    if a > n {
                                        a = n
                                    } else {
                                        return Ok(Val::Bool(false))
                                    }
                                }
                                Ok(Val::Bool(true))
                            } else {
                                Err(EvalError::EvalError)
                            }
                        },
                        _ => {
                            Err(EvalError::EvalError)
                        }
                    }
                } else {
                    Err(EvalError::EvalError)
                }
            },
            _ => {
                Err(EvalError::EvalError)
            }
        }
    }

    fn eval_lt(&mut self, s: &Expr) -> EvalResult {
        match *s {
            Expr::List(ref l) => {
                if let Expr::Symbol(ref n) = l[0] {
                    match &n[..] {
                        "<" => {
                            if l.len() > 2 {
                                let mut a = try_number!(self.eval(&l[1]));
                                for e in &l[2..] {
                                    let n = try_number!(self.eval(e));
                                    if a < n {
                                        a = n
                                    } else {
                                        return Ok(Val::Bool(false))
                                    }
                                }
                                Ok(Val::Bool(true))
                            } else {
                                Err(EvalError::EvalError)
                            }
                        },
                        _ => {
                            Err(EvalError::EvalError)
                        }
                    }
                } else {
                    Err(EvalError::EvalError)
                }
            },
            _ => {
                Err(EvalError::EvalError)
            }
        }
    }

    fn eval_call(&mut self, s: &Expr) -> EvalResult {
        match *s {
            Expr::List(ref l) => {
                if let Expr::Symbol(ref n) = l[0] {
                    let fun = match self.env.get(n) {
                        Some(v) => {
                            v.clone()
                        },
                        _ => {
                            return Err(EvalError::EvalError)
                        }
                    };

                    let mut e_params = vec![];
                    for e in &l[1..] {
                        e_params.push(try!(self.eval(e)))
                    }

                    let mut ctx = Context::new();

                    match fun {
                        Val::Fn { ref params, ref body } => {
                            for (p, e) in params.iter().zip(e_params.iter()) {
                                match p {
                                    &Expr::Symbol(ref name) => {
                                        ctx.env.insert(name.clone(), e.clone());
                                    },
                                    _ => {
                                        return Err(EvalError::EvalError)
                                    }
                                }
                            }
                            let mut result = v_list![];
                            for e in body {
                                result = try!(ctx.eval(e))
                            }
                            Ok(result)
                        },
                        _ => {
                            Err(EvalError::EvalError)
                        }
                    }
                } else {
                    Err(EvalError::EvalError)
                }
            },
            _ => {
                Err(EvalError::EvalError)
            }
        }
    }

    fn eval_def(&mut self, s: &Expr) -> EvalResult {
        if let Expr::List(ref l) = *s {
            if let Expr::Symbol(ref n) = l[0] {
                match &n[..] {
                    "def" => {
                        if l.len() == 3 {
                            if let Expr::Symbol(ref n) = l[1] {
                                self.eval(&l[2]).and_then(|v| {
                                    self.env.insert(n.clone(), v.clone());
                                    Ok(v)
                                })
                            } else {
                                Err(EvalError::EvalError)
                            }
                        } else {
                            Err(EvalError::IncorrectNumberOfArguments)
                        }
                    },
                    _ => {
                        Err(EvalError::IncorrectSpecialForm)
                    }
                }
            } else {
                Err(EvalError::EvalError)
            }
        } else {
            Err(EvalError::EvalError)
        }
    }

    fn eval_fn(&mut self, s: &Expr) -> EvalResult {
        if let Expr::List(ref l) = *s {
            if let Expr::Symbol(ref n) = l[0] {
                match &n[..] {
                    "fn" => {
                        if l.len() >= 3 {
                            if let Expr::List(ref params) = l[1] {
                                Ok(Val::Fn {
                                    params: params.iter().cloned().collect::<Vec<Expr>>(),
                                    body: l.iter().skip(2).cloned().collect::<Vec<Expr>>()
                                })
                            } else {
                                Err(EvalError::EvalError)
                            }
                        } else {
                            Err(EvalError::IncorrectNumberOfArguments)
                        }
                    },
                    _ => {
                        Err(EvalError::IncorrectSpecialForm)
                    }
                }
            } else {
                Err(EvalError::EvalError)
            }
        } else {
            Err(EvalError::EvalError)
        }
    }

    fn eval_plus(&mut self, s: &Expr) -> EvalResult {
        if let Expr::List(ref l) = *s {
            if let Expr::Symbol(ref n) = l[0] {
                match &n[..] {
                    "+" => {
                        if l.len() > 1 {
                            let mut a = 0_f64;
                            for i in &l[1..] {
                                a += try_number!(self.eval(i));
                            }
                            Ok(Val::Number(a))
                        } else {
                            Err(EvalError::IncorrectNumberOfArguments)
                        }
                    },
                    _ => {
                        Err(EvalError::IncorrectSpecialForm)
                    }
                }
            } else {
                Err(EvalError::EvalError)
            }
        } else {
            Err(EvalError::EvalError)
        }
    }

    fn eval_minus(&mut self, s: &Expr) -> EvalResult {
        if let Expr::List(ref l) = *s {
            if let Expr::Symbol(ref n) = l[0] {
                match &n[..] {
                    "-" => {
                        if l.len() > 1 {
                            if let Expr::Number(n) = l[1] {
                                let mut a = n;
                                for i in &l[2..] {
                                    a -= try_number!(self.eval(i));
                                }
                                Ok(Val::Number(a))
                            } else {
                                Err(EvalError::IncorrectTypeOfArgument)
                            }
                        } else {
                            Err(EvalError::IncorrectNumberOfArguments)
                        }
                    },
                    _ => {
                        Err(EvalError::IncorrectSpecialForm)
                    }
                }
            } else {
                Err(EvalError::EvalError)
            }
        } else {
            Err(EvalError::EvalError)
        }
    }

    fn eval_div(&mut self, s: &Expr) -> EvalResult {
        if let Expr::List(ref l) = *s {
            if let Expr::Symbol(ref n) = l[0] {
                match &n[..] {
                    "/" => {
                        if l.len() > 1 {
                            if let Expr::Number(n) = l[1] {
                                let mut a = n;
                                for i in &l[2..] {
                                    a /= try_number!(self.eval(i))
                                }
                                Ok(Val::Number(a))
                            } else {
                                Err(EvalError::IncorrectTypeOfArgument)
                            }
                        } else {
                            Err(EvalError::IncorrectNumberOfArguments)
                        }
                    },
                    _ => {
                        Err(EvalError::IncorrectSpecialForm)
                    }
                }
            } else {
                Err(EvalError::EvalError)
            }
        } else {
            Err(EvalError::EvalError)
        }
    }

    fn eval_mul(&mut self, s: &Expr) -> EvalResult {
        if let Expr::List(ref l) = *s {
            if let Expr::Symbol(ref n) = l[0] {
                match &n[..] {
                    "*" => {
                        if l.len() > 1 {
                            let mut a = 1_f64;
                            for i in &l[1..] {
                                a *= try_number!(self.eval(i));
                            }
                            Ok(Val::Number(a))
                        } else {
                            Err(EvalError::IncorrectNumberOfArguments)
                        }
                    },
                    _ => {
                        Err(EvalError::IncorrectSpecialForm)
                    }
                }
            } else {
                Err(EvalError::EvalError)
            }
        } else {
            Err(EvalError::EvalError)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Context;
    use super::EvalError::EvalError;

    #[test]
    fn test_eval_number_to_itself() {
        let num = 10_f64;
        let mut ctx = Context::new();
        let expected_result = v_number!(num);
        let actual_result = ctx.eval(&e_number!(num));
        assert_eq!(expected_result, actual_result.ok().unwrap());
    }

    #[test]
    fn test_eval_string_to_itself() {
        let s = "rust is awesome";
        let mut ctx = Context::new();
        let expected_result = v_string!(s);
        let actual_result = ctx.eval(&e_string!(s));
        assert_eq!(expected_result, actual_result.ok().unwrap());
    }

    #[test]
    fn test_eval_undefined_symbol_to_error() {
        let mut ctx = Context::new();
        let expected_result = EvalError;
        let actual_result = ctx.eval(&e_symbol!("a"));
        assert_eq!(expected_result, actual_result.err().unwrap());
    }

    #[test]
    fn test_eval_true_to_matching_bool() {
        let mut ctx = Context::new();
        let expected_result = v_bool!(true);
        let actual_result = ctx.eval(&e_symbol!("true"));
        assert_eq!(expected_result, actual_result.ok().unwrap());
    }

    #[test]
    fn test_eval_false_to_matching_bool() {
        let mut ctx = Context::new();
        let expected_result = v_bool!(false);
        let actual_result = ctx.eval(&e_symbol!("false"));
        assert_eq!(expected_result, actual_result.ok().unwrap());
    }

    #[test]
    fn test_eval_nil_to_empty_list() {
        let mut ctx = Context::new();
        let expected_result = v_list![];
        let actual_result = ctx.eval(&e_symbol!("nil"));
        assert_eq!(expected_result, actual_result.ok().unwrap());
    }

    #[test]
    fn test_eval_def_special_form() {
        let num = 1_f64;
        let mut ctx = Context::new();
        let expected_result = v_number!(num);
        let actual_input = e_list![e_symbol!("def"), e_symbol!("a"), e_number!(num)];
        let actual_result = ctx.eval(&actual_input);
        assert_eq!(expected_result, actual_result.ok().unwrap());
    }

    #[test]
    fn test_eval_fn_special_form_and_call_define_function() {
        let mut ctx = Context::new();
        let fun = e_list![e_symbol!("fn"),
                          e_list![e_symbol!("a"), e_symbol!("b")],
                          e_list![e_symbol!("+"), e_symbol!("a"), e_symbol!("b")]];
        ctx.eval(&e_list![e_symbol!("def"), e_symbol!("add"), fun]).ok().unwrap();
        let expected_result = v_number!(3_f64);
        let actual_result = ctx.eval(&e_list![e_symbol!("add"),
                                              e_number!(1_f64),
                                              e_number!(2_f64)]);
        assert_eq!(expected_result, actual_result.ok().unwrap());
    }

    #[test]
    fn test_eval_plus_builtin_fn() {
        let mut ctx = Context::new();
        ctx.eval(&e_list![e_symbol!("def"), e_symbol!("a"), e_number!(1_f64)]).ok().unwrap();
        ctx.eval(&e_list![e_symbol!("def"), e_symbol!("b"), e_number!(2_f64)]).ok().unwrap();
        let actual_input = e_list![e_symbol!("+"),
                                   e_list![e_symbol!("+"), e_symbol!("a"), e_symbol!("b")],
                                   e_number!(3_f64)];
        assert_eq!(v_number!(6_f64), ctx.eval(&actual_input).ok().unwrap());
    }

    #[test]
    fn test_eval_minus_builtin_fn() {
        let mut ctx = Context::new();
        let actual_input = e_list![e_symbol!("-"), e_number!(3_f64), e_number!(2_f64)];
        let actual_result = ctx.eval(&actual_input);
        let expected_result = v_number!(1_f64);
        assert_eq!(expected_result, actual_result.ok().unwrap());
    }

    #[test]
    fn test_eval_div_builtin_fn() {
        let mut ctx = Context::new();
        let actual_input = e_list![e_symbol!("/"), e_number!(3_f64), e_number!(2_f64)];
        let actual_result = ctx.eval(&actual_input);
        let expected_result = v_number!(1.5);
        assert_eq!(expected_result, actual_result.ok().unwrap());
    }

    #[test]
    fn test_eval_mul_builtin_fn() {
        let mut ctx = Context::new();
        let actual_input = e_list![e_symbol!("*"), e_number!(3.5), e_number!(2_f64)];
        let actual_result = ctx.eval(&actual_input);
        let expected_result = v_number!(7_f64);
        assert_eq!(expected_result, actual_result.ok().unwrap());
    }

    #[test]
    fn test_eval_lt_builtin_fn_positive_case() {
        let mut ctx = Context::new();
        let actual_input = e_list![e_symbol!("<"),
                                   e_number!(1_f64),
                                   e_number!(2_f64),
                                   e_number!(3_f64)];
        let actual_result = ctx.eval(&actual_input);
        let expected_result = v_bool!(true);
        assert_eq!(expected_result, actual_result.ok().unwrap());
    }

    #[test]
    fn test_eval_lt_builtin_fn_negative_case() {
        let mut ctx = Context::new();
        let actual_input = e_list![e_symbol!("<"),
                                   e_number!(3.5),
                                   e_number!(20_f64),
                                   e_number!(1_f64)];
        let actual_result = ctx.eval(&actual_input);
        let expected_result = v_bool!(false);
        assert_eq!(expected_result, actual_result.ok().unwrap());
    }

    #[test]
    fn test_eval_gt_builtin_fn_positive_case() {
        let mut ctx = Context::new();
        ctx.eval(&e_list![e_symbol!("def"), e_symbol!("a"), e_number!(3_f64)]).ok().unwrap();
        let actual_input = e_list![e_symbol!(">"),
                                   e_symbol!("a"),
                                   e_number!(2_f64),
                                   e_number!(1_f64)];
        let actual_result = ctx.eval(&actual_input);
        let expected_result = v_bool!(true);
        assert_eq!(expected_result, actual_result.ok().unwrap());
    }

    #[test]
    fn test_eval_gt_builtin_fn_negative_case() {
        let mut ctx = Context::new();
        ctx.eval(&e_list![e_symbol!("def"), e_symbol!("a"), e_number!(20_f64)]).ok().unwrap();
        let actual_input = e_list![e_symbol!(">"),
                                   e_number!(3.5),
                                   e_symbol!("a"),
                                   e_number!(1_f64)];
        let actual_result = ctx.eval(&actual_input);
        let expected_result = v_bool!(false);
        assert_eq!(expected_result, actual_result.ok().unwrap());
    }

    #[test]
    fn test_eval_eq_builtin_fn_positive_case() {
        let mut ctx = Context::new();
        ctx.eval(&e_list![e_symbol!("def"), e_symbol!("a"), e_number!(3_f64)]).ok().unwrap();
        let actual_input = e_list![e_symbol!("="),
                                   e_symbol!("a"),
                                   e_number!(3_f64),
                                   e_number!(3_f64)];
        let actual_result = ctx.eval(&actual_input);
        let expected_result = v_bool!(true);
        assert_eq!(expected_result, actual_result.ok().unwrap());
    }

    #[test]
    fn test_eval_eq_builtin_fn_negative_case() {
        let mut ctx = Context::new();
        ctx.eval(&e_list![e_symbol!["def"], e_symbol!["a"], e_number![1_f64]]).ok().unwrap();
        let actual_input = e_list![e_symbol!["="],
                                   e_number![3.5],
                                   e_number![20_f64],
                                   e_symbol!["a"]];
        let actual_result = ctx.eval(&actual_input);
        let expected_result = v_bool![false];
        assert_eq!(expected_result, actual_result.ok().unwrap());
    }
}
