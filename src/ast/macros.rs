#[macro_export]
macro_rules! e_number {
    ($e:expr) => ($crate::ast::Expr::Number($e))
}

#[macro_export]
macro_rules! e_bool {
    ($e:expr) => ($crate::ast::Expr::Bool($e))
}

#[macro_export]
macro_rules! e_string {
    ($e:expr) => ($crate::ast::Expr::String($e.to_string()))
}

#[macro_export]
macro_rules! e_symbol {
    ($e:expr) => ($crate::ast::Expr::Symbol($e.to_string()))
}

#[macro_export]
macro_rules! e_list {
    ($($e:expr),*) => ($crate::ast::Expr::List(vec![$($e),*]))
}

#[macro_export]
macro_rules! e_vec {
    ($($e:expr),*) => ($crate::ast::Expr::Vec(vec![$($e),*]))
}

#[macro_export]
macro_rules! e_def {
    ($sym:expr, $e:expr) => ($crate::ast::Expr::Def {
        sym: $sym.to_string(),
        expr: Box::new($e),
    })
}

#[macro_export]
macro_rules! e_fn {
    ([$($params:expr),*], [$($e:expr),*]) => ($crate::ast::Expr::Fn {
        params: vec![$($params),*],
        body: vec![$($e),*],
    })
}

#[macro_export]
macro_rules! e_macro {
    ([$($params:expr),*], [$($e:expr),*]) => ($crate::ast::Expr::Macro {
        params: vec![$($params),*],
        body: vec![$($e),*],
    })
}

#[macro_export]
macro_rules! e_call {
    ($name:expr, $($arg:expr), *) => ($crate::ast::Expr::Call {
        name: $name.to_string(),
        args: vec![$($arg),*]
    })
}
