use std::fmt;
use ast::Node;
use utils::format_vec;

#[derive(Debug, PartialEq, Clone)]
pub struct Let {
    bindings: Vec<Node>,
    body: Vec<Node>,
}

impl Let {
    pub fn new(bindings: Vec<Node>, body: Vec<Node>) -> Let {
        Let {
            bindings: bindings,
            body: body,
        }
    }

    pub fn bindings(&self) -> &Vec<Node> {
        &self.bindings
    }

    pub fn body(&self) -> &Vec<Node> {
        &self.body
    }
}

impl fmt::Display for Let {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(let [{}] {})", format_vec(&self.bindings[..]), format_vec(&self.body[..]))
    }
}
