#[cfg(test)]
mod tests;

use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    Number(f64),
    Bool(bool),
    String(String),
    Symbol {
        ns: Option<String>,
        name: String,
    },
    Keyword {
        ns: Option<String>,
        name: String,
    },
    List(Vec<Node>),
    Vec(Vec<Node>),
    Let {
        bindings: Vec<Node>,
        body: Vec<Node>,
    },
    Fn {
        params: Vec<Node>,
        body: Vec<Node>,
    },
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
        if let Node::Bool(b) = *self {
            b
        } else {
            true
        }
    }

    pub fn is_symbol(&self, name: &str) -> bool {
        if let Node::Symbol { name: ref n, .. } = *self {
            &n[..] == name
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

fn format_vec(v: &[Node]) -> String {
    let mut a = String::new();
    if !v.is_empty() {
        let last_idx = v.len() - 1;
        for (i, e) in v.iter().enumerate() {
            if i < last_idx {
                a.push_str(&format!("{} ", e))
            } else {
                a.push_str(&format!("{}", e))
            }
        }
    }
    a
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Node::Number(n) => {
                write!(f, "{}", n)
            },
            Node::Bool(b) => {
                write!(f, "{}", b)
            },
            Node::Symbol { ref ns, ref name, .. } => {
                if let Some(ref ns) = *ns {
                    write!(f, "{}/{}", ns, name)
                } else {
                    write!(f, "{}", name)
                }
            },
            Node::Keyword { ref ns, ref name, .. } => {
                if let Some(ref ns) = *ns {
                    write!(f, ":{}/{}", ns, name)
                } else {
                    write!(f, ":{}", name)
                }
            },
            Node::String(ref s) => {
                write!(f, r#""{}""#, s)
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
            Node::Let { ref bindings, ref body } => {
                write!(f, "(let [{}] {})", format_vec(bindings), format_vec(body))
            },
            Node::Fn { ref params, ref body } => {
                write!(f, "(fn [{}] {})", format_vec(params), format_vec(body))
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
