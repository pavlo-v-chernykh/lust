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

#[cfg(test)]
mod tests {
    use super::Scope;

    #[test]
    fn test_insert_to_and_get_from_root_scope() {
        let mut scope = Scope::new();
        let key = "rust-is-terrific".to_string();
        let val = e_number!(10.5);
        scope.insert(key.clone(), val.clone());
        assert_eq!(&val, scope.get(&key).unwrap());
    }

    #[test]
    fn test_insert_to_and_get_from_child_scope() {
        let root_scope = Scope::new();
        let mut scope = Scope::new_chained(&root_scope);
        let key = "rust-is-terrific".to_string();
        let val = e_number!(10.5);
        scope.insert(key.clone(), val.clone());
        assert_eq!(&val, scope.get(&key).unwrap());
    }

    #[test]
    fn test_insert_to_root_scope_and_get_from_child_scope() {
        let mut root_scope = Scope::new();
        let key = "rust-is-terrific".to_string();
        let val = e_number!(10.5);
        root_scope.insert(key.clone(), val.clone());
        let scope = Scope::new_chained(&root_scope);
        assert_eq!(&val, scope.get(&key).unwrap());
    }

    #[test]
    fn test_insert_to_child_scope_and_get_none_from_root_scope() {
        let root_scope = Scope::new();
        let key = "rust-is-terrific".to_string();
        let val = e_number!(10.5);
        let mut scope = Scope::new_chained(&root_scope);
        scope.insert(key.clone(), val.clone());
        assert!(root_scope.get(&key).is_none());
    }

    #[test]
    fn test_shadow_val_in_root_scope() {
        let mut root_scope = Scope::new();
        let key = "rust-is-terrific".to_string();
        let val_in_root = e_number!(10.5);
        root_scope.insert(key.clone(), val_in_root);
        let val_in_child = e_number!(0_f64);
        let mut scope = Scope::new_chained(&root_scope);
        scope.insert(key.clone(), val_in_child.clone());
        assert_eq!(&val_in_child, scope.get(&key).unwrap());
    }
}
