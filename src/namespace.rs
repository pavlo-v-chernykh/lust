use ::scope::Scope;
use ::ast::Expr;

pub struct Namespace<'a> {
    name: String,
    mappings: Scope<'a>,
}

impl<'a> Namespace<'a> {
    fn new(name: String) -> Namespace<'a> {
        Namespace {
            name: name,
            mappings: Scope::new_std(),
        }
    }

    pub fn insert(&mut self, s: String, e: Expr) -> Option<Expr> {
        self.mappings.insert(s, e)
    }

    pub fn get(&self, s: &String) -> Option<&Expr> {
        self.mappings.get(s)
    }
}
