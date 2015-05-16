use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub struct Number {
    value: f64,
}

impl Number {
    pub fn new(value: f64) -> Number {
        Number { value: value }
    }

    pub fn value(&self) -> f64 {
        self.value
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}
