#![feature(unicode)]
#![feature(old_io)]
use std::old_io;
use interpreter::Interpreter;

mod common;
mod lexer;
mod parser;
mod interpreter;
mod printer;

#[cfg_attr(test, allow(dead_code))]
fn main() {
    let mut intr = Interpreter::new();
    loop {
        print!("-> ");
        let inpt = old_io::stdin().read_line().ok().unwrap();
        println!("{}", intr.eval(inpt.chars()).ok().unwrap());
    }
}
