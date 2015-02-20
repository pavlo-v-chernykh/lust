use std::collections::HashMap;
use common::Atom::{Number, Symbol};
use common::Atom::String as AtomString;
use common::Sexp::{self, Atom, List};

#[derive(Debug, PartialEq)]
enum EvalError{
    EvalError,
    IncorrectSpecialForm,
    IncorrectNumberOfArguments,
    IncorrectTypeOfArgument
}

fn eval_def(s: &Sexp, e: &mut HashMap<String, Sexp>) -> Result<Sexp, EvalError> {
    if let List(ref l) = *s {
        if let Atom(Symbol(ref n)) = *l.first().unwrap() {
            match &n[] {
                "def" => {
                    if l.len() == 3 {
                        if let Atom(Symbol(ref n)) = l[1] {
                            eval(&l[2], e).and_then(|a| {
                                e.insert(n.clone(), a.clone());
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

fn eval_plus(s: &Sexp, e: &mut HashMap<String, Sexp>) -> Result<Sexp, EvalError> {
    if let List(ref l) = *s {
        if let Atom(Symbol(ref n)) = *l.first().unwrap() {
            match &n[] {
                "+" => {
                    if l.len() > 1 {
                        let mut a = 0f64;
                        for i in l.iter().skip(1) {
                            match eval(i, e) {
                                Ok(Atom(Number(n))) => {
                                    a += n
                                },
                                _ => {
                                    return Err(EvalError::IncorrectTypeOfArgument)
                                }
                            }
                        }
                        return Ok(Atom(Number(a)))
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

fn eval(s: &Sexp, e: &mut HashMap<String, Sexp>) -> Result<Sexp, EvalError> {
    match *s {
        Atom(Number(_)) => {
            Ok(s.clone())
        },
        Atom(Symbol(ref name)) => {
            if let Some(s) = e.get(name) {
                Ok(s.clone())
            } else {
                Err(EvalError::EvalError)
            }
        },
        Atom(AtomString(ref s)) => {
            Ok(Atom(AtomString(s.clone())))
        },
        List(ref l) if l.is_empty() => {
            Ok(Atom(Symbol("nil".to_string())))
        },
        List(ref l) => {
            if let Atom(Symbol(ref n)) = *l.first().unwrap() {
                match &n[] {
                    "def" => {
                        eval_def(s, e)
                    },
                    "+" => {
                        eval_plus(s, e)
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

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use common::Atom::{Number, Symbol};
    use common::Sexp::{Atom, List};
    use super::EvalError::EvalError;
    use super::eval;

    #[test]
    fn test_eval_atom_number_to_itself() {
        let number = 10f64;
        let expected_result = Atom(Number(number));
        let actual_result = eval(&Atom(Number(number)), &mut HashMap::new());
        assert_eq!(expected_result, actual_result.ok().unwrap());
    }

    #[test]
    fn test_eval_atom_symbol_to_number() {
        let num = 10f64;
        let mut env = HashMap::new();
        env.insert("a".to_string(), Atom(Number(num)));
        let expected_result = Atom(Number(num));
        let actual_result = eval(&Atom(Symbol("a".to_string())), &mut env);
        assert_eq!(expected_result, actual_result.ok().unwrap());
    }

    #[test]
    fn test_eval_atom_symbol_to_nil() {
        let mut env = HashMap::new();
        env.insert("a".to_string(), Atom(Symbol("nil".to_string())));
        let expected_result = Atom(Symbol("nil".to_string()));
        let actual_result = eval(&Atom(Symbol("a".to_string())), &mut env);
        assert_eq!(expected_result, actual_result.ok().unwrap());
    }

    #[test]
    fn test_eval_atom_symbol_to_non_value() {
        let mut env = HashMap::new();
        let expected_result = EvalError;
        let actual_result = eval(&Atom(Symbol("a".to_string())), &mut env);
        assert_eq!(expected_result, actual_result.err().unwrap());
    }

    #[test]
    fn test_eval_atom_nil_to_itself() {
        let mut env = HashMap::new();
        env.insert("nil".to_string(), Atom(Symbol("nil".to_string())));
        let expected_result = Atom(Symbol("nil".to_string()));
        let actual_result = eval(&Atom(Symbol("nil".to_string())), &mut env);
        assert_eq!(expected_result, actual_result.ok().unwrap());
    }

    #[test]
    fn test_eval_empty_list_to_nil() {
        let mut env = HashMap::new();
        let expected_result = Atom(Symbol("nil".to_string()));
        let actual_result = eval(&List(vec![]), &mut env);
        assert_eq!(expected_result, actual_result.ok().unwrap());
    }

    #[test]
    fn test_eval_def_special_form() {
        let mut env = HashMap::new();
        let num = 1f64;
        let actual_input = List(vec![Atom(Symbol("def".to_string())),
                                     Atom(Symbol("a".to_string())),
                                     Atom(Number(num))]);
        let actual_result = eval(&actual_input, &mut env);
        let expected_result = Atom(Number(num));
        assert_eq!(expected_result, actual_result.ok().unwrap());
    }

    #[test]
    fn test_eval_nested_plus_special_form() {
        let actual_input = List(vec![Atom(Symbol("+".to_string())),
                                     List(vec![Atom(Symbol("+".to_string())),
                                               Atom(Number(1f64)),
                                               Atom(Number(2f64))]),
                                     Atom(Number(3f64))]);
        let actual_result = eval(&actual_input, &mut HashMap::new());
        let expected_result = Atom(Number(6f64));
        assert_eq!(expected_result, actual_result.ok().unwrap());
    }
}
