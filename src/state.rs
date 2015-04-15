use std::collections::HashMap;
use namespace::Namespace;

struct State<'ns> {
    current: String,
    namespaces: HashMap<String, Namespace<'ns>>
}
