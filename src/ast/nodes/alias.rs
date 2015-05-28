use std::fmt;
use super::Symbol;

#[derive(Debug, PartialEq, Clone)]
pub struct Alias {
    symbol: Symbol,
}

impl Alias {
    pub fn new(symbol: Symbol) -> Alias {
        Alias { symbol: symbol }
    }

    pub fn symbol(&self) -> &Symbol {
        &self.symbol
    }
}

impl fmt::Display for Alias {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.symbol)
    }
}
