use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub struct Bool {
    value: bool,
}

impl Bool {
    pub fn new(value: bool) -> Bool {
        Bool { value: value }
    }

    pub fn value(&self) -> bool {
        self.value
    }
}

impl fmt::Display for Bool {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}
