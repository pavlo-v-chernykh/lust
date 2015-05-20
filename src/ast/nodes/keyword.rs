use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub struct Keyword {
    ns: Option<String>,
    name: String,
}

impl Keyword {
    pub fn new(ns: Option<String>, name: String) -> Keyword {
        Keyword {
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

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(ref ns) = self.ns {
            write!(f, ":{}/{}", ns, self.name)
        } else {
            write!(f, ":{}", self.name)
        }
    }
}
