#[cfg(test)]
mod tests;

use std::collections::HashMap;
use ast::Expr;

#[derive(Debug)]
pub struct State<'s> {
    default: String,
    namespaces: HashMap<String, HashMap<String, Expr>>,
    parent: Option<&'s State<'s>>,
    id: usize,
}

impl<'s> State<'s> {
    pub fn new(default: String) -> State<'s> {
        let mut default_ns = HashMap::new();
        default_ns.insert("nil".to_string(), e_list![]);
        default_ns.insert("true".to_string(), e_bool!(true));
        default_ns.insert("false".to_string(), e_bool!(false));

        let mut namespaces = HashMap::new();
        namespaces.insert(default.clone(), default_ns);

        State {
            default: default,
            namespaces: namespaces,
            parent: None,
            id: 0,
        }
    }

    pub fn new_chained(parent: &'s State<'s>) -> State<'s> {
        let mut state = State::new(format!("{}_chained", parent.default));
        state.parent = Some(parent);
        state
    }

    pub fn insert(&mut self, name: String, expr: Expr) -> Option<Expr> {
        self.namespaces
            .get_mut(&self.default)
            .and_then(|scope| scope.insert(name, expr))
    }

    pub fn get(&self, ns: Option<&String>, name: &String) -> Option<&Expr> {
        let mut state = self;
        loop {
            let v = state.namespaces
                         .get(ns.unwrap_or(&state.default))
                         .and_then(|scope| scope.get(name));
            if v.is_none() && state.parent.is_some() {
                state = state.parent.unwrap();
            } else {
                return v
            }
        }
    }

    pub fn next_id(&mut self) -> usize {
        let next = self.id;
        self.id += 1;
        next
    }

}
