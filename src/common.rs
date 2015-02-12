#[derive(Debug, PartialEq, Clone)]
pub enum Atom {
    Number(f64),
    Symbol(String),
    Nil
}

#[derive(Debug, PartialEq, Clone)]
pub enum Sexp {
    Atom(Atom),
    List(Vec<Sexp>)
}
