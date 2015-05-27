use std::fmt;
use ast::Node;
use ast::nodes::Symbol;

#[derive(Debug, PartialEq, Clone)]
pub struct Def {
    symbol: Symbol,
    expr: Box<Node>,
}

impl Def {
    pub fn new(symbol: Symbol, expr: Node) -> Def {
        Def {
            symbol: symbol,
            expr: Box::new(expr),
        }
    }

    pub fn symbol(&self) -> &Symbol {
        &self.symbol
    }

    pub fn expr(&self) -> &Node {
        &self.expr
    }
}

impl fmt::Display for Def {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(def {} {})", self.symbol.name(), self.expr)
    }
}
