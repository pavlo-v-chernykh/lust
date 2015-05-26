use std::fmt;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Symbol {
    ns: Option<String>,
    name: String,
}

impl Symbol {
    pub fn new(ns: Option<String>, name: String) -> Symbol {
        Symbol {
            ns: ns,
            name: name,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn ns(&self) -> Option<&String> {
        self.ns.as_ref()
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(ref ns) = self.ns {
            write!(f, "{}/{}", ns, self.name)
        } else {
            write!(f, "{}", self.name)
        }
    }
}
