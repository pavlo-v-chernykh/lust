#[derive(Debug, PartialEq)]
pub enum Token {
    Number(f64),
    String(String),
    Symbol(String),
    ListStart,
    ListEnd,
}

macro_rules! t_number {
    ($e:expr) => ($crate::token::Token::Number($e));
}

macro_rules! t_string {
    ($e:expr) => ($crate::token::Token::String($e.to_string()));
}

macro_rules! t_symbol {
    ($e:expr) => ($crate::token::Token::Symbol($e.to_string()));
}

macro_rules! t_list_start {
    () => ($crate::token::Token::ListStart);
}

macro_rules! t_list_end {
    () => ($crate::token::Token::ListEnd);
}
