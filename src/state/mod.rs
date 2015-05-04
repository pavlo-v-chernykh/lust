mod error;
#[cfg(test)]
mod tests;

use std::collections::HashMap;
use ast::Expr;
use self::error::EvalError::*;
use self::error::EvalError;

pub type EvalResult = Result<Expr, EvalError>;

#[derive(Debug)]
struct Namespace {
    mappings: HashMap<String, Expr>,
}

impl Namespace {
    fn new() -> Namespace {
        Namespace { mappings: HashMap::new(), }
    }

    fn get(&self, name: &String) -> Option<&Expr> {
        self.mappings.get(name)
    }

    fn insert(&mut self, name: String, expr: Expr) -> Option<Expr> {
        self.mappings.insert(name, expr)
    }
}

#[derive(Debug)]
pub struct State<'s> {
    current: String,
    namespaces: HashMap<String, Namespace>,
    parent: Option<&'s State<'s>>,
    id: usize,
}

impl<'s> State<'s> {
    pub fn new(current: String) -> State<'s> {
        let mut current_ns = Namespace::new();
        current_ns.insert("nil".to_string(), e_list![]);
        current_ns.insert("true".to_string(), e_bool!(true));
        current_ns.insert("false".to_string(), e_bool!(false));

        let mut namespaces = HashMap::new();
        namespaces.insert(current.clone(), current_ns);

        State {
            current: current,
            namespaces: namespaces,
            parent: None,
            id: 0,
        }
    }

    pub fn eval(&mut self, expr: &Expr) -> EvalResult {
        match try!(self.expand(expr)) {
            ref symbol_expr @ Expr::Symbol { .. } => {
                self.eval_symbol(symbol_expr)
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

    fn new_chained(parent: &'s State<'s>) -> State<'s> {
        let mut state = State::new(format!("{}_chained", parent.current));
        state.parent = Some(parent);
        state
    }

    fn insert(&mut self, name: String, expr: Expr) -> Option<Expr> {
        self.namespaces
            .get_mut(&self.current)
            .and_then(|scope| scope.insert(name, expr))
    }

    fn get(&self, ns: Option<&String>, name: &String) -> Option<&Expr> {
        let mut state = self;
        loop {
            let v = state.namespaces
                         .get(ns.unwrap_or(&state.current))
                         .and_then(|scope| scope.get(name));
            if v.is_none() && state.parent.is_some() {
                state = state.parent.unwrap();
            } else {
                return v
            }
        }
    }

    fn get_current(&self) -> &String {
        &self.current
    }

    fn set_current(&mut self, current: String) -> String {
        if !self.namespaces.contains_key(&current) {
            self.namespaces.insert(current.clone(), Namespace::new());
        }
        let old_current = self.current.clone();
        self.current = current;
        old_current
    }

    fn next_id(&mut self) -> usize {
        let next = self.id;
        self.id += 1;
        next
    }

    fn eval_symbol(&mut self, expr: &Expr) -> EvalResult {
        if let Expr::Symbol { ref ns, ref name, .. } = *expr {
            self.get(ns.as_ref(), name)
                .map(|e| Ok(e.clone()))
                .unwrap_or(Err(ResolveError(name.clone())))
        } else {
            Err(DispatchError(expr.clone()))
        }
    }

    fn eval_quoted(&mut self, expr: &Expr) -> EvalResult {
        match *expr {
            Expr::Symbol { .. } => {
                Ok(expr.clone())
            },
            Expr::List(ref l) => {
                let mut v = vec![];
                for e in l {
                    if e.is_call_of("unquote-splicing") {
                        if let Expr::List(ref l) = try!(self.eval(&e)) {
                            for e in l {
                                v.push(e.clone())
                            }
                        } else {
                            return Err(IncorrectTypeOfArgumentError(e.clone()))
                        }
                    } else {
                        v.push(try!(self.eval_quoted(&e)))
                    }
                }
                Ok(Expr::List(v))
            },
            _ => {
                self.eval(expr)
            },
        }
    }

    fn eval_def(&mut self, expr: &Expr) -> EvalResult {
        if let Expr::Def { ref sym, ref expr } = *expr {
            let e = try!(self.eval(expr));
            self.insert(sym.clone(), e.clone());
            Ok(e)
        } else {
            Err(DispatchError(expr.clone()))
        }
    }

    fn eval_let(&mut self, expr: &Expr) -> EvalResult {
        if let Expr::Let { ref bindings, ref body } = *expr {
            let ref mut let_state = State::new_chained(self);
            for c in bindings.chunks(2) {
                if let (Some(&Expr::Symbol { ref name, .. }), Some(be)) = (c.first(), c.last()) {
                    let evaled_be = try!(let_state.eval(&be));
                    let_state.insert(name.clone(), evaled_be);
                }
            }
            let mut result = e_list![];
            for e in body {
                result = try!(let_state.eval(&e));
            }
            Ok(result)
        } else {
            Err(DispatchError(expr.clone()))
        }
    }

    fn eval_call(&mut self, expr: &Expr) -> EvalResult {
        if let Expr::Call { ref name, .. } = *expr {
            match &name[..] {
                "+" => {
                    self.eval_call_builtin_plus(expr)
                },
                "-" => {
                    self.eval_call_builtin_minus(expr)
                },
                "*" => {
                    self.eval_call_builtin_mul(expr)
                },
                "/" => {
                    self.eval_call_builtin_div(expr)
                },
                "<" => {
                    self.eval_call_builtin_lt(expr)
                },
                ">" => {
                    self.eval_call_builtin_gt(expr)
                },
                "=" => {
                    self.eval_call_builtin_eq(expr)
                },
                "if" => {
                    self.eval_call_builtin_if(expr)
                },
                "quote" => {
                    self.eval_call_builtin_quote(expr)
                },
                "syntax-quote" => {
                    self.eval_call_builtin_syntax_quote(expr)
                },
                "unquote" => {
                    self.eval_call_builtin_unquote(expr)
                },
                "unquote-splicing" => {
                    self.eval_call_builtin_unquote_splicing(expr)
                },
                "eval" => {
                    self.eval_call_builtin_eval(expr)
                },
                "gensym" => {
                    self.eval_call_builtin_gensym(expr)
                },
                "in-ns" => {
                    self.eval_call_builtin_in_ns(expr)
                },
                _ => {
                    self.eval_call_custom(expr)
                },
            }
        } else {
            Err(DispatchError(expr.clone()))
        }
    }

    fn eval_call_builtin_plus(&mut self, expr: &Expr) -> EvalResult {
        if let Expr::Call { ref args, .. } = *expr {
            let mut result = 0_f64;
            for a in args {
                if let Expr::Number(n) = try!(self.eval(&a)) {
                    result += n;
                } else {
                    return Err(IncorrectTypeOfArgumentError(a.clone()))
                }
            }
            Ok(Expr::Number(result))
        } else {
            Err(DispatchError(expr.clone()))
        }
    }

    fn eval_call_builtin_minus(&mut self, expr: &Expr) -> EvalResult {
        if let Expr::Call { ref args, .. } = *expr {
            if args.len() >= 1 {
                if let Expr::Number(n) = try!(self.eval(&args[0])) {
                    let mut result = if args.len() == 1 { -n } else { n };
                    for a in &args[1..] {
                        if let Expr::Number(n) = try!(self.eval(&a)) {
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
                Err(IncorrectNumberOfArgumentsError(expr.clone()))
            }
        } else {
            Err(DispatchError(expr.clone()))
        }
    }

    fn eval_call_builtin_mul(&mut self, expr: &Expr) -> EvalResult {
        if let Expr::Call { ref args, .. } = *expr {
            let mut result = 1_f64;
            for a in args {
                if let Expr::Number(n) = try!(self.eval(&a)) {
                    result *= n
                } else {
                    return Err(IncorrectTypeOfArgumentError(a.clone()))
                }
            }
            Ok(Expr::Number(result))
        } else {
            Err(DispatchError(expr.clone()))
        }
    }

    fn eval_call_builtin_div(&mut self, expr: &Expr) -> EvalResult {
        if let Expr::Call { ref args, .. } = *expr {
            if args.len() >= 1 {
                if let Expr::Number(n) = try!(self.eval(&args[0])) {
                    let mut result = if args.len() == 1 { 1. / n } else { n };
                    for a in &args[1..] {
                        if let Expr::Number(n) = try!(self.eval(&a)) {
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
                Err(IncorrectNumberOfArgumentsError(expr.clone()))
            }
        } else {
            Err(DispatchError(expr.clone()))
        }
    }

    fn eval_call_builtin_lt(&mut self, expr: &Expr) -> EvalResult {
        if let Expr::Call { ref args, .. } = *expr {
            if args.len() >= 1 {
                if let Expr::Number(n) = try!(self.eval(&args[0])) {
                    let mut temp = n;
                    for a in &args[1..] {
                        if let Expr::Number(n) = try!(self.eval(&a)) {
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
                Err(IncorrectNumberOfArgumentsError(expr.clone()))
            }
        } else {
            Err(DispatchError(expr.clone()))
        }
    }

    fn eval_call_builtin_gt(&mut self, expr: &Expr) -> EvalResult {
        if let Expr::Call { ref args, .. } = *expr {
            if args.len() >= 1 {
                if let Expr::Number(n) = try!(self.eval(&args[0])) {
                    let mut temp = n;
                    for a in &args[1..] {
                        if let Expr::Number(n) = try!(self.eval(&a)) {
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
                Err(IncorrectNumberOfArgumentsError(expr.clone()))
            }
        } else {
            Err(DispatchError(expr.clone()))
        }
    }

    fn eval_call_builtin_eq(&mut self, expr: &Expr) -> EvalResult {
        if let Expr::Call { ref args, .. } = *expr {
            if args.len() >= 1 {
                if let Expr::Number(n) = try!(self.eval(&args[0])) {
                    let mut temp = n;
                    for a in &args[1..] {
                        if let Expr::Number(n) = try!(self.eval(&a)) {
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
                Err(IncorrectNumberOfArgumentsError(expr.clone()))
            }
        } else {
            Err(DispatchError(expr.clone()))
        }
    }

    fn eval_call_builtin_if(&mut self, expr: &Expr) -> EvalResult {
        if let Expr::Call { ref args, .. } = *expr {
            if args.len() == 3 {
                if try!(self.eval(&args[0])).as_bool() {
                    self.eval(&args[1])
                } else {
                    self.eval(&args[2])
                }
            } else {
                Err(IncorrectNumberOfArgumentsError(expr.clone()))
            }
        } else {
            Err(DispatchError(expr.clone()))
        }
    }

    fn eval_call_builtin_quote(&mut self, expr: &Expr) -> EvalResult {
        if let Expr::Call { ref args, .. } = *expr {
            if args.len() == 1 {
                self.eval_quoted(&args[0])
            } else {
                Err(IncorrectNumberOfArgumentsError(expr.clone()))
            }
        } else {
            Err(DispatchError(expr.clone()))
        }
    }

    fn eval_call_builtin_syntax_quote(&mut self, expr: &Expr) -> EvalResult {
        if let Expr::Call { ref args, .. } = *expr {
            if args.len() == 1 {
                self.eval_quoted(&args[0])
            } else {
                Err(IncorrectNumberOfArgumentsError(expr.clone()))
            }
        } else {
            Err(DispatchError(expr.clone()))
        }
    }

    fn eval_call_builtin_unquote(&mut self, expr: &Expr) -> EvalResult {
        if let Expr::Call { ref args, .. } = *expr {
            if args.len() == 1 {
                self.eval(&args[0])
            } else {
                Err(IncorrectNumberOfArgumentsError(expr.clone()))
            }
        } else {
            Err(DispatchError(expr.clone()))
        }
    }

    fn eval_call_builtin_unquote_splicing(&mut self, expr: &Expr) -> EvalResult {
        if let Expr::Call { ref args, .. } = *expr {
            if args.len() == 1 {
                self.eval(&args[0])
            } else {
                Err(IncorrectNumberOfArgumentsError(expr.clone()))
            }
        } else {
            Err(DispatchError(expr.clone()))
        }
    }

    fn eval_call_builtin_eval(&mut self, expr: &Expr) -> EvalResult {
        if let Expr::Call { ref args, .. } = *expr {
            if args.len() == 1 {
                self.eval(&args[0]).and_then(|e| self.eval(&e))
            } else {
                Err(IncorrectNumberOfArgumentsError(expr.clone()))
            }
        } else {
            Err(DispatchError(expr.clone()))
        }
    }

    fn eval_call_builtin_gensym(&mut self, expr: &Expr) -> EvalResult {
        if let Expr::Call { ref args, .. } = *expr {
            if args.len() == 1 {
                if let Expr::String(ref s) = args[0] {
                    Ok(e_symbol![format!("{}{}", s, self.next_id())])
                } else {
                    Err(IncorrectTypeOfArgumentError(args[0].clone()))
                }
            } else {
                Err(IncorrectNumberOfArgumentsError(expr.clone()))
            }
        } else {
            Err(DispatchError(expr.clone()))
        }
    }

    fn eval_call_builtin_in_ns(&mut self, expr: &Expr) -> EvalResult {
        if let Expr::Call { ref args, .. } = *expr {
            if args.len() == 1 {
                if let Expr::Symbol { ref name, .. } = try!(self.eval(&args[0])) {
                    let old_current = self.get_current().clone();
                    self.set_current(name.clone());
                    Ok(e_symbol![old_current])
                } else {
                    Err(IncorrectTypeOfArgumentError(args[0].clone()))
                }
            } else {
                Err(IncorrectNumberOfArgumentsError(expr.clone()))
            }
        } else {
            Err(DispatchError(expr.clone()))
        }
    }

    fn eval_call_custom(&mut self, expr: &Expr) -> EvalResult {
        if let Expr::Call { ref ns, ref name, ref args, .. } = *expr {
            let func = try!(self.get(ns.as_ref(), name)
                                .map(|e| Ok(e.clone()))
                                .unwrap_or_else(|| Err(ResolveError(name.clone()))));
            match func {
                Expr::Fn { ref params, ref body } => {
                    if args.len() != params.len() {
                        return Err(IncorrectNumberOfArgumentsError(expr.clone()))
                    }

                    let mut e_args = vec![];
                    for a in args {
                        e_args.push(try!(self.eval(a)))
                    }

                    let ref mut fn_state = State::new_chained(self);
                    for (p, a) in params.iter().zip(e_args.iter()) {
                        if let &Expr::Symbol { ref name, .. } = p {
                            fn_state.insert(name.clone(), a.clone());
                        } else {
                            return Err(IncorrectTypeOfArgumentError(p.clone()))
                        }
                    }

                    let mut result = e_list![];
                    for e in body {
                        result = try!(fn_state.eval(e));
                    }

                    Ok(result)

                },
                Expr::Macro { ref params, ref body } => {
                    if args.len() != params.len() {
                        return Err(IncorrectNumberOfArgumentsError(expr.clone()))
                    }

                    let ref mut macro_state = State::new_chained(self);
                    for (p, a) in params.iter().zip(args.iter()) {
                        if let Expr::Symbol { ref name, .. } = *p {
                            macro_state.insert(name.clone(), a.clone());
                        } else {
                            return Err(IncorrectTypeOfArgumentError(p.clone()))
                        }
                    }

                    let mut result = e_list![];
                    for e in body {
                        result = try!(macro_state.eval(&e));
                    }

                    Ok(try!(macro_state.expand(&result)))
                },
                _ => {
                    Err(IncorrectTypeOfArgumentError(expr.clone()))
                }
            }
        } else {
            Err(DispatchError(expr.clone()))
        }
    }

    fn expand(&mut self, expr: &Expr) -> EvalResult {
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
                        "syntax-quote" => {
                            return self.expand_syntax_quote(expr)
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

    fn expand_quoted(&mut self, expr: &Expr) -> EvalResult {
        match *expr {
            Expr::List(ref l) if l.len() > 0 => {
                if l[0].is_symbol("unquote") || l[0].is_symbol("unquote-splicing") {
                    self.expand(expr)
                } else {
                    let mut v = vec![];
                    for i in l {
                        v.push(try!(self.expand_quoted(i)));
                    }
                    Ok(Expr::List(v))
                }
            },
            _ => {
                self.expand(expr)
            }
        }
    }

    fn expand_syntax_quoted(&mut self, expr: &Expr) -> EvalResult {
        match *expr {
            Expr::Symbol { ns: None, ref name } => {
                Ok(e_symbol![self.get_current(), name])
            },
            Expr::List(ref l) if l.len() > 0 => {
                if l[0].is_symbol("unquote") || l[0].is_symbol("unquote-splicing") {
                    self.expand(expr)
                } else {
                    let mut v = vec![];
                    for i in l {
                        v.push(try!(self.expand_syntax_quoted(i)));
                    }
                    Ok(Expr::List(v))
                }
            },
            _ => {
                self.expand(expr)
            }
        }
    }

    fn expand_def(&mut self, expr: &Expr) -> EvalResult {
        if let Expr::List(ref l) = *expr {
            if l.len() == 3 {
                if let Expr::Symbol { ref name, .. } = l[1] {
                    Ok(Expr::Def {
                        sym: name.clone(),
                        expr: Box::new(try!(self.expand(&l[2]))),
                    })
                } else {
                    Err(IncorrectTypeOfArgumentError(l[1].clone()))
                }
            } else {
                Err(IncorrectNumberOfArgumentsError(expr.clone()))
            }
        } else {
            Err(DispatchError(expr.clone()))
        }
    }

    fn expand_fn(&mut self, expr: &Expr) -> EvalResult {
        if let Expr::List(ref l) = *expr {
            if l.len() >= 3 {
                if let Expr::Vec(ref params) = l[1] {
                    let mut fn_params = vec![];
                    for p in params {
                        fn_params.push(try!(self.expand(p)))
                    }
                    let mut fn_body = vec![];
                    for be in &l[2..] {
                        fn_body.push(try!(self.expand(be)))
                    }
                    Ok(Expr::Fn {
                        params: fn_params,
                        body: fn_body,
                    })
                } else {
                    Err(IncorrectTypeOfArgumentError(l[1].clone()))
                }
            } else {
                Err(IncorrectNumberOfArgumentsError(expr.clone()))
            }
        } else {
            Err(DispatchError(expr.clone()))
        }
    }

    fn expand_macro(&mut self, expr: &Expr) -> EvalResult {
        if let Expr::List(ref l) = *expr {
            if l.len() >= 3 {
                if let Expr::Vec(ref params) = l[1] {
                    let mut macro_params = vec![];
                    for p in params {
                        macro_params.push(try!(self.expand(p)))
                    }
                    let mut macro_body = vec![];
                    for be in &l[2..] {
                        macro_body.push(try!(self.expand(be)))
                    }
                    Ok(Expr::Macro {
                        params: macro_params,
                        body: macro_body,
                    })
                } else {
                    Err(IncorrectTypeOfArgumentError(l[1].clone()))
                }
            } else {
                Err(IncorrectNumberOfArgumentsError(expr.clone()))
            }
        } else {
            Err(DispatchError(expr.clone()))
        }
    }

    fn expand_quote(&mut self, expr: &Expr) -> EvalResult {
        if let Expr::List(ref l) = *expr {
            if l.len() == 2 {
                Ok(e_call!["quote", try!(self.expand_quoted(&l[1]))])
            } else {
                Err(IncorrectNumberOfArgumentsError(expr.clone()))
            }
        } else {
            Err(DispatchError(expr.clone()))
        }
    }

    fn expand_syntax_quote(&mut self, expr: &Expr) -> EvalResult {
        if let Expr::List(ref l) = *expr {
            if l.len() == 2 {
                Ok(e_call!["syntax-quote", try!(self.expand_syntax_quoted(&l[1]))])
            } else {
                Err(IncorrectNumberOfArgumentsError(expr.clone()))
            }
        } else {
            Err(DispatchError(expr.clone()))
        }
    }

    fn expand_unquote(&mut self, expr: &Expr) -> EvalResult {
        if let Expr::List(ref l) = *expr {
            if l.len() == 2 {
                Ok(e_call!["unquote", try!(self.expand(&l[1]))])
            } else {
                Err(IncorrectNumberOfArgumentsError(expr.clone()))
            }
        } else {
            Err(DispatchError(expr.clone()))
        }
    }

    fn expand_unquote_splicing(&mut self, expr: &Expr) -> EvalResult {
        if let Expr::List(ref l) = *expr {
            if l.len() == 2 {
                Ok(e_call!["unquote-splicing", try!(self.expand(&l[1]))])
            } else {
                Err(IncorrectNumberOfArgumentsError(expr.clone()))
            }
        } else {
            Err(DispatchError(expr.clone()))
        }
    }

    fn expand_let(&mut self, expr: &Expr) -> EvalResult {
        if let Expr::List(ref l) = *expr {
            if l.len() >= 3 {
                if let Expr::Vec(ref v) = l[1] {
                    if v.len() % 2 == 0 {
                        let mut let_bindings = vec![];
                        for c in v.chunks(2) {
                            if let Some(s @ &Expr::Symbol {.. }) = c.first() {
                                let_bindings.push(s.clone())
                            } else {
                                return Err(IncorrectTypeOfArgumentError(expr.clone()))
                            }
                            if let Some(ref e) = c.last() {
                                let_bindings.push(try!(self.expand(e)))
                            } else {
                                return Err(IncorrectTypeOfArgumentError(expr.clone()))
                            }
                        }
                        let mut let_body = vec![];
                        for be in &l[2..] {
                            let_body.push(try!(self.expand(be)))
                        }
                        Ok(Expr::Let {
                            bindings: let_bindings,
                            body: let_body,
                        })
                    } else {
                        Err(IncorrectNumberOfArgumentsError(expr.clone()))
                    }
                } else {
                    Err(IncorrectTypeOfArgumentError(expr.clone()))
                }
            } else {
                Err(IncorrectNumberOfArgumentsError(expr.clone()))
            }
        } else {
            Err(DispatchError(expr.clone()))
        }
    }

    fn expand_call(&mut self, expr: &Expr) -> EvalResult {
        if let Expr::List(ref l) = *expr {
            if let Expr::Symbol { ref ns, ref name, .. } = l[0] {
                let mut args = vec![];
                for a in &l[1..] {
                    args.push(try!(self.expand(a)))
                }
                let call = Expr::Call {
                    ns: ns.clone(),
                    name: name.clone(),
                    args: args
                };
                if self.get(ns.as_ref(), name).map_or(false, |e| e.is_macro()) {
                    Ok(try!(self.eval(&call)))
                } else {
                    Ok(call)
                }
            } else {
                Err(IncorrectTypeOfArgumentError(l[0].clone()))
            }
        } else {
            Err(DispatchError(expr.clone()))
        }
    }
}
