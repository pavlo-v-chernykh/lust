mod error;
#[cfg(test)]
mod tests;

use std::fmt;
use scope::Scope;
use ast::error::EvalError::*;
pub use ast::error::EvalError;

pub type EvalResult = Result<Expr, EvalError>;

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Number(f64),
    Bool(bool),
    String(String),
    Symbol {
        ns: String,
        name: String,
    },
    Keyword(String),
    List(Vec<Expr>),
    Vec(Vec<Expr>),
    Let {
        bindings: Vec<Expr>,
        body: Vec<Expr>,
    },
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

static mut id: usize = 0;

fn next_id() -> usize {
    unsafe {
        let next = id;
        id += 1;
        next
    }
}

impl Expr {
    pub fn eval(&self, scope: &mut Scope) -> EvalResult {
        match try!(self.expand(scope)) {
            Expr::Symbol { ref name, .. } => {
                scope.get(name)
                     .map(|e| Ok(e.clone()))
                     .unwrap_or_else(|| Err(ResolveError(name.clone())))
            },
            def_expr @ Expr::Def { .. } => {
                def_expr.eval_def(scope)
            },
            call_expr @ Expr::Call { .. } => {
                call_expr.eval_call(scope)
            },
            let_expr @ Expr::Let { .. } => {
                let_expr.eval_let(scope)
            },
            other_expr => {
                Ok(other_expr)
            },
        }
    }

    fn eval_quoted(&self, scope: &mut Scope) -> EvalResult {
        match *self {
            Expr::Symbol { .. } => {
                Ok(self.clone())
            },
            Expr::List(ref l) => {
                let mut v = vec![];
                for e in l {
                    if e.is_call_of("unquote-splicing") {
                        if let Expr::List(ref l) = try!(e.eval(scope)) {
                            for e in l {
                                v.push(e.clone())
                            }
                        } else {
                            return Err(IncorrectTypeOfArgumentError(e.clone()))
                        }
                    } else {
                        v.push(try!(e.eval_quoted(scope)))
                    }
                }
                Ok(Expr::List(v))
            },
            _ => {
                self.eval(scope)
            },
        }
    }

    fn eval_def(&self, scope: &mut Scope) -> EvalResult {
        if let Expr::Def { ref sym, ref expr } = *self {
            let e = try!(expr.eval(scope));
            scope.insert(sym.clone(), e.clone());
            Ok(e)
        } else {
            Err(DispatchError(self.clone()))
        }
    }

    fn eval_let(&self, scope: &mut Scope) -> EvalResult {
        if let Expr::Let { ref bindings, ref body } = *self {
            let ref mut let_scope = Scope::new_chained(scope);
            for c in bindings.chunks(2) {
                if let (Some(&Expr::Symbol { ref name, .. }), Some(be)) = (c.first(), c.last()) {
                    let evaled_be = try!(be.eval(let_scope));
                    let_scope.insert(name.clone(), evaled_be);
                }
            }
            let mut result = e_list![];
            for e in body {
                result = try!(e.eval(let_scope));
            }
            Ok(result)
        } else {
            Err(DispatchError(self.clone()))
        }
    }

    fn eval_call(&self, scope: &mut Scope) -> EvalResult {
        if let Expr::Call { ref name, .. } = *self {
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
                "if" => {
                    self.eval_call_builtin_if(scope)
                },
                "quote" => {
                    self.eval_call_builtin_quote(scope)
                },
                "unquote" => {
                    self.eval_call_builtin_unquote(scope)
                },
                "unquote-splicing" => {
                    self.eval_call_builtin_unquote_splicing(scope)
                },
                "eval" => {
                    self.eval_call_builtin_eval(scope)
                },
                "gensym" => {
                    self.eval_call_builtin_gensym()
                },
                _ => {
                    self.eval_call_custom(scope)
                },
            }
        } else {
            Err(DispatchError(self.clone()))
        }
    }

    fn eval_call_builtin_plus(&self, scope: &mut Scope) -> EvalResult {
        if let Expr::Call { ref args, .. } = *self {
            let mut result = 0_f64;
            for a in args {
                if let Expr::Number(n) = try!(a.eval(scope)) {
                    result += n;
                } else {
                    return Err(IncorrectTypeOfArgumentError(a.clone()))
                }
            }
            Ok(Expr::Number(result))
        } else {
            Err(DispatchError(self.clone()))
        }
    }

    fn eval_call_builtin_minus(&self, scope: &mut Scope) -> EvalResult {
        if let Expr::Call { ref args, .. } = *self {
            if args.len() >= 1 {
                if let Expr::Number(n) = try!(args[0].eval(scope)) {
                    let mut result = if args.len() == 1 { -n } else { n };
                    for a in &args[1..] {
                        if let Expr::Number(n) = try!(a.eval(scope)) {
                            result -= n
                        } else {
                            return Err(IncorrectTypeOfArgumentError(a.clone()))
                        }
                    }
                    Ok(Expr::Number(result))
                } else {
                    Err(IncorrectTypeOfArgumentError(args[0].clone()))
                }
            } else {
                Err(IncorrectNumberOfArgumentsError(self.clone()))
            }
        } else {
            Err(DispatchError(self.clone()))
        }
    }

    fn eval_call_builtin_mul(&self, scope: &mut Scope) -> EvalResult {
        if let Expr::Call { ref args, .. } = *self {
            let mut result = 1_f64;
            for a in args {
                if let Expr::Number(n) = try!(a.eval(scope)) {
                    result *= n
                } else {
                    return Err(IncorrectTypeOfArgumentError(a.clone()))
                }
            }
            Ok(Expr::Number(result))
        } else {
            Err(DispatchError(self.clone()))
        }
    }

    fn eval_call_builtin_div(&self, scope: &mut Scope) -> EvalResult {
        if let Expr::Call { ref args, .. } = *self {
            if args.len() >= 1 {
                if let Expr::Number(n) = try!(args[0].eval(scope)) {
                    let mut result = if args.len() == 1 { 1. / n } else { n };
                    for a in &args[1..] {
                        if let Expr::Number(n) = try!(a.eval(scope)) {
                            result /= n
                        } else {
                            return Err(IncorrectTypeOfArgumentError(a.clone()))
                        }
                    }
                    Ok(Expr::Number(result))
                } else {
                    Err(IncorrectTypeOfArgumentError(args[0].clone()))
                }
            } else {
                Err(IncorrectNumberOfArgumentsError(self.clone()))
            }
        } else {
            Err(DispatchError(self.clone()))
        }
    }

    fn eval_call_builtin_lt(&self, scope: &mut Scope) -> EvalResult {
        if let Expr::Call { ref args, .. } = *self {
            if args.len() >= 1 {
                if let Expr::Number(n) = try!(args[0].eval(scope)) {
                    let mut temp = n;
                    for a in &args[1..] {
                        if let Expr::Number(n) = try!(a.eval(scope)) {
                            if temp < n {
                                temp = n
                            } else {
                                return Ok(Expr::Bool(false))
                            }
                        } else {
                            return Err(IncorrectTypeOfArgumentError(a.clone()))
                        }
                    }
                    Ok(Expr::Bool(true))
                } else {
                    Err(IncorrectTypeOfArgumentError(args[0].clone()))
                }
            } else {
                Err(IncorrectNumberOfArgumentsError(self.clone()))
            }
        } else {
            Err(DispatchError(self.clone()))
        }
    }

    fn eval_call_builtin_gt(&self, scope: &mut Scope) -> EvalResult {
        if let Expr::Call { ref args, .. } = *self {
            if args.len() >= 1 {
                if let Expr::Number(n) = try!(args[0].eval(scope)) {
                    let mut temp = n;
                    for a in &args[1..] {
                        if let Expr::Number(n) = try!(a.eval(scope)) {
                            if temp > n {
                                temp = n
                            } else {
                                return Ok(Expr::Bool(false))
                            }
                        } else {
                            return Err(IncorrectTypeOfArgumentError(a.clone()))
                        }
                    }
                    Ok(Expr::Bool(true))
                } else {
                    Err(IncorrectTypeOfArgumentError(args[0].clone()))
                }
            } else {
                Err(IncorrectNumberOfArgumentsError(self.clone()))
            }
        } else {
            Err(DispatchError(self.clone()))
        }
    }

    fn eval_call_builtin_eq(&self, scope: &mut Scope) -> EvalResult {
        if let Expr::Call { ref args, .. } = *self {
            if args.len() >= 1 {
                if let Expr::Number(n) = try!(args[0].eval(scope)) {
                    let mut temp = n;
                    for a in &args[1..] {
                        if let Expr::Number(n) = try!(a.eval(scope)) {
                            if temp == n {
                                temp = n
                            } else {
                                return Ok(Expr::Bool(false))
                            }
                        } else {
                            return Err(IncorrectTypeOfArgumentError(a.clone()))
                        }
                    }
                    Ok(Expr::Bool(true))
                } else {
                    Err(IncorrectTypeOfArgumentError(args[0].clone()))
                }
            } else {
                Err(IncorrectNumberOfArgumentsError(self.clone()))
            }
        } else {
            Err(DispatchError(self.clone()))
        }
    }

    fn eval_call_builtin_if(&self, scope: &mut Scope) -> EvalResult {
        if let Expr::Call { ref args, .. } = *self {
            if args.len() == 3 {
                if try!(args[0].eval(scope)).as_bool() {
                    args[1].eval(scope)
                } else {
                    args[2].eval(scope)
                }
            } else {
                Err(IncorrectNumberOfArgumentsError(self.clone()))
            }
        } else {
            Err(DispatchError(self.clone()))
        }
    }

    fn eval_call_builtin_quote(&self, scope: &mut Scope) -> EvalResult {
        if let Expr::Call { ref args, .. } = *self {
            if args.len() == 1 {
                args[0].eval_quoted(scope)
            } else {
                Err(IncorrectNumberOfArgumentsError(self.clone()))
            }
        } else {
            Err(DispatchError(self.clone()))
        }
    }

    fn eval_call_builtin_unquote(&self, scope: &mut Scope) -> EvalResult {
        if let Expr::Call { ref args, .. } = *self {
            if args.len() == 1 {
                args[0].eval(scope)
            } else {
                Err(IncorrectNumberOfArgumentsError(self.clone()))
            }
        } else {
            Err(DispatchError(self.clone()))
        }
    }

    fn eval_call_builtin_unquote_splicing(&self, scope: &mut Scope) -> EvalResult {
        if let Expr::Call { ref args, .. } = *self {
            if args.len() == 1 {
                args[0].eval(scope)
            } else {
                Err(IncorrectNumberOfArgumentsError(self.clone()))
            }
        } else {
            Err(DispatchError(self.clone()))
        }
    }

    fn eval_call_builtin_eval(&self, scope: &mut Scope) -> EvalResult {
        if let Expr::Call { ref args, .. } = *self {
            if args.len() == 1 {
                args[0].eval(scope).and_then(|e| e.eval(scope))
            } else {
                Err(IncorrectNumberOfArgumentsError(self.clone()))
            }
        } else {
            Err(DispatchError(self.clone()))
        }
    }

    fn eval_call_builtin_gensym(&self) -> EvalResult {
        if let Expr::Call { ref args, .. } = *self {
            if args.len() == 1 {
                if let Expr::String(ref s) = args[0] {
                    Ok(e_symbol![format!("{}{}", s, next_id())])
                } else {
                    Err(IncorrectTypeOfArgumentError(args[0].clone()))
                }
            } else {
                Err(IncorrectNumberOfArgumentsError(self.clone()))
            }
        } else {
            Err(DispatchError(self.clone()))
        }
    }

    fn eval_call_custom(&self, scope: &mut Scope) -> EvalResult {
        if let Expr::Call { ref name, ref args } = *self {
            let func = try!(scope.get(name)
                                 .map(|e| Ok(e.clone()))
                                 .unwrap_or_else(|| Err(ResolveError(name.clone()))));
            match func {
                Expr::Fn { ref params, ref body } => {
                    if args.len() != params.len() {
                        return Err(IncorrectNumberOfArgumentsError(self.clone()))
                    }

                    let mut e_args = vec![];
                    for a in args {
                        e_args.push(try!(a.eval(scope)))
                    }

                    let ref mut fn_scope = Scope::new_chained(&scope);
                    for (p, a) in params.iter().zip(e_args.iter()) {
                        if let &Expr::Symbol { ref name, .. } = p {
                            fn_scope.insert(name.clone(), a.clone());
                        } else {
                            return Err(IncorrectTypeOfArgumentError(p.clone()))
                        }
                    }

                    let mut result = e_list![];
                    for e in body {
                        result = try!(e.eval(fn_scope));
                    }

                    Ok(result)

                },
                Expr::Macro { ref params, ref body } => {
                    if args.len() != params.len() {
                        return Err(IncorrectNumberOfArgumentsError(self.clone()))
                    }

                    let ref mut fn_scope = Scope::new_chained(&scope);
                    for (p, a) in params.iter().zip(args.iter()) {
                        if let Expr::Symbol { ref name, .. } = *p {
                            fn_scope.insert(name.clone(), a.clone());
                        } else {
                            return Err(IncorrectTypeOfArgumentError(p.clone()))
                        }
                    }

                    let mut result = e_list![];
                    for e in body {
                        result = try!(e.eval(fn_scope));
                    }

                    Ok(try!(result.expand(fn_scope)))
                },
                _ => {
                    Err(IncorrectTypeOfArgumentError(self.clone()))
                }
            }
        } else {
            Err(DispatchError(self.clone()))
        }
    }

    fn expand(&self, scope: &mut Scope) -> EvalResult {
        if let Expr::List(ref l) = *self {
            if l.len() > 0 {
                if let Expr::Symbol { ref name, .. } = l[0] {
                    match &name[..] {
                        "def" => {
                            return self.expand_def(scope)
                        },
                        "fn" => {
                            return self.expand_fn(scope)
                        },
                        "macro" => {
                            return self.expand_macro(scope)
                        },
                        "quote" => {
                            return self.expand_quote(scope)
                        },
                        "unquote" => {
                            return self.expand_unquote(scope)
                        },
                        "unquote-splicing" => {
                            return self.expand_unquote_splicing(scope)
                        },
                        "let" => {
                            return self.expand_let(scope)
                        },
                        _ => {
                            return self.expand_call(scope)
                        }
                    }
                } else {
                    return Err(DispatchError(l[0].clone()))
                }
            }
        }
        Ok(self.clone())
    }

    fn expand_quoted(&self, scope: &mut Scope) -> EvalResult {
        match *self {
            Expr::List(ref l) if l.len() > 0 => {
                if l[0].is_symbol("unquote") {
                    self.expand(scope)
                } else if l[0].is_symbol("unquote-splicing") {
                    self.expand(scope)
                } else {
                    let mut v = vec![];
                    for i in l {
                        v.push(try!(i.expand_quoted(scope)));
                    }
                    Ok(Expr::List(v))
                }
            },
            _ => {
                self.expand(scope)
            }
        }
    }

    fn expand_def(&self, scope: &mut Scope) -> EvalResult {
        if let Expr::List(ref l) = *self {
            if l.len() == 3 {
                if let Expr::Symbol { ref name, .. } = l[1] {
                    Ok(Expr::Def {
                        sym: name.clone(),
                        expr: Box::new(try!(l[2].expand(scope))),
                    })
                } else {
                    Err(IncorrectTypeOfArgumentError(l[1].clone()))
                }
            } else {
                Err(IncorrectNumberOfArgumentsError(self.clone()))
            }
        } else {
            Err(DispatchError(self.clone()))
        }
    }

    fn expand_fn(&self, scope: &mut Scope) -> EvalResult {
        if let Expr::List(ref l) = *self {
            if l.len() >= 3 {
                if let Expr::Vec(ref params) = l[1] {
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
                    Err(IncorrectTypeOfArgumentError(l[1].clone()))
                }
            } else {
                Err(IncorrectNumberOfArgumentsError(self.clone()))
            }
        } else {
            Err(DispatchError(self.clone()))
        }
    }

    fn expand_macro(&self, scope: &mut Scope) -> EvalResult {
        if let Expr::List(ref l) = *self {
            if l.len() >= 3 {
                if let Expr::Vec(ref params) = l[1] {
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
                    Err(IncorrectTypeOfArgumentError(l[1].clone()))
                }
            } else {
                Err(IncorrectNumberOfArgumentsError(self.clone()))
            }
        } else {
            Err(DispatchError(self.clone()))
        }
    }

    fn expand_quote(&self, scope: &mut Scope) -> EvalResult {
        if let Expr::List(ref l) = *self {
            if l.len() == 2 {
                Ok(e_call!["quote", try!(l[1].expand_quoted(scope))])
            } else {
                Err(IncorrectNumberOfArgumentsError(self.clone()))
            }
        } else {
            Err(DispatchError(self.clone()))
        }
    }

    fn expand_unquote(&self, scope: &mut Scope) -> EvalResult {
        if let Expr::List(ref l) = *self {
            if l.len() == 2 {
                Ok(e_call!["unquote", try!(l[1].expand(scope))])
            } else {
                Err(IncorrectNumberOfArgumentsError(self.clone()))
            }
        } else {
            Err(DispatchError(self.clone()))
        }
    }

    fn expand_unquote_splicing(&self, scope: &mut Scope) -> EvalResult {
        if let Expr::List(ref l) = *self {
            if l.len() == 2 {
                Ok(e_call!["unquote-splicing", try!(l[1].expand(scope))])
            } else {
                Err(IncorrectNumberOfArgumentsError(self.clone()))
            }
        } else {
            Err(DispatchError(self.clone()))
        }
    }

    fn expand_let(&self, scope: &mut Scope) -> EvalResult {
        if let Expr::List(ref l) = *self {
            if l.len() >= 3 {
                if let Expr::Vec(ref v) = l[1] {
                    if v.len() % 2 == 0 {
                        let mut let_bindings = vec![];
                        for c in v.chunks(2) {
                            if let Some(s @ &Expr::Symbol {.. }) = c.first() {
                                let_bindings.push(s.clone())
                            } else {
                                return Err(IncorrectTypeOfArgumentError(self.clone()))
                            }
                            if let Some(ref e) = c.last() {
                                let_bindings.push(try!(e.expand(scope)))
                            } else {
                                return Err(IncorrectTypeOfArgumentError(self.clone()))
                            }
                        }
                        let mut let_body = vec![];
                        for be in &l[2..] {
                            let_body.push(try!(be.expand(scope)))
                        }
                        Ok(Expr::Let {
                            bindings: let_bindings,
                            body: let_body,
                        })
                    } else {
                        Err(IncorrectNumberOfArgumentsError(self.clone()))
                    }
                } else {
                    Err(IncorrectTypeOfArgumentError(self.clone()))
                }
            } else {
                Err(IncorrectNumberOfArgumentsError(self.clone()))
            }
        } else {
            Err(DispatchError(self.clone()))
        }
    }

    fn expand_call(&self, scope: &mut Scope) -> EvalResult {
        if let Expr::List(ref l) = *self {
            if let Expr::Symbol { ref name, .. } = l[0] {
                let mut args = vec![];
                for a in &l[1..] {
                    args.push(try!(a.expand(scope)))
                }
                let call = Expr::Call {
                    name: name.clone(),
                    args: args
                };
                if scope.get(name).map_or(false, |e| e.is_macro()) {
                    Ok(try!(call.eval(scope)))
                } else {
                    Ok(call)
                }
            } else {
                Err(IncorrectTypeOfArgumentError(l[0].clone()))
            }
        } else {
            Err(DispatchError(self.clone()))
        }
    }

    fn as_bool(&self) -> bool {
        match *self {
            Expr::Bool(b) => {
                b
            }
            _ => {
                true
            },
        }
    }

    fn is_symbol(&self, sym: &str) -> bool {
        match *self {
            Expr::Symbol { ref name, .. } if &name[..] == sym => {
                true
            },
            _ => {
                false
            }
        }
    }

    fn is_macro(&self) -> bool {
        match *self {
            Expr::Macro { .. } => {
                true
            },
            _ => {
                false
            }
        }
    }

    fn is_call_of(&self, name: &str) -> bool {
        match *self {
            Expr::Call { name: ref n, .. } => {
                &n[..] == name
            },
            _ => {
                false
            }
        }
    }
}

fn format_vec(v: &[Expr]) -> String {
        let mut a = String::new();
        if !v.is_empty() {
            let last_idx = v.len() - 1;
            for (i, e) in v.iter().enumerate() {
                if i < last_idx {
                    a.push_str(&format!("{} ", e))
                } else {
                    a.push_str(&format!("{}", e))
                }
            }
        }
        a
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Expr::Number(n) => {
                write!(f, "{}", n)
            },
            Expr::Bool(b) => {
                write!(f, "{}", b)
            },
            Expr::Symbol { ref name, .. } => {
                write!(f, "{}", name)
            },
            Expr::Keyword(ref s) => {
                write!(f, "{}", s)
            },
            Expr::String(ref s) => {
                write!(f, r#""{}""#, s)
            },
            Expr::List(ref l) => {
                write!(f, "({})", format_vec(l))
            },
            Expr::Vec(ref v) => {
                write!(f, "[{}]", format_vec(v))
            },
            Expr::Def { ref sym, ref expr } => {
                write!(f, "(def {} {})", sym, expr)
            },
            Expr::Let { ref bindings, ref body } => {
                write!(f, "(let [{}] {})", format_vec(bindings), format_vec(body))
            },
            Expr::Fn { ref params, ref body } => {
                write!(f, "(fn [{}] {})", format_vec(params), format_vec(body))
            },
            Expr::Macro { ref params, ref body } => {
                write!(f, "(macro [{}] {})", format_vec(params), format_vec(body))
            },
            Expr::Call { ref name, ref args } => {
                let mut a = format!("({}", name);
                if args.is_empty() {
                    a.push_str(")")
                } else {
                    a.push_str(&format!(" {})", format_vec(args)))
                }
                write!(f, "{}", a)
            },
        }
    }
}
