use std::fmt;

pub struct Symbol {
    ns: Option<String>,
    name: String,
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
