use std::fmt;
use ast::Node;
use utils::format_vec;

#[derive(Debug, PartialEq, Clone)]
pub struct Vector {
    vector: Vec<Node>,
}

impl Vector {
    pub fn new(vector: Vec<Node>) -> Vector {
        Vector { vector: vector }
    }

    pub fn vector(&self) -> &[Node] {
        &self.vector[..]
    }
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", format_vec(&self.vector[..]))
    }
}
