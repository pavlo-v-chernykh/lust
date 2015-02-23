use std::collections::HashMap;
use common::{Atom, Sexp};
use parser::{Parser};

#[derive(Debug, PartialEq)]
enum EvalError{
    EvalError,
    IncorrectSpecialForm,
    IncorrectNumberOfArguments,
    IncorrectTypeOfArgument
}

struct Env {
    current: HashMap<String, Sexp>,
}

impl Env {
    fn new() -> Env {
        let mut env = Env {
            current: HashMap::new(),
        };
        env.current.insert("nil".to_string(),
                           Sexp::Atom(Atom::Symbol("nil".to_string())));
        env.current.insert("true".to_string(),
                           Sexp::Atom(Atom::Bool(true)));
        env.current.insert("false".to_string(),
                           Sexp::Atom(Atom::Bool(false)));
        env
    }

    fn get(&self, k: &String) -> Option<&Sexp> {
        self.current.get(k)
    }

    fn insert(&mut self, k: String, v: Sexp) -> Option<Sexp> {
        self.current.insert(k, v)
    }
}


pub struct Interpreter {
    env: Env
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            env: Env::new(),
        }
    }

    pub fn eval<T: Iterator<Item=char>>(&mut self, src: T) -> Result<Sexp, EvalError> {
        match Parser::new(src).parse() {
            Ok(sexp) => {
                self.eval_sexp(&sexp)
            },
            Err(_) => {
                Err(EvalError::EvalError)
            }
        }
    }

    fn eval_sexp(&mut self, s: &Sexp) -> Result<Sexp, EvalError> {
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
                                self.eval_sexp(&l[2]).and_then(|a| {
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
                                match self.eval_sexp(i) {
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
                                    match self.eval_sexp(i) {
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
                                    match self.eval_sexp(i) {
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
                                match self.eval_sexp(i) {
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
    use super::Interpreter;
    use super::EvalError::EvalError;

    #[test]
    fn test_eval_number_to_itself() {
        let number = 10_f64;
        let s = format!("{}", number);
        let mut intr = Interpreter::new();
        let expected_result = Sexp::Atom(Atom::Number(number));
        let actual_result = intr.eval(s.chars());
        assert_eq!(expected_result, actual_result.ok().unwrap());
    }

    #[test]
    fn test_eval_string_to_itself() {
        let s = "rust is awesome";
        let actual_input = format!(r#""{}""#, s);
        let expected_result = Sexp::Atom(Atom::String(s.to_string()));
        let mut intr = Interpreter::new();
        let actual_result = intr.eval(actual_input.chars());
        assert_eq!(expected_result, actual_result.ok().unwrap());
    }

    #[test]
    fn test_eval_undefined_symbol_to_error() {
        let mut intr = Interpreter::new();
        let expected_result = EvalError;
        let actual_result = intr.eval("a".chars());
        assert_eq!(expected_result, actual_result.err().unwrap());
    }

    #[test]
    fn test_eval_true_to_matching_bool() {
        let mut intr = Interpreter::new();
        let expected_result = Sexp::Atom(Atom::Bool(true));
        let actual_result = intr.eval("true".chars());
        assert_eq!(expected_result, actual_result.ok().unwrap());
    }

    #[test]
    fn test_eval_false_to_matching_bool() {
        let mut intr = Interpreter::new();
        let expected_result = Sexp::Atom(Atom::Bool(false));
        let actual_result = intr.eval("false".chars());
        assert_eq!(expected_result, actual_result.ok().unwrap());
    }

    #[test]
    fn test_eval_nil_to_itself() {
        let mut intr = Interpreter::new();
        let expected_result = Sexp::Atom(Atom::Symbol("nil".to_string()));
        let actual_result = intr.eval("nil".chars());
        assert_eq!(expected_result, actual_result.ok().unwrap());
    }

    #[test]
    fn test_eval_def_special_form() {
        let mut intr = Interpreter::new();
        let actual_result = intr.eval("(def a 1)".chars());
        let expected_result = Sexp::Atom(Atom::Number(1_f64));
        assert_eq!(expected_result, actual_result.ok().unwrap());
    }

    #[test]
    fn test_eval_nested_plus_special_form() {
        let mut intr = Interpreter::new();
        let actual_result = intr.eval("(+ (+ 1 2) 3)".chars());
        let expected_result = Sexp::Atom(Atom::Number(6_f64));
        assert_eq!(expected_result, actual_result.ok().unwrap());
    }

    #[test]
    fn test_eval_minus_special_form() {
        let mut intr = Interpreter::new();
        let actual_result = intr.eval("(- 3 2)".chars());
        let expected_result = Sexp::Atom(Atom::Number(1_f64));
        assert_eq!(expected_result, actual_result.ok().unwrap());
    }

    #[test]
    fn test_eval_div_special_form() {
        let mut intr = Interpreter::new();
        let actual_result = intr.eval("(/ 3 2)".chars());
        let expected_result = Sexp::Atom(Atom::Number(1.5));
        assert_eq!(expected_result, actual_result.ok().unwrap());
    }

    #[test]
    fn test_eval_mul_special_form() {
        let mut intr = Interpreter::new();
        let actual_result = intr.eval("(* 3.5 2)".chars());
        let expected_result = Sexp::Atom(Atom::Number(7_f64));
        assert_eq!(expected_result, actual_result.ok().unwrap());
    }

    #[test]
    fn test_eval_plus_special_form_using_defined_symbols() {
        let mut intr = Interpreter::new();
        intr.eval("(def a 1)".chars()).ok().unwrap();
        intr.eval("(def b 2)".chars()).ok().unwrap();
        let expected_result = Sexp::Atom(Atom::Number(3_f64));
        let actual_result = intr.eval("(+ a b)".chars());
        assert_eq!(expected_result, actual_result.ok().unwrap());
    }
}
