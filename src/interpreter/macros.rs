macro_rules! v_number {
    ($e:expr) => ($crate::interpreter::Val::Number($e));
}

macro_rules! v_bool {
    ($e:expr) => ($crate::interpreter::Val::Bool($e));
}

macro_rules! v_string {
    ($e:expr) => ($crate::interpreter::Val::String($e.to_string()));
}

macro_rules! v_symbol {
    ($e:expr) => ($crate::interpreter::Val::Symbol($e.to_string()));
}

macro_rules! v_list {
    ($($e:expr),*) => ($crate::interpreter::Val::List(vec![$($e),*]));
}
