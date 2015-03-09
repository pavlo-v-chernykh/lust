#[macro_use]
mod macros;
mod error;

use expr::Expr;
use scope::Scope;
use self::error::{ExpandError, ExpandErrorCode};

#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    Number(f64),
    String(String),
    Symbol(String),
    List(Vec<Node>),
}

type ExpandResult = Result<Expr, ExpandError>;

impl Node {
    pub fn expand(&self, scope: &mut Scope) -> ExpandResult {
        match self {
            &Node::Number(n) => {
                Ok(Expr::Number(n))
            },
            &Node::String(ref s) => {
                Ok(Expr::String(s.clone()))
            },
            &Node::Symbol(ref s) => {
                Ok(Expr::Symbol(s.clone()))
            },
            &Node::List(ref l) => {
                if let Node::Symbol(ref n) = l[0] {
                    match &n[..] {
                        "def" => {
                            self.expand_def(scope)
                        },
                        "fn" => {
                            self.expand_fn(scope)
                        },
                        "macro" => {
                            self.expand_macro(scope)
                        },
                        "quote" => {
                            self.expand_quote(scope)
                        },
                        _ => {
                            self.expand_call(scope)
                        }
                    }
                } else {
                    Node::error(ExpandErrorCode::UnknownError)
                }
            },
        }
    }

    fn expand_quoted(&self, scope: &mut Scope) -> ExpandResult {
        match self {
            &Node::List(ref l) => {
                let mut v = vec![];
                for i in l {
                    v.push(try!(i.expand_quoted(scope)));
                }
                Ok(Expr::List(v))
            },
            _ => {
                self.expand(scope)
            }
        }
    }

    fn expand_def(&self, scope: &mut Scope) -> ExpandResult {
        if let &Node::List(ref l) = self {
            if l.len() == 3 {
                if let Node::Symbol(ref n) = l[1] {
                    Ok(Expr::Def {
                        sym: n.clone(),
                        expr: Box::new(try!(l[2].expand(scope))),
                    })
                } else {
                    Node::error(ExpandErrorCode::UnknownError)
                }
            } else {
                Node::error(ExpandErrorCode::UnknownError)
            }
        } else {
            Node::error(ExpandErrorCode::UnknownError)
        }
    }

    fn expand_fn(&self, scope: &mut Scope) -> ExpandResult {
        if let &Node::List(ref l) = self {
            if l.len() >= 3 {
                if let Node::List(ref params) = l[1] {
                    let mut fn_params = vec![];
                    for p in params {
                        fn_params.push(try!(p.expand(scope)))
                    }
                    let mut fn_body = vec![];
                    for be in &l[2..] {
                        fn_body.push(try!(be.expand(scope)))
                    }
                    Ok(Expr::Fn {
                        params: fn_params,
                        body: fn_body,
                    })
                } else {
                    Node::error(ExpandErrorCode::UnknownError)
                }
            } else {
                Node::error(ExpandErrorCode::UnknownError)
            }
        } else {
            Node::error(ExpandErrorCode::UnknownError)
        }
    }

    fn expand_macro(&self, scope: &mut Scope) -> ExpandResult {
        if let &Node::List(ref l) = self {
            if l.len() >= 3 {
                if let Node::List(ref params) = l[1] {
                    let mut macro_params = vec![];
                    for p in params {
                        macro_params.push(try!(p.expand(scope)))
                    }
                    let mut macro_body = vec![];
                    for be in &l[2..] {
                        macro_body.push(try!(be.expand(scope)))
                    }
                    Ok(Expr::Macro {
                        params: macro_params,
                        body: macro_body,
                    })
                } else {
                    Node::error(ExpandErrorCode::UnknownError)
                }
            } else {
                Node::error(ExpandErrorCode::UnknownError)
            }
        } else {
            Node::error(ExpandErrorCode::UnknownError)
        }
    }

    fn expand_quote(&self, scope: &mut Scope) -> ExpandResult {
        if let &Node::List(ref l) = self {
            if l.len() == 2 {
                Ok(Expr::Call {
                    name: "quote".to_string(),
                    args: vec![try!(l[1].expand_quoted(scope))],
                })
            } else {
                Node::error(ExpandErrorCode::UnknownError)
            }
        } else {
            Node::error(ExpandErrorCode::UnknownError)
        }
    }

    fn expand_call(&self, scope: &mut Scope) -> ExpandResult {
        if let &Node::List(ref l) = self {
            if let Node::Symbol(ref name) = l[0] {
                let mut args = vec![];
                for a in &l[1..] {
                    args.push(try!(a.expand(scope)))
                }
                let call = Expr::Call {
                    name: name.clone(),
                    args: args
                };
                let is_macro = match scope.get(name) {
                    Some(&Expr::Macro { .. }) => true,
                    _ => false
                };
                if is_macro {
                    Ok(try!(call.eval(scope)))
                } else {
                    Ok(call)
                }
            } else {
                Node::error(ExpandErrorCode::UnknownError)
            }
        } else {
            Node::error(ExpandErrorCode::UnknownError)
        }
    }

    fn error(code: ExpandErrorCode) -> ExpandResult {
        Err(ExpandError::new(code))
    }
}

#[cfg(test)]
mod tests {
    use scope::Scope;

    #[test]
    fn test_expand_number() {
        let ref mut scope = Scope::new_std();
        let num = 1_f64;
        assert_eq!(e_number!(num), n_number!(num).expand(scope).ok().unwrap());
    }

    #[test]
    fn test_expand_string() {
        let ref mut scope = Scope::new_std();
        let s = "rust is wonderful";
        assert_eq!(e_string!(s), n_string!(s).expand(scope).ok().unwrap());
    }

    #[test]
    fn test_expand_symbol() {
        let ref mut scope = Scope::new_std();
        let s = "+";
        assert_eq!(e_symbol!(s), n_symbol!(s).expand(scope).ok().unwrap());
    }

    #[test]
    fn test_expand_fn() {
        let ref mut scope = Scope::new_std();
        let e = e_fn!([e_symbol!("a")],
                      [e_call!["+", e_symbol!("a"), e_number!(1_f64)]]);
        let n = n_list![n_symbol!("fn"),
                        n_list![n_symbol!("a")],
                        n_list![n_symbol!("+"), n_symbol!("a"), n_number!(1_f64)]];
        assert_eq!(e, n.expand(scope).ok().unwrap());
    }

    #[test]
    fn test_expand_macro() {
        let ref mut scope = Scope::new_std();
        let e = e_macro!([e_symbol!("a")],
                         [e_call!["+", e_symbol!("a"), e_number!(1_f64)]]);
        let n = n_list![n_symbol!("macro"),
                        n_list![n_symbol!("a")],
                        n_list![n_symbol!("+"), n_symbol!("a"), n_number!(1_f64)]];
        assert_eq!(e, n.expand(scope).ok().unwrap());
    }

    #[test]
    fn test_expand_def() {
        let ref mut scope = Scope::new_std();
        let e = e_def!["a", e_number![1_f64]];
        let n = n_list![n_symbol!["def"], n_symbol!["a"], n_number![1_f64]];
        assert_eq!(e, n.expand(scope).ok().unwrap());
    }

    #[test]
    fn test_expand_call_fn() {
        let ref mut scope = Scope::new_std();
        let e = e_call!["+", e_symbol!["a"], e_number![1_f64]];
        let n = n_list![n_symbol!["+"], n_symbol!["a"], n_number![1_f64]];
        assert_eq!(e, n.expand(scope).ok().unwrap());
    }

    #[test]
    fn expand_call_macro() {
        let ref mut scope = Scope::new_std();
        let m = e_def!["m", e_macro![[e_symbol!["a"], e_symbol!["b"]],
                                     [e_call!["+", e_symbol!["a"], e_symbol!["b"]]]]];
        m.eval(scope).ok().unwrap();
        let e = e_number![3.];
        let n = n_list![n_symbol!["m"], n_number![1.], n_number![2.]];
        assert_eq!(e, n.expand(scope).ok().unwrap());
    }

    #[test]
    fn test_expand_quote() {
        let ref mut scope = Scope::new_std();
        let n = n_list![n_symbol!["quote"], n_symbol!["a"]];
        assert_eq!(e_call!["quote", e_symbol!["a"]], n.expand(scope).ok().unwrap());
        let n = n_list![n_symbol!["quote"],
                        n_list![n_symbol!["+"], n_symbol!["a"], n_symbol!["b"]]];
        assert_eq!(e_call!["quote", e_list![e_symbol!["+"], e_symbol!["a"], e_symbol!["b"]]],
                   n.expand(scope).ok().unwrap());
    }
}
