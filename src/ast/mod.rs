#[cfg(test)]
mod tests;

use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
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
    List(Vec<Expr>),
    Vec(Vec<Expr>),
    Let {
        bindings: Vec<Expr>,
        body: Vec<Expr>,
    },
    Fn {
        params: Vec<Expr>,
        body: Vec<Expr>,
    },
    Macro {
        params: Vec<Expr>,
        body: Vec<Expr>,
    },
    Def {
        sym: String,
        expr: Box<Expr>,
    },
    Call {
        ns: Option<String>,
        name: String,
        args: Vec<Expr>,
    },
}

impl Expr {
    pub fn as_bool(&self) -> bool {
        if let Expr::Bool(b) = *self {
            b
        } else {
            true
        }
    }

    pub fn is_symbol(&self, name: &str) -> bool {
        if let Expr::Symbol { name: ref n, .. } = *self {
            &n[..] == name
        } else {
            false
        }
    }

    pub fn is_macro(&self) -> bool {
        if let Expr::Macro { .. } = *self {
            true
        } else {
            false
        }
    }

    pub fn is_call_of(&self, name: &str) -> bool {
        if let Expr::Call { name: ref n, .. } = *self {
            &n[..] == name
        } else {
            false
        }
    }
}

fn format_vec(v: &[Expr]) -> String {
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

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Expr::Number(n) => {
                write!(f, "{}", n)
            },
            Expr::Bool(b) => {
                write!(f, "{}", b)
            },
            Expr::Symbol { ref ns, ref name, .. } => {
                if let Some(ref ns) = *ns {
                    write!(f, "{}/{}", ns, name)
                } else {
                    write!(f, "{}", name)
                }
            },
            Expr::Keyword { ref ns, ref name, .. } => {
                if let Some(ref ns) = *ns {
                    write!(f, ":{}/{}", ns, name)
                } else {
                    write!(f, ":{}", name)
                }
            },
            Expr::String(ref s) => {
                write!(f, r#""{}""#, s)
            },
            Expr::List(ref l) => {
                write!(f, "({})", format_vec(l))
            },
            Expr::Vec(ref v) => {
                write!(f, "[{}]", format_vec(v))
            },
            Expr::Def { ref sym, ref expr } => {
                write!(f, "(def {} {})", sym, expr)
            },
            Expr::Let { ref bindings, ref body } => {
                write!(f, "(let [{}] {})", format_vec(bindings), format_vec(body))
            },
            Expr::Fn { ref params, ref body } => {
                write!(f, "(fn [{}] {})", format_vec(params), format_vec(body))
            },
            Expr::Macro { ref params, ref body } => {
                write!(f, "(macro [{}] {})", format_vec(params), format_vec(body))
            },
            Expr::Call { ref ns, ref name, ref args } => {
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
