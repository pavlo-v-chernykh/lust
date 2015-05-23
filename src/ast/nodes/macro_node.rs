use std::fmt;
use ast::Node;
use utils::format_vec;

#[derive(Debug, PartialEq, Clone)]
pub struct Macro {
    params: Vec<Node>,
    body: Vec<Node>,
}

impl Macro {
    pub fn new(params: Vec<Node>, body: Vec<Node>) -> Macro {
        Macro {
            params: params,
            body: body,
        }
    }

    pub fn params(&self) -> &Vec<Node> {
        &self.params
    }

    pub fn body(&self) -> &Vec<Node> {
        &self.body
    }
}

impl fmt::Display for Macro {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(macro [{}] {})", format_vec(&self.params[..]), format_vec(&self.body[..]))
    }
}
