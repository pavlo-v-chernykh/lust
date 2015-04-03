#[cfg(test)]
mod tests;

use std::collections::HashMap;
use ast::Expr;

#[derive(Debug)]
pub struct Scope<'a> {
    env: HashMap<String, Expr>,
    parent: Option<&'a Scope<'a>>
}

impl<'a> Scope<'a> {
    pub fn new() -> Scope<'a> {
        Scope {
            env: HashMap::new(),
            parent: None
        }
    }

    pub fn new_std() -> Scope<'a> {
        let mut scope = Scope::new();
        scope.insert("nil".to_string(), e_list![]);
        scope.insert("true".to_string(), e_bool!(true));
        scope.insert("false".to_string(), e_bool!(false));
        scope
    }

    pub fn new_chained(parent: &'a Scope<'a>) -> Scope<'a> {
        let mut scope = Scope::new();
        scope.parent = Some(parent);
        scope
    }

    pub fn get(&self, s: &String) -> Option<&Expr> {
        let mut scope = self;
        loop {
            let v = scope.env.get(s);
            if v.is_none() && scope.parent.is_some() {
                scope = scope.parent.unwrap();
            } else {
                return v
            }
        }
    }

    pub fn insert(&mut self, s: String, e: Expr) -> Option<Expr> {
        self.env.insert(s, e)
    }
}
