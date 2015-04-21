use std::collections::HashMap;
use scope::Scope;

pub struct State<'s> {
    default_ns: String,
    namespaces: HashMap<String, Scope<'s>>,
}

impl<'s> State<'s> {
    pub fn new(default: String) -> State<'s> {
        State {
            default_ns: default,
            namespaces: HashMap::new(),
        }
    }
}
