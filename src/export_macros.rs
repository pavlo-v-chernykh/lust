#[macro_export]
macro_rules! e_number {
    ($e:expr) => ($crate::Node::Number($e))
}

#[macro_export]
macro_rules! e_bool {
    ($e:expr) => ($crate::Node::Bool($e))
}

#[macro_export]
macro_rules! e_string {
    ($e:expr) => ($crate::Node::String($e.to_string()))
}

#[macro_export]
macro_rules! e_symbol {
    ($name:expr) => ($crate::Node::Symbol {
        ns: None,
        name: $name.to_string(),
    });
    ($ns:expr, $name:expr) => ($crate::Node::Symbol {
        ns: Some($ns.to_string()),
        name: $name.to_string(),
    });
}

#[macro_export]
macro_rules! e_list {
    ($($e:expr),*) => ($crate::Node::List(vec![$($e),*]))
}

#[macro_export]
macro_rules! e_vec {
    ($($e:expr),*) => ($crate::Node::Vec(vec![$($e),*]))
}

#[macro_export]
macro_rules! e_def {
    ($sym:expr, $e:expr) => ($crate::Node::Def {
        sym: $sym.to_string(),
        expr: Box::new($e),
    })
}

#[macro_export]
macro_rules! e_fn {
    ([$($params:expr),*], [$($e:expr),*]) => ($crate::Node::Fn {
        params: vec![$($params),*],
        body: vec![$($e),*],
    })
}

#[macro_export]
macro_rules! e_macro {
    ([$($params:expr),*], [$($e:expr),*]) => ($crate::Node::Macro {
        params: vec![$($params),*],
        body: vec![$($e),*],
    })
}

#[macro_export]
macro_rules! e_call {
    ($name:expr, $($arg:expr), *) => ($crate::Node::Call {
        ns: None,
        name: $name.to_string(),
        args: vec![$($arg),*]
    });
   ($ns:expr, $name:expr, $($arg:expr),*) => (::ast::Node::Call {
        ns: Some($ns.to_string()),
        name: $name.to_string(),
        args: vec![$($arg),*],
    });
}

#[macro_export]
macro_rules! is_file {
    ($md:expr) => ($md.map(|s| s.is_file()).unwrap_or(false))
}

#[macro_export]
macro_rules! is_file_exists {
    ($md:expr) => ($md.is_ok())
}
