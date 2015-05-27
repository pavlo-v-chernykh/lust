use std::fmt;
use ast::Node;
use ast::nodes::Symbol;
use utils::format_vec;

#[derive(Debug, PartialEq, Clone)]
pub struct Call {
    symbol: Symbol,
    args: Vec<Node>,
}

impl Call {
    pub fn new(symbol: Symbol, args: Vec<Node>) -> Call {
        Call {
            symbol: symbol,
            args: args,
        }
    }

    pub fn symbol(&self) -> &Symbol {
        &self.symbol
    }

    pub fn args(&self) -> &Vec<Node> {
        &self.args
    }
}

impl fmt::Display for Call {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut a = String::new();
        a.push_str(&format!("{}", self.symbol));
        if !self.args.is_empty() {
            a.push_str(&format!(" {}", format_vec(&self.args[..])))
        }
        write!(f, "({})", a)
    }
}
