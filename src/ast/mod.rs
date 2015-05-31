#[cfg(test)]
mod tests;
pub mod nodes;

use std::fmt;
use utils::format_vec;

#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    Number(nodes::Number),
    Bool(nodes::Bool),
    String(nodes::String),
    Symbol(nodes::Symbol),
    Keyword(nodes::Symbol),
    Alias(nodes::Symbol),
    List(nodes::List),
    Vec(Vec<Node>),
    Let(nodes::Let),
    Fn(nodes::Fn),
    Macro(nodes::Fn),
    Def(nodes::Def),
    Call(nodes::Call),
}

impl Node {
    pub fn as_bool(&self) -> bool {
        if let Node::Bool(ref b) = *self {
            b.value()
        } else {
            true
        }
    }

    pub fn is_symbol(&self, name: &str) -> bool {
        if let Node::Symbol(ref s) = *self {
            s.name() == name
        } else {
            false
        }
    }

    pub fn is_macro(&self) -> bool {
        if let Node::Macro(..) = *self {
            true
        } else {
            false
        }
    }

    pub fn is_call_of(&self, name: &str) -> bool {
        if let Node::Call(ref c) = *self {
            &c.symbol().name()[..] == name
        } else {
            false
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Node::Number(ref n) => {
                write!(f, "{}", n)
            },
            Node::Bool(ref b) => {
                write!(f, "{}", b)
            },
            Node::Symbol(ref s) => {
                write!(f, "{}", s)
            },
            Node::Alias(ref s) => {
                write!(f, "{}", s)
            },
            Node::Keyword(ref s) => {
                write!(f, ":{}", s)
            },
            Node::String(ref s) => {
                write!(f, r#""{}""#, s)
            },
            Node::List(ref l) => {
                write!(f, "({})", l)
            },
            Node::Vec(ref v) => {
                write!(f, "[{}]", format_vec(v))
            },
            Node::Def(ref d) => {
                write!(f, "{}", d)
            },
            Node::Let(ref l) => {
                write!(f, "{}", l)
            },
            Node::Fn(ref fn_node) => {
                write!(f, "(fn {})", fn_node)
            },
            Node::Macro(ref fn_node) => {
                write!(f, "(macro {})", fn_node)
            },
            Node::Call(ref c) => {
                write!(f, "{}", c)
            },
        }
    }
}
