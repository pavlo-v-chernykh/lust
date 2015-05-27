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
    Keyword(nodes::Keyword),
    Alias(nodes::Alias),
    List(Vec<Node>),
    Vec(Vec<Node>),
    Let(nodes::Let),
    Fn(nodes::Fn),
    Macro(nodes::Macro),
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
            Node::Alias(ref a) => {
                write!(f, "{}", a)
            },
            Node::Keyword(ref k) => {
                write!(f, "{}", k)
            },
            Node::String(ref s) => {
                write!(f, "{}", s)
            },
            Node::List(ref l) => {
                write!(f, "({})", format_vec(l))
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
                write!(f, "{}", fn_node)
            },
            Node::Macro(ref m) => {
                write!(f, "{}", m)
            },
            Node::Call(ref c) => {
                write!(f, "{}", c)
            },
        }
    }
}
