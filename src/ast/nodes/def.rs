use ast::Node;
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub struct Def {
    sym: String,
    expr: Box<Node>,
}

impl Def {
    pub fn new(sym: String, expr: Node) -> Def {
        Def {
            sym: sym,
            expr: Box::new(expr),
        }
    }

    pub fn sym(&self) -> &String {
        &self.sym
    }

    pub fn expr(&self) -> &Node {
        &self.expr
    }
}

impl fmt::Display for Def {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(def {} {})", self.sym, self.expr)
    }
}
