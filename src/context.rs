use std::collections::HashMap;
use std::rc::Rc;
use common::{Atom, Sexp};

#[derive(Debug, PartialEq)]
enum EvalError{
    EvalError,
    IncorrectSpecialForm,
    IncorrectNumberOfArguments,
    IncorrectTypeOfArgument
}

pub struct Context {
    parent: Option<Rc<Context>>,
    env: HashMap<String, Sexp>,
}

impl Context {
    pub fn new() -> Context {
        let mut intr = Context {
            parent: None,
            env: HashMap::new(),
        };
        intr.env.insert("nil".to_string(),
                        Sexp::Atom(Atom::Symbol("nil".to_string())));
        intr.env.insert("true".to_string(),
                        Sexp::Atom(Atom::Bool(true)));
        intr.env.insert("false".to_string(),
                        Sexp::Atom(Atom::Bool(false)));
        intr
    }

    pub fn eval(&mut self, s: &Sexp) -> Result<Sexp, EvalError> {
        match *s {
            Sexp::Atom(Atom::Number(_)) => {
                Ok(s.clone())
            },
            Sexp::Atom(Atom::Bool(_)) => {
                Ok(s.clone())
            },
            Sexp::Atom(Atom::Symbol(ref name)) => {
                if let Some(s) = self.env.get(name) {
                    Ok(s.clone())
                } else {
                    Err(EvalError::EvalError)
                }
            },
            Sexp::Atom(Atom::String(ref s)) => {
                Ok(Sexp::Atom(Atom::String(s.clone())))
            },
            Sexp::List(ref l) => {
                if let Sexp::Atom(Atom::Symbol(ref n)) = *l.first().unwrap() {
                    match &n[..] {
                        "def" => {
                            self.eval_def(s)
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
                        _ => {
                            Err(EvalError::EvalError)
                        }
                    }
                } else {
                    Err(EvalError::EvalError)
                }
            }
        }
    }

    fn eval_def(&mut self, s: &Sexp) -> Result<Sexp, EvalError> {
        if let Sexp::List(ref l) = *s {
            if let Sexp::Atom(Atom::Symbol(ref n)) = *l.first().unwrap() {
                match &n[..] {
                    "def" => {
                        if l.len() == 3 {
                            if let Sexp::Atom(Atom::Symbol(ref n)) = l[1] {
                                self.eval(&l[2]).and_then(|a| {
                                    self.env.insert(n.clone(), a.clone());
                                    Ok(a)
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

    fn eval_plus(&mut self, s: &Sexp) -> Result<Sexp, EvalError> {
        if let Sexp::List(ref l) = *s {
            if let Sexp::Atom(Atom::Symbol(ref n)) = *l.first().unwrap() {
                match &n[..] {
                    "+" => {
                        if l.len() > 1 {
                            let mut a = 0_f64;
                            for i in l.iter().skip(1) {
                                match self.eval(i) {
                                    Ok(Sexp::Atom(Atom::Number(n))) => {
                                        a += n
                                    },
                                    _ => {
                                        return Err(EvalError::IncorrectTypeOfArgument)
                                    }
                                }
                            }
                            Ok(Sexp::Atom(Atom::Number(a)))
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

    fn eval_minus(&mut self, s: &Sexp) -> Result<Sexp, EvalError> {
        if let Sexp::List(ref l) = *s {
            if let Sexp::Atom(Atom::Symbol(ref n)) = *l.first().unwrap() {
                match &n[..] {
                    "-" => {
                        if l.len() > 1 {
                            if let Sexp::Atom(Atom::Number(n)) = l[1] {
                                let mut a = n;
                                for i in l.iter().skip(2) {
                                    match self.eval(i) {
                                        Ok(Sexp::Atom(Atom::Number(n))) => {
                                            a -= n
                                        },
                                        _ => {
                                            return Err(EvalError::IncorrectTypeOfArgument)
                                        }
                                    }
                                }
                                Ok(Sexp::Atom(Atom::Number(a)))
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

    fn eval_div(&mut self, s: &Sexp) -> Result<Sexp, EvalError> {
        if let Sexp::List(ref l) = *s {
            if let Sexp::Atom(Atom::Symbol(ref n)) = *l.first().unwrap() {
                match &n[..] {
                    "/" => {
                        if l.len() > 1 {
                            if let Sexp::Atom(Atom::Number(n)) = l[1] {
                                let mut a = n;
                                for i in l.iter().skip(2) {
                                    match self.eval(i) {
                                        Ok(Sexp::Atom(Atom::Number(n))) => {
                                            a /= n
                                        },
                                        _ => {
                                            return Err(EvalError::IncorrectTypeOfArgument)
                                        }
                                    }
                                }
                                Ok(Sexp::Atom(Atom::Number(a)))
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

    fn eval_mul(&mut self, s: &Sexp) -> Result<Sexp, EvalError> {
        if let Sexp::List(ref l) = *s {
            if let Sexp::Atom(Atom::Symbol(ref n)) = *l.first().unwrap() {
                match &n[..] {
                    "*" => {
                        if l.len() > 1 {
                            let mut a = 1_f64;
                            for i in l.iter().skip(1) {
                                match self.eval(i) {
                                    Ok(Sexp::Atom(Atom::Number(n))) => {
                                        a *= n
                                    },
                                    _ => {
                                        return Err(EvalError::IncorrectTypeOfArgument)
                                    }
                                }
                            }
                            Ok(Sexp::Atom(Atom::Number(a)))
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
    use common::{Atom, Sexp};
    use super::Context;
    use super::EvalError::EvalError;

    #[test]
    fn test_eval_number_to_itself() {
        let num = 10_f64;
        let mut intr = Context::new();
        let expected_result = Sexp::Atom(Atom::Number(num));
        let actual_result = intr.eval(&Sexp::Atom(Atom::Number(num)));
        assert_eq!(expected_result, actual_result.ok().unwrap());
    }

    #[test]
    fn test_eval_string_to_itself() {
        let s = "rust is awesome";
        let mut intr = Context::new();
        let expected_result = Sexp::Atom(Atom::String(s.to_string()));
        let actual_result = intr.eval(&Sexp::Atom(Atom::String(s.to_string())));
        assert_eq!(expected_result, actual_result.ok().unwrap());
    }

    #[test]
    fn test_eval_undefined_symbol_to_error() {
        let mut intr = Context::new();
        let expected_result = EvalError;
        let actual_result = intr.eval(&Sexp::Atom(Atom::Symbol("a".to_string())));
        assert_eq!(expected_result, actual_result.err().unwrap());
    }

    #[test]
    fn test_eval_true_to_matching_bool() {
        let mut intr = Context::new();
        let expected_result = Sexp::Atom(Atom::Bool(true));
        let actual_result = intr.eval(&Sexp::Atom(Atom::Symbol("true".to_string())));
        assert_eq!(expected_result, actual_result.ok().unwrap());
    }

    #[test]
    fn test_eval_false_to_matching_bool() {
        let mut intr = Context::new();
        let expected_result = Sexp::Atom(Atom::Bool(false));
        let actual_result = intr.eval(&Sexp::Atom(Atom::Symbol("false".to_string())));
        assert_eq!(expected_result, actual_result.ok().unwrap());
    }

    #[test]
    fn test_eval_nil_to_itself() {
        let mut intr = Context::new();
        let expected_result = Sexp::Atom(Atom::Symbol("nil".to_string()));
        let actual_result = intr.eval(&Sexp::Atom(Atom::Symbol("nil".to_string())));
        assert_eq!(expected_result, actual_result.ok().unwrap());
    }

    #[test]
    fn test_eval_def_special_form() {
        let num = 1_f64;
        let mut intr = Context::new();
        let expected_result = Sexp::Atom(Atom::Number(num));
        let actual_input = Sexp::List(vec![Sexp::Atom(Atom::Symbol("def".to_string())),
                                           Sexp::Atom(Atom::Symbol("a".to_string())),
                                           Sexp::Atom(Atom::Number(num))]);
        let actual_result = intr.eval(&actual_input);
        assert_eq!(expected_result, actual_result.ok().unwrap());
    }

    #[test]
    fn test_eval_nested_plus_special_form() {
        let mut intr = Context::new();
        let actual_input = Sexp::List(vec![Sexp::Atom(Atom::Symbol("+".to_string())),
                                           Sexp::List(vec![Sexp::Atom(Atom::Symbol("+".to_string())),
                                                           Sexp::Atom(Atom::Number(1_f64)),
                                                           Sexp::Atom(Atom::Number(2_f64))]),
                                           Sexp::Atom(Atom::Number(3_f64))]);
        let actual_result = intr.eval(&actual_input);
        let expected_result = Sexp::Atom(Atom::Number(6_f64));
        assert_eq!(expected_result, actual_result.ok().unwrap());
    }

    #[test]
    fn test_eval_minus_special_form() {
        let mut intr = Context::new();
        let actual_input = Sexp::List(vec![Sexp::Atom(Atom::Symbol("-".to_string())),
                                           Sexp::Atom(Atom::Number(3_f64)),
                                           Sexp::Atom(Atom::Number(2_f64))]);
        let actual_result = intr.eval(&actual_input);
        let expected_result = Sexp::Atom(Atom::Number(1_f64));
        assert_eq!(expected_result, actual_result.ok().unwrap());
    }

    #[test]
    fn test_eval_div_special_form() {
        let mut intr = Context::new();
        let actual_input = Sexp::List(vec![Sexp::Atom(Atom::Symbol("/".to_string())),
                                           Sexp::Atom(Atom::Number(3_f64)),
                                           Sexp::Atom(Atom::Number(2_f64))]);
        let actual_result = intr.eval(&actual_input);
        let expected_result = Sexp::Atom(Atom::Number(1.5));
        assert_eq!(expected_result, actual_result.ok().unwrap());
    }

    #[test]
    fn test_eval_mul_special_form() {
        let mut intr = Context::new();
        let actual_input = Sexp::List(vec![Sexp::Atom(Atom::Symbol("*".to_string())),
                                           Sexp::Atom(Atom::Number(3.5)),
                                           Sexp::Atom(Atom::Number(2_f64))]);
        let actual_result = intr.eval(&actual_input);
        let expected_result = Sexp::Atom(Atom::Number(7_f64));
        assert_eq!(expected_result, actual_result.ok().unwrap());
    }

    #[test]
    fn test_eval_plus_special_form_using_defined_symbols() {
        let mut intr = Context::new();
        intr.eval(&Sexp::List(vec![Sexp::Atom(Atom::Symbol("def".to_string())),
                                   Sexp::Atom(Atom::Symbol("a".to_string())),
                                   Sexp::Atom(Atom::Number(1_f64))])).ok().unwrap();
        intr.eval(&Sexp::List(vec![Sexp::Atom(Atom::Symbol("def".to_string())),
                                   Sexp::Atom(Atom::Symbol("b".to_string())),
                                   Sexp::Atom(Atom::Number(2_f64))])).ok().unwrap();
        let expected_result = Sexp::Atom(Atom::Number(3_f64));
        let actual_result = intr.eval(&Sexp::List(vec![Sexp::Atom(Atom::Symbol("+".to_string())),
                                                       Sexp::Atom(Atom::Symbol("a".to_string())),
                                                       Sexp::Atom(Atom::Symbol("b".to_string()))]));
        assert_eq!(expected_result, actual_result.ok().unwrap());
    }
}
