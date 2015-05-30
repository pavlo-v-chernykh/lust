use std::fmt;
use ast::Node;
use utils::format_vec;

#[derive(Debug, PartialEq, Clone)]
pub struct List {
    list: Vec<Node>,
}

impl List {
    pub fn new(list: Vec<Node>) -> List {
        List { list: list }
    }

    pub fn list(&self) -> &[Node] {
        &self.list[..]
    }
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", format_vec(&self.list[..]))
    }
}
