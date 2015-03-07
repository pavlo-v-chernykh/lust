macro_rules! n_number {
    ($e:expr) => ($crate::ast::Node::Number($e))
}

macro_rules! n_bool {
    ($e:expr) => ($crate::ast::Node::Bool($e))
}

macro_rules! n_string {
    ($e:expr) => ($crate::ast::Node::String($e.to_string()))
}

macro_rules! n_symbol {
    ($e:expr) => ($crate::ast::Node::Symbol($e.to_string()))
}

macro_rules! n_list {
    ($($e:expr),*) => ($crate::ast::Node::List(vec![$($e),*]))
}
