macro_rules! e_number {
    ($n:expr) => ($crate::parser::Expr::Number($n))
}

macro_rules! e_string {
    ($n:expr) => ($crate::parser::Expr::String($n.to_string()))
}

macro_rules! e_symbol {
    ($n:expr) => ($crate::parser::Expr::Symbol($n.to_string()))
}

macro_rules! e_list {
    ($($l:expr),*) => ($crate::parser::Expr::List(vec![$($l),*]))
}
