use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub struct Alias {
    ns: String,
    name: String,
}

impl Alias {
    pub fn new(ns: String, name: String) -> Alias {
        Alias {
            ns: ns,
            name: name,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn ns(&self) -> &String {
        &self.ns
    }
}

impl fmt::Display for Alias {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.ns, self.name)
    }
}
