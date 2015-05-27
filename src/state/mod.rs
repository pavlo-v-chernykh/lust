mod error;
#[cfg(test)]
mod tests;

use std::collections::HashMap;
use std::io::Read;
use std::path::Path;
use std::fs::{File, metadata};
use self::error::EvalError::*;
use self::error::EvalError;
use ast::Node;
use ast::nodes::Symbol;
use parser::Parser;

pub type EvalResult = Result<Node, EvalError>;

#[derive(Debug)]
pub struct State<'s> {
    current: String,
    state: HashMap<Symbol, Node>,
    parent: Option<&'s State<'s>>,
    id: usize,
}

impl<'s> State<'s> {
    pub fn new(current: String) -> State<'s> {
        let mut state = HashMap::new();
        state.insert(Symbol::new(Some(current.clone()), "nil".to_string()), n_list![]);
        state.insert(Symbol::new(Some(current.clone()), "true".to_string()), n_bool!(true));
        state.insert(Symbol::new(Some(current.clone()), "false".to_string()), n_bool!(false));

        State {
            current: current,
            state: state,
            parent: None,
            id: 0,
        }
    }

    pub fn eval(&mut self, node: &Node) -> EvalResult {
        match try!(self.expand(node)) {
            ref symbol_node @ Node::Symbol(..) => {
                self.eval_symbol(symbol_node)
            },
            ref def_node @ Node::Def(..) => {
                self.eval_def(def_node)
            },
            ref call_node @ Node::Call(..) => {
                self.eval_call(call_node)
            },
            ref let_node @ Node::Let(..) => {
                self.eval_let(let_node)
            },
            other_node => {
                Ok(other_node)
            },
        }
    }

    fn new_chained(parent: &'s State<'s>) -> State<'s> {
        let mut state = State::new(format!("{}_chained", parent.current));
        state.parent = Some(parent);
        state
    }

    fn insert(&mut self, symbol: Symbol, node: Node) -> Option<Node> {
        if symbol.ns().is_some() {
            self.state.insert(symbol, node)
        } else {
            self.state.insert(Symbol::new(Some(self.current.clone()), symbol.name().clone()), node)
        }
    }

    fn get(&self, symbol: &Symbol) -> Option<&Node> {
        let mut state = self;
        loop {
            let mut v = if symbol.ns().is_some() {
                state.state.get(symbol)
            } else {
                state.state.get(&Symbol::new(Some(state.current.clone()), symbol.name().clone()))
            };
            if let Some(&Node::Alias(ref a)) = v {
                v = state.state.get(&a.to_symbol());
            }
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
        let old_current = self.current.clone();
        self.current = current;
        old_current
    }

    fn next_id(&mut self) -> usize {
        let next = self.id;
        self.id += 1;
        next
    }

    fn eval_symbol(&mut self, node: &Node) -> EvalResult {
        if let Node::Symbol(ref s) = *node {
            self.get(s)
                .map(|e| Ok(e.clone()))
                .unwrap_or(Err(ResolveError(s.name().clone())))
        } else {
            Err(DispatchError(node.clone()))
        }
    }

    fn eval_quoted(&mut self, node: &Node) -> EvalResult {
        match *node {
            Node::Symbol(..) => {
                Ok(node.clone())
            },
            Node::List(ref l) => {
                let mut v = vec![];
                for e in l {
                    if e.is_call_of("unquote-splicing") {
                        if let Node::List(ref l) = try!(self.eval(&e)) {
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
                Ok(Node::List(v))
            },
            _ => {
                self.eval(node)
            },
        }
    }

    fn eval_def(&mut self, node: &Node) -> EvalResult {
        if let Node::Def(ref d) = *node {
            let e = try!(self.eval(d.expr()));
            self.insert(d.symbol().clone(), e.clone());
            Ok(e)
        } else {
            Err(DispatchError(node.clone()))
        }
    }

    fn eval_let(&mut self, node: &Node) -> EvalResult {
        if let Node::Let(ref l) = *node {
            let ref mut let_state = State::new_chained(self);
            for c in l.bindings().chunks(2) {
                if let (Some(&Node::Symbol(ref s)), Some(be)) = (c.first(), c.last()) {
                    let evaled_be = try!(let_state.eval(&be));
                    let_state.insert(s.clone(), evaled_be);
                }
            }
            let mut result = n_list![];
            for e in l.body() {
                result = try!(let_state.eval(&e));
            }
            Ok(result)
        } else {
            Err(DispatchError(node.clone()))
        }
    }

    fn eval_call(&mut self, node: &Node) -> EvalResult {
        if let Node::Call(ref c) = *node {
            match &c.symbol().name()[..] {
                "+" => {
                    self.eval_call_builtin_plus(node)
                },
                "-" => {
                    self.eval_call_builtin_minus(node)
                },
                "*" => {
                    self.eval_call_builtin_mul(node)
                },
                "/" => {
                    self.eval_call_builtin_div(node)
                },
                "<" => {
                    self.eval_call_builtin_lt(node)
                },
                ">" => {
                    self.eval_call_builtin_gt(node)
                },
                "=" => {
                    self.eval_call_builtin_eq(node)
                },
                "if" => {
                    self.eval_call_builtin_if(node)
                },
                "quote" => {
                    self.eval_call_builtin_quote(node)
                },
                "syntax-quote" => {
                    self.eval_call_builtin_syntax_quote(node)
                },
                "unquote" => {
                    self.eval_call_builtin_unquote(node)
                },
                "unquote-splicing" => {
                    self.eval_call_builtin_unquote_splicing(node)
                },
                "eval" => {
                    self.eval_call_builtin_eval(node)
                },
                "apply" => {
                    self.eval_call_builtin_apply(node)
                },
                "gensym" => {
                    self.eval_call_builtin_gensym(node)
                },
                "in-ns" => {
                    self.eval_call_builtin_in_ns(node)
                },
                "load" => {
                    self.eval_call_builtin_load(node)
                },
                "refer" => {
                    self.eval_call_builtin_refer(node)
                },
                _ => {
                    self.eval_call_custom(node)
                },
            }
        } else {
            Err(DispatchError(node.clone()))
        }
    }

    fn eval_call_builtin_plus(&mut self, node: &Node) -> EvalResult {
        if let Node::Call(ref c) = *node {
            let mut result = 0_f64;
            for a in c.args() {
                if let Node::Number(n) = try!(self.eval(&a)) {
                    result += n.value();
                } else {
                    return Err(IncorrectTypeOfArgumentError(a.clone()))
                }
            }
            Ok(n_number![result])
        } else {
            Err(DispatchError(node.clone()))
        }
    }

    fn eval_call_builtin_minus(&mut self, node: &Node) -> EvalResult {
        if let Node::Call(ref c) = *node {
            let args = c.args();
            if args.len() >= 1 {
                if let Node::Number(n) = try!(self.eval(&args[0])) {
                    let value = n.value();
                    let mut result = if args.len() == 1 { -value } else { value };
                    for a in &args[1..] {
                        if let Node::Number(n) = try!(self.eval(&a)) {
                            result -= n.value()
                        } else {
                            return Err(IncorrectTypeOfArgumentError(a.clone()))
                        }
                    }
                    Ok(n_number![result])
                } else {
                    Err(IncorrectTypeOfArgumentError(args[0].clone()))
                }
            } else {
                Err(IncorrectNumberOfArgumentsError(node.clone()))
            }
        } else {
            Err(DispatchError(node.clone()))
        }
    }

    fn eval_call_builtin_mul(&mut self, node: &Node) -> EvalResult {
        if let Node::Call(ref c) = *node {
            let args = c.args();
            let mut result = 1_f64;
            for a in args {
                if let Node::Number(n) = try!(self.eval(&a)) {
                    result *= n.value()
                } else {
                    return Err(IncorrectTypeOfArgumentError(a.clone()))
                }
            }
            Ok(n_number![result])
        } else {
            Err(DispatchError(node.clone()))
        }
    }

    fn eval_call_builtin_div(&mut self, node: &Node) -> EvalResult {
        if let Node::Call(ref c) = *node {
            let args = c.args();
            if args.len() >= 1 {
                if let Node::Number(n) = try!(self.eval(&args[0])) {
                    let value = n.value();
                    let mut result = if args.len() == 1 { 1. / value } else { value };
                    for a in &args[1..] {
                        if let Node::Number(n) = try!(self.eval(&a)) {
                            result /= n.value()
                        } else {
                            return Err(IncorrectTypeOfArgumentError(a.clone()))
                        }
                    }
                    Ok(n_number![result])
                } else {
                    Err(IncorrectTypeOfArgumentError(args[0].clone()))
                }
            } else {
                Err(IncorrectNumberOfArgumentsError(node.clone()))
            }
        } else {
            Err(DispatchError(node.clone()))
        }
    }

    fn eval_call_builtin_lt(&mut self, node: &Node) -> EvalResult {
        if let Node::Call(ref c) = *node {
            let args = c.args();
            if args.len() >= 1 {
                if let Node::Number(n) = try!(self.eval(&args[0])) {
                    let mut temp = n.value();
                    for a in &args[1..] {
                        if let Node::Number(n) = try!(self.eval(&a)) {
                            let value = n.value();
                            if temp < value {
                                temp = value
                            } else {
                                return Ok(n_bool![false])
                            }
                        } else {
                            return Err(IncorrectTypeOfArgumentError(a.clone()))
                        }
                    }
                    Ok(n_bool![true])
                } else {
                    Err(IncorrectTypeOfArgumentError(args[0].clone()))
                }
            } else {
                Err(IncorrectNumberOfArgumentsError(node.clone()))
            }
        } else {
            Err(DispatchError(node.clone()))
        }
    }

    fn eval_call_builtin_gt(&mut self, node: &Node) -> EvalResult {
        if let Node::Call(ref c) = *node {
            let args = c.args();
            if args.len() >= 1 {
                if let Node::Number(n) = try!(self.eval(&args[0])) {
                    let mut temp = n.value();
                    for a in &args[1..] {
                        if let Node::Number(n) = try!(self.eval(&a)) {
                            let value = n.value();
                            if temp > value {
                                temp = value
                            } else {
                                return Ok(n_bool![false])
                            }
                        } else {
                            return Err(IncorrectTypeOfArgumentError(a.clone()))
                        }
                    }
                    Ok(n_bool![true])
                } else {
                    Err(IncorrectTypeOfArgumentError(args[0].clone()))
                }
            } else {
                Err(IncorrectNumberOfArgumentsError(node.clone()))
            }
        } else {
            Err(DispatchError(node.clone()))
        }
    }

    fn eval_call_builtin_eq(&mut self, node: &Node) -> EvalResult {
        if let Node::Call(ref c) = *node {
            let args = c.args();
            if args.len() >= 1 {
                if let Node::Number(n) = try!(self.eval(&args[0])) {
                    let mut temp = n.value();
                    for a in &args[1..] {
                        if let Node::Number(n) = try!(self.eval(&a)) {
                            let value = n.value();
                            if temp == value {
                                temp = value
                            } else {
                                return Ok(n_bool![false])
                            }
                        } else {
                            return Err(IncorrectTypeOfArgumentError(a.clone()))
                        }
                    }
                    Ok(n_bool![true])
                } else {
                    Err(IncorrectTypeOfArgumentError(args[0].clone()))
                }
            } else {
                Err(IncorrectNumberOfArgumentsError(node.clone()))
            }
        } else {
            Err(DispatchError(node.clone()))
        }
    }

    fn eval_call_builtin_if(&mut self, node: &Node) -> EvalResult {
        if let Node::Call(ref c) = *node {
            let args = c.args();
            if args.len() == 3 {
                if try!(self.eval(&args[0])).as_bool() {
                    self.eval(&args[1])
                } else {
                    self.eval(&args[2])
                }
            } else {
                Err(IncorrectNumberOfArgumentsError(node.clone()))
            }
        } else {
            Err(DispatchError(node.clone()))
        }
    }

    fn eval_call_builtin_quote(&mut self, node: &Node) -> EvalResult {
        if let Node::Call(ref c) = *node {
            let args = c.args();
            if args.len() == 1 {
                self.eval_quoted(&args[0])
            } else {
                Err(IncorrectNumberOfArgumentsError(node.clone()))
            }
        } else {
            Err(DispatchError(node.clone()))
        }
    }

    fn eval_call_builtin_syntax_quote(&mut self, node: &Node) -> EvalResult {
        if let Node::Call(ref c) = *node {
            let args = c.args();
            if args.len() == 1 {
                self.eval_quoted(&args[0])
            } else {
                Err(IncorrectNumberOfArgumentsError(node.clone()))
            }
        } else {
            Err(DispatchError(node.clone()))
        }
    }

    fn eval_call_builtin_unquote(&mut self, node: &Node) -> EvalResult {
        if let Node::Call(ref c) = *node {
            let args = c.args();
            if args.len() == 1 {
                self.eval(&args[0])
            } else {
                Err(IncorrectNumberOfArgumentsError(node.clone()))
            }
        } else {
            Err(DispatchError(node.clone()))
        }
    }

    fn eval_call_builtin_unquote_splicing(&mut self, node: &Node) -> EvalResult {
        if let Node::Call(ref c) = *node {
            let args = c.args();
            if args.len() == 1 {
                self.eval(&args[0])
            } else {
                Err(IncorrectNumberOfArgumentsError(node.clone()))
            }
        } else {
            Err(DispatchError(node.clone()))
        }
    }

    fn eval_call_builtin_eval(&mut self, node: &Node) -> EvalResult {
        if let Node::Call(ref c) = *node {
            let args = c.args();
            if args.len() == 1 {
                self.eval(&args[0]).and_then(|e| self.eval(&e))
            } else {
                Err(IncorrectNumberOfArgumentsError(node.clone()))
            }
        } else {
            Err(DispatchError(node.clone()))
        }
    }

    fn eval_call_builtin_apply(&mut self, node: &Node) -> EvalResult {
        if let Node::Call(ref c) = *node {
            let args = c.args();
            if args.len() == 2 {
                if let Node::Symbol(ref s) = args[0] {
                    if let Node::Vec(v) = try!(self.eval(&args[1])) {
                        self.eval(&n_call![s.ns().map(|ns| ns.clone()), s.name().clone(), v])
                    } else {
                        Err(IncorrectTypeOfArgumentError(args[1].clone()))
                    }
                } else {
                    Err(IncorrectTypeOfArgumentError(args[0].clone()))
                }
            } else {
                Err(IncorrectNumberOfArgumentsError(node.clone()))
            }
        } else {
            Err(DispatchError(node.clone()))
        }
    }

    fn eval_call_builtin_gensym(&mut self, node: &Node) -> EvalResult {
        if let Node::Call(ref c) = *node {
            let args = c.args();
            if args.len() == 1 {
                if let Node::String(ref s) = args[0] {
                    Ok(n_symbol![format!("{}{}", s, self.next_id())])
                } else {
                    Err(IncorrectTypeOfArgumentError(args[0].clone()))
                }
            } else {
                Err(IncorrectNumberOfArgumentsError(node.clone()))
            }
        } else {
            Err(DispatchError(node.clone()))
        }
    }

    fn eval_call_builtin_in_ns(&mut self, node: &Node) -> EvalResult {
        if let Node::Call(ref c) = *node {
            let args = c.args();
            if args.len() == 1 {
                if let Node::Symbol(ref s) = try!(self.eval(&args[0])) {
                    let old_current = self.get_current().clone();
                    self.set_current(s.name().clone());
                    Ok(n_symbol![old_current])
                } else {
                    Err(IncorrectTypeOfArgumentError(args[0].clone()))
                }
            } else {
                Err(IncorrectNumberOfArgumentsError(node.clone()))
            }
        } else {
            Err(DispatchError(node.clone()))
        }
    }

    fn eval_call_builtin_load(&mut self, node: &Node) -> EvalResult {
        if let Node::Call(ref c) = *node {
            let args = c.args();
            if args.len() == 1 {
                if let Node::String(ref s) = try!(self.eval(&args[0])) {
                    let path = Path::new(s.value());
                    let md = metadata(path);
                    if is_file_exists!(md) {
                        if is_file!(md) {
                            let mut file = try!(File::open(&path));
                            let ref mut buf = String::new();
                            try!(file.read_to_string(buf));
                            let mut last_evaled = None;
                            for parsed_expr in Parser::new(buf.chars()) {
                                last_evaled = Some(try!(self.eval(&try!(parsed_expr))));
                            }
                            last_evaled.map_or(Ok(n_list![]), |e| Ok(e))
                        } else {
                            Err(IncorrectTypeOfArgumentError(Node::String(s.clone())))
                        }
                    } else {
                        Err(IncorrectTypeOfArgumentError(Node::String(s.clone())))
                    }
                } else {
                    Err(IncorrectTypeOfArgumentError(args[0].clone()))
                }
            } else {
                Err(IncorrectNumberOfArgumentsError(node.clone()))
            }
        } else {
            Err(DispatchError(node.clone()))
        }
    }

    fn eval_call_builtin_refer(&mut self, node: &Node) -> EvalResult {
        if let Node::Call(ref c) = *node {
            let args = c.args();
            if args.len() == 2 {
                if let Node::Symbol(ref s) = args[0] {
                    if let Node::Symbol(ref to_s) = args[1] {
                        self.insert(s.clone(), n_alias![to_s.ns().unwrap(), to_s.name().clone()]);
                        Ok(n_list![])
                    } else {
                        Err(IncorrectTypeOfArgumentError(args[1].clone()))
                    }
                } else {
                    Err(IncorrectTypeOfArgumentError(args[0].clone()))
                }
            } else {
                Err(IncorrectNumberOfArgumentsError(node.clone()))
            }
        } else {
            Err(DispatchError(node.clone()))
        }
    }

    fn eval_call_custom(&mut self, node: &Node) -> EvalResult {
        if let Node::Call(ref c) = *node {
            let (symbol, args) = (c.symbol(), c.args());
            let func = try!(self.get(symbol)
                                .map(|e| Ok(e.clone()))
                                .unwrap_or_else(|| Err(ResolveError(symbol.name().clone()))));
            match func {
                Node::Fn(ref f) => {
                    if args.len() != f.params().len() {
                        return Err(IncorrectNumberOfArgumentsError(node.clone()))
                    }

                    let mut e_args = vec![];
                    for a in args {
                        e_args.push(try!(self.eval(a)))
                    }

                    let ref mut fn_state = State::new_chained(self);
                    for (p, a) in f.params().iter().zip(e_args.iter()) {
                        if let &Node::Symbol(ref s) = p {
                            fn_state.insert(s.clone(), a.clone());
                        } else {
                            return Err(IncorrectTypeOfArgumentError(p.clone()))
                        }
                    }

                    let mut result = n_list![];
                    for e in f.body() {
                        result = try!(fn_state.eval(e));
                    }

                    Ok(result)

                },
                Node::Macro(ref m) => {
                    if args.len() != m.params().len() {
                        return Err(IncorrectNumberOfArgumentsError(node.clone()))
                    }

                    let ref mut macro_state = State::new_chained(self);
                    for (p, a) in m.params().iter().zip(args.iter()) {
                        if let Node::Symbol(ref s) = *p {
                            macro_state.insert(s.clone(), a.clone());
                        } else {
                            return Err(IncorrectTypeOfArgumentError(p.clone()))
                        }
                    }

                    let mut result = n_list![];
                    for e in m.body() {
                        result = try!(macro_state.eval(&e));
                    }

                    Ok(try!(macro_state.expand(&result)))
                },
                _ => {
                    Err(IncorrectTypeOfArgumentError(node.clone()))
                }
            }
        } else {
            Err(DispatchError(node.clone()))
        }
    }

    fn expand(&mut self, node: &Node) -> EvalResult {
        if let Node::List(ref l) = *node {
            if l.len() > 0 {
                if let Node::Symbol(ref s) = l[0] {
                    match &s.name()[..] {
                        "def" => {
                            return self.expand_def(node)
                        },
                        "fn" => {
                            return self.expand_fn(node)
                        },
                        "macro" => {
                            return self.expand_macro(node)
                        },
                        "quote" => {
                            return self.expand_quote(node)
                        },
                        "syntax-quote" => {
                            return self.expand_syntax_quote(node)
                        },
                        "unquote" => {
                            return self.expand_unquote(node)
                        },
                        "unquote-splicing" => {
                            return self.expand_unquote_splicing(node)
                        },
                        "let" => {
                            return self.expand_let(node)
                        },
                        _ => {
                            return self.expand_call(node)
                        }
                    }
                } else {
                    return Err(DispatchError(l[0].clone()))
                }
            }
        }
        Ok(node.clone())
    }

    fn expand_quoted(&mut self, node: &Node) -> EvalResult {
        match *node {
            Node::List(ref l) if l.len() > 0 => {
                if l[0].is_symbol("unquote") || l[0].is_symbol("unquote-splicing") {
                    self.expand(node)
                } else {
                    let mut v = vec![];
                    for i in l {
                        v.push(try!(self.expand_quoted(i)));
                    }
                    Ok(Node::List(v))
                }
            },
            _ => {
                self.expand(node)
            }
        }
    }

    fn expand_syntax_quoted(&mut self, node: &Node) -> EvalResult {
        match *node {
            Node::Symbol(ref s) if s.ns().is_none() => {
                Ok(n_symbol![Some(self.get_current().clone()), s.name()])
            },
            Node::List(ref l) if l.len() > 0 => {
                if l[0].is_symbol("unquote") || l[0].is_symbol("unquote-splicing") {
                    self.expand(node)
                } else {
                    let mut v = vec![];
                    for i in l {
                        v.push(try!(self.expand_syntax_quoted(i)));
                    }
                    Ok(Node::List(v))
                }
            },
            _ => {
                self.expand(node)
            }
        }
    }

    fn expand_def(&mut self, node: &Node) -> EvalResult {
        if let Node::List(ref l) = *node {
            if l.len() == 3 {
                if let Node::Symbol(ref s) = l[1] {
                    Ok(n_def![s.name(), try!(self.expand(&l[2]))])
                } else {
                    Err(IncorrectTypeOfArgumentError(l[1].clone()))
                }
            } else {
                Err(IncorrectNumberOfArgumentsError(node.clone()))
            }
        } else {
            Err(DispatchError(node.clone()))
        }
    }

    fn expand_fn(&mut self, node: &Node) -> EvalResult {
        if let Node::List(ref l) = *node {
            if l.len() >= 3 {
                if let Node::Vec(ref params) = l[1] {
                    let mut fn_params = vec![];
                    for p in params {
                        fn_params.push(try!(self.expand(p)))
                    }
                    let mut fn_body = vec![];
                    for be in &l[2..] {
                        fn_body.push(try!(self.expand(be)))
                    }
                    Ok(n_fn![fn_params, fn_body])
                } else {
                    Err(IncorrectTypeOfArgumentError(l[1].clone()))
                }
            } else {
                Err(IncorrectNumberOfArgumentsError(node.clone()))
            }
        } else {
            Err(DispatchError(node.clone()))
        }
    }

    fn expand_macro(&mut self, node: &Node) -> EvalResult {
        if let Node::List(ref l) = *node {
            if l.len() >= 3 {
                if let Node::Vec(ref params) = l[1] {
                    let mut macro_params = vec![];
                    for p in params {
                        macro_params.push(try!(self.expand(p)))
                    }
                    let mut macro_body = vec![];
                    for be in &l[2..] {
                        macro_body.push(try!(self.expand(be)))
                    }
                    Ok(n_macro![macro_params, macro_body])
                } else {
                    Err(IncorrectTypeOfArgumentError(l[1].clone()))
                }
            } else {
                Err(IncorrectNumberOfArgumentsError(node.clone()))
            }
        } else {
            Err(DispatchError(node.clone()))
        }
    }

    fn expand_quote(&mut self, node: &Node) -> EvalResult {
        if let Node::List(ref l) = *node {
            if l.len() == 2 {
                Ok(n_call!["quote", vec![try!(self.expand_quoted(&l[1]))]])
            } else {
                Err(IncorrectNumberOfArgumentsError(node.clone()))
            }
        } else {
            Err(DispatchError(node.clone()))
        }
    }

    fn expand_syntax_quote(&mut self, node: &Node) -> EvalResult {
        if let Node::List(ref l) = *node {
            if l.len() == 2 {
                Ok(n_call!["syntax-quote", vec![try!(self.expand_syntax_quoted(&l[1]))]])
            } else {
                Err(IncorrectNumberOfArgumentsError(node.clone()))
            }
        } else {
            Err(DispatchError(node.clone()))
        }
    }

    fn expand_unquote(&mut self, node: &Node) -> EvalResult {
        if let Node::List(ref l) = *node {
            if l.len() == 2 {
                Ok(n_call!["unquote", vec![try!(self.expand(&l[1]))]])
            } else {
                Err(IncorrectNumberOfArgumentsError(node.clone()))
            }
        } else {
            Err(DispatchError(node.clone()))
        }
    }

    fn expand_unquote_splicing(&mut self, node: &Node) -> EvalResult {
        if let Node::List(ref l) = *node {
            if l.len() == 2 {
                Ok(n_call!["unquote-splicing", vec![try!(self.expand(&l[1]))]])
            } else {
                Err(IncorrectNumberOfArgumentsError(node.clone()))
            }
        } else {
            Err(DispatchError(node.clone()))
        }
    }

    fn expand_let(&mut self, node: &Node) -> EvalResult {
        if let Node::List(ref l) = *node {
            if l.len() >= 3 {
                if let Node::Vec(ref v) = l[1] {
                    if v.len() % 2 == 0 {
                        let mut let_bindings = vec![];
                        for c in v.chunks(2) {
                            if let Some(s @ &Node::Symbol(..)) = c.first() {
                                let_bindings.push(s.clone())
                            } else {
                                return Err(IncorrectTypeOfArgumentError(node.clone()))
                            }
                            if let Some(ref e) = c.last() {
                                let_bindings.push(try!(self.expand(e)))
                            } else {
                                return Err(IncorrectTypeOfArgumentError(node.clone()))
                            }
                        }
                        let mut let_body = vec![];
                        for be in &l[2..] {
                            let_body.push(try!(self.expand(be)))
                        }
                        Ok(n_let![let_bindings, let_body])
                    } else {
                        Err(IncorrectNumberOfArgumentsError(node.clone()))
                    }
                } else {
                    Err(IncorrectTypeOfArgumentError(node.clone()))
                }
            } else {
                Err(IncorrectNumberOfArgumentsError(node.clone()))
            }
        } else {
            Err(DispatchError(node.clone()))
        }
    }

    fn expand_call(&mut self, node: &Node) -> EvalResult {
        if let Node::List(ref l) = *node {
            if let Node::Symbol(ref s) = l[0] {
                let mut args = vec![];
                for a in &l[1..] {
                    args.push(try!(self.expand(a)))
                }
                let call = n_call![s.ns().map(|ns| ns.clone()), s.name().clone(), args];
                if self.get(s).map_or(false, |e| e.is_macro()) {
                    Ok(try!(self.eval(&call)))
                } else {
                    Ok(call)
                }
            } else {
                Err(IncorrectTypeOfArgumentError(l[0].clone()))
            }
        } else {
            Err(DispatchError(node.clone()))
        }
    }
}
