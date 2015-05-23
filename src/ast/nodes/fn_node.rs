use std::fmt;
use ast::Node;
use utils::format_vec;

#[derive(Debug, PartialEq, Clone)]
pub struct Fn {
    params: Vec<Node>,
    body: Vec<Node>,
}

impl Fn {
    pub fn new(params: Vec<Node>, body: Vec<Node>) -> Fn {
        Fn {
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

impl fmt::Display for Fn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(fn [{}] {})", format_vec(&self.params[..]), format_vec(&self.body[..]))
    }
}
