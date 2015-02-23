#[derive(Debug, PartialEq)]
pub enum Token {
    Number(f64),
    String(String),
    Symbol(String),
    ListStart,
    ListEnd,
}
