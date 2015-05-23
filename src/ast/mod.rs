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
    Macro {
        params: Vec<Node>,
        body: Vec<Node>,
    },
    Def {
        sym: String,
        expr: Box<Node>,
    },
    Call {
        ns: Option<String>,
        name: String,
        args: Vec<Node>,
    },
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
        if let Node::Macro { .. } = *self {
            true
        } else {
            false
        }
    }

    pub fn is_call_of(&self, name: &str) -> bool {
        if let Node::Call { name: ref n, .. } = *self {
            &n[..] == name
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
            Node::Def { ref sym, ref expr } => {
                write!(f, "(def {} {})", sym, expr)
            },
            Node::Let(ref l) => {
                write!(f, "{}", l)
            },
            Node::Fn(ref fn_node) => {
                write!(f, "{}", fn_node)
            },
            Node::Macro { ref params, ref body } => {
                write!(f, "(macro [{}] {})", format_vec(params), format_vec(body))
            },
            Node::Call { ref ns, ref name, ref args } => {
                let mut a = "(".to_string();
                if let Some(ref ns) = *ns {
                    a.push_str(&format!("{}/{}", ns, name));
                } else {
                    a.push_str(&format!("{}", name));
                }
                if args.is_empty() {
                    a.push_str(")")
                } else {
                    a.push_str(&format!(" {})", format_vec(args)))
                }
                write!(f, "{}", a)
            },
        }
    }
}
