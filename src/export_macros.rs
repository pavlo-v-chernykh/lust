#[macro_export]
macro_rules! e_number {
    ($e:expr) => ($crate::Expr::Number($e))
}

#[macro_export]
macro_rules! e_bool {
    ($e:expr) => ($crate::Expr::Bool($e))
}

#[macro_export]
macro_rules! e_string {
    ($e:expr) => ($crate::Expr::String($e.to_string()))
}

#[macro_export]
macro_rules! e_symbol {
    ($e:expr) => ($crate::Expr::Symbol($e.to_string()))
}

#[macro_export]
macro_rules! e_list {
    ($($e:expr),*) => ($crate::Expr::List(vec![$($e),*]))
}

#[macro_export]
macro_rules! e_vec {
    ($($e:expr),*) => ($crate::Expr::Vec(vec![$($e),*]))
}

#[macro_export]
macro_rules! e_def {
    ($sym:expr, $e:expr) => ($crate::Expr::Def {
        sym: $sym.to_string(),
        expr: Box::new($e),
    })
}

#[macro_export]
macro_rules! e_fn {
    ([$($params:expr),*], [$($e:expr),*]) => ($crate::Expr::Fn {
        params: vec![$($params),*],
        body: vec![$($e),*],
    })
}

#[macro_export]
macro_rules! e_macro {
    ([$($params:expr),*], [$($e:expr),*]) => ($crate::Expr::Macro {
        params: vec![$($params),*],
        body: vec![$($e),*],
    })
}

#[macro_export]
macro_rules! e_call {
    ($name:expr, $($arg:expr), *) => ($crate::Expr::Call {
        name: $name.to_string(),
        args: vec![$($arg),*]
    })
}
