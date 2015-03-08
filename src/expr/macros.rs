macro_rules! e_number {
    ($e:expr) => ($crate::expr::Expr::Number($e))
}

macro_rules! e_bool {
    ($e:expr) => ($crate::expr::Expr::Bool($e))
}

macro_rules! e_string {
    ($e:expr) => ($crate::expr::Expr::String($e.to_string()))
}

macro_rules! e_symbol {
    ($e:expr) => ($crate::expr::Expr::Symbol($e.to_string()))
}

macro_rules! e_list {
    ($($e:expr),*) => ($crate::expr::Expr::List(vec![$($e),*]))
}

macro_rules! e_def {
    ($sym:expr, $e:expr) => ($crate::expr::Expr::Def {
        sym: $sym.to_string(),
        expr: Box::new($e),
    })
}

macro_rules! e_fn {
    ([$($params:expr),*], [$($e:expr),*]) => ($crate::expr::Expr::Fn {
        params: vec![$($params),*],
        body: vec![$($e),*],
    })
}

macro_rules! e_macro {
    ([$($params:expr),*], [$($e:expr),*]) => ($crate::expr::Expr::Macro {
        params: vec![$($params),*],
        body: vec![$($e),*],
    })
}

macro_rules! e_call {
    ($name:expr, $($arg:expr), *) => ($crate::expr::Expr::Call {
        name: $name.to_string(),
        args: vec![$($arg),*]
    })
}
