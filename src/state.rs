use std::collections::HashMap;
use scope::Scope;
use ast::{Expr, EvalResult};
use ast::EvalError::*;

pub struct State<'s> {
    default_ns: String,
    namespaces: HashMap<String, Scope<'s>>,
    parent: Option<&'s State<'s>>,
}

impl<'s> State<'s> {
    pub fn new(default: String) -> State<'s> {
        State {
            default_ns: default,
            namespaces: HashMap::new(),
            parent: None,
        }
    }

    pub fn insert(&mut self, ns: Option<String>, name: String, expr: Expr) -> Option<Expr> {
        if let Some(ns) = ns {
            if let Some(ns) = self.namespaces.get_mut(&ns) {
                return ns.insert(name, expr)
            }
        } else {
            if let Some(ns) = self.namespaces.get_mut(&self.default_ns) {
                return ns.insert(name, expr)
            }
        }
        None
    }

    pub fn eval(&self, expr: &Expr) -> EvalResult {
        match try!(self.expand(expr)) {
            ref sym_expr @ Expr::Symbol { .. } => {
                self.eval_symbol(sym_expr)
            },
            ref def_expr @ Expr::Def { .. } => {
                self.eval_def(def_expr)
            },
            ref call_expr @ Expr::Call { .. } => {
                self.eval_call(call_expr)
            },
            ref let_expr @ Expr::Let { .. } => {
                self.eval_let(let_expr)
            },
            other_expr => {
                Ok(other_expr)
            },
        }
    }

    fn eval_symbol(&self, expr: &Expr) -> EvalResult {
        if let Expr::Symbol { ref ns, ref name , .. } = *expr {
            let ns_name = ns.as_ref().unwrap_or(&self.default_ns);
            match self.namespaces.get(ns_name) {
                Some(scope) => {
                    scope.get(name)
                         .map_or(Err(ResolveError(name.clone())), |e| Ok(e.clone()))
                },
                None => {
                    Err(ResolveError(ns_name.clone()))
                }
            }
        } else {
            Err(DispatchError(expr.clone()))
        }
    }

    fn eval_def(&self, expr: &Expr) -> EvalResult {
        Ok(expr.clone())
    }

    fn eval_call(&self, expr: &Expr) -> EvalResult {
        Ok(expr.clone())
    }

    fn eval_let(&self, expr: &Expr) -> EvalResult {
        Ok(expr.clone())
    }

    fn expand(&self, expr: &Expr) -> EvalResult {
        if let Expr::List(ref l) = *expr {
            if l.len() > 0 {
                if let Expr::Symbol { ref name, .. } = l[0] {
                    match &name[..] {
                        "def" => {
                            return self.expand_def(expr)
                        },
                        "fn" => {
                            return self.expand_fn(expr)
                        },
                        "macro" => {
                            return self.expand_macro(expr)
                        },
                        "quote" => {
                            return self.expand_quote(expr)
                        },
                        "unquote" => {
                            return self.expand_unquote(expr)
                        },
                        "unquote-splicing" => {
                            return self.expand_unquote_splicing(expr)
                        },
                        "let" => {
                            return self.expand_let(expr)
                        },
                        _ => {
                            return self.expand_call(expr)
                        }
                    }
                } else {
                    return Err(DispatchError(l[0].clone()))
                }
            }
        }
        Ok(expr.clone())
    }

    fn expand_def(&self, expr: &Expr) -> EvalResult {
        Ok(expr.clone())
    }

    fn expand_fn(&self, expr: &Expr) -> EvalResult {
        Ok(expr.clone())
    }

    fn expand_macro(&self, expr: &Expr) -> EvalResult {
        Ok(expr.clone())
    }

    fn expand_quote(&self, expr: &Expr) -> EvalResult {
        Ok(expr.clone())
    }

    fn expand_unquote(&self, expr: &Expr) -> EvalResult {
        Ok(expr.clone())
    }

    fn expand_unquote_splicing(&self, expr: &Expr) -> EvalResult {
        Ok(expr.clone())
    }

    fn expand_let(&self, expr: &Expr) -> EvalResult {
        Ok(expr.clone())
    }

    fn expand_call(&self, expr: &Expr) -> EvalResult {
        Ok(expr.clone())
    }

}
