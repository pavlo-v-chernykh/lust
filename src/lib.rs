#[macro_use]
mod lexer;
#[macro_use]
pub mod ast;
#[macro_use]
mod parser;
mod scope;

pub use parser::Parser;
pub use scope::Scope;
