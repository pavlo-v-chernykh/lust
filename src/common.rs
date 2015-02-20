#[derive(Debug, PartialEq, Clone)]
pub enum Atom {
    Number(f64),
    String(String),
    Symbol(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Sexp {
    Atom(Atom),
    List(Vec<Sexp>)
}
