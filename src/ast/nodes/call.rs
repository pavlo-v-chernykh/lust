use std::fmt;
use ast::Node;
use utils::format_vec;

#[derive(Debug, PartialEq, Clone)]
pub struct Call {
    ns: Option<String>,
    name: String,
    args: Vec<Node>,
}

impl Call {
    pub fn new(ns: Option<String>, name: String, args: Vec<Node>) -> Call {
        Call {
            ns: ns,
            name: name,
            args: args,
        }
    }

    pub fn ns(&self) -> Option<&String> {
        self.ns.as_ref()
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn args(&self) -> &Vec<Node> {
        &self.args
    }
}

impl fmt::Display for Call {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut a = "(".to_string();
        if let Some(ref ns) = self.ns {
            a.push_str(&format!("{}/{}", ns, self.name));
        } else {
            a.push_str(&format!("{}", self.name));
        }
        if self.args.is_empty() {
            a.push_str(")")
        } else {
            a.push_str(&format!(" {})", format_vec(&self.args[..])))
        }
        write!(f, "{}", a)
    }
}
