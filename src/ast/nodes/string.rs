use std::fmt;
use std::string;

pub struct String {
    value: string::String
}

impl String {
    pub fn new(value: string::String) -> String {
        String {
            value: value
        }
    }

    pub fn value(&self) -> &string::String {
        &self.value
    }
}

impl fmt::Display for String {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, r#""{}""#, self.value)
    }
}
