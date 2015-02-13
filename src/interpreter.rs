use std::collections::HashMap;
use common::Atom::{Number, Symbol, Nil};
use common::Sexp::{self, Atom, List};

#[derive(Debug, PartialEq)]
enum EvalError{
    EvalError
}

fn eval(s: Sexp, e: &HashMap<String, Sexp>) -> Result<Sexp, EvalError> {
    match s {
        Atom(Number(_)) | Atom(Nil) => {
            Ok(s)
        },
        Atom(Symbol(ref name)) => {
            if let Some(s) = e.get(name) {
                Ok(s.clone())
            } else {
                Err(EvalError::EvalError)
            }
        },
        List(ref l) if l.is_empty() => {
            Ok(Atom(Nil))
        }
        _ => {
            Err(EvalError::EvalError)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use common::Atom::{Number, Symbol, Nil};
    use common::Sexp::{Atom, List};
    use super::EvalError::EvalError;
    use super::eval;

    #[test]
    fn test_eval_atom_number_to_itself() {
        let number = 10f64;
        let expected_result = Atom(Number(number));
        let actual_result = eval(Atom(Number(number)), &HashMap::new());
        assert_eq!(expected_result, actual_result.ok().unwrap());
    }

    #[test]
    fn test_eval_atom_symbol_to_number() {
        let num = 10f64;
        let mut env = HashMap::new();
        env.insert("a".to_string(), Atom(Number(num)));
        let expected_result = Atom(Number(num));
        let actual_result = eval(Atom(Symbol("a".to_string())), &env);
        assert_eq!(expected_result, actual_result.ok().unwrap());
    }

    #[test]
    fn test_eval_atom_symbol_to_nil() {
        let mut env = HashMap::new();
        env.insert("a".to_string(), Atom(Nil));
        let expected_result = Atom(Nil);
        let actual_result = eval(Atom(Symbol("a".to_string())), &env);
        assert_eq!(expected_result, actual_result.ok().unwrap());
    }

    #[test]
    fn test_eval_atom_symbol_to_non_value() {
        let env = HashMap::new();
        let expected_result = EvalError;
        let actual_result = eval(Atom(Symbol("a".to_string())), &env);
        assert_eq!(expected_result, actual_result.err().unwrap());
    }

    #[test]
    fn test_eval_atom_nil_to_itself() {
        let env = HashMap::new();
        let expected_result = Atom(Nil);
        let actual_result = eval(Atom(Nil), &env);
        assert_eq!(expected_result, actual_result.ok().unwrap());
    }

    #[test]
    fn test_eval_empty_list_to_nil() {
        let env = HashMap::new();
        let expected_result = Atom(Nil);
        let actual_result = eval(List(vec![]), &env);
        assert_eq!(expected_result, actual_result.ok().unwrap());
    }
}
