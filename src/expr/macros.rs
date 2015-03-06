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
