#[macro_use]
mod lexer;
#[macro_use]
mod expr;
#[macro_use]
mod parser;
mod scope;

use std::io;
use parser::Parser;
use scope::Scope;

#[cfg_attr(test, allow(dead_code))]
fn main() {
    let ref mut root_scope = Scope::new_std();
    let ref mut stdin = io::stdin();
    loop {
        print!("-> ");
        let ref mut buf = String::new();
        match stdin.read_line(buf) {
            Ok(_) => {
                match Parser::new(buf.chars()).parse() {
                    Ok(ref expr) => {
                        match expr.expand(root_scope) {
                            Ok(ref expr) => {
                                match expr.eval(root_scope) {
                                    Ok(ref res) => {
                                        println!("{}", res);
                                    },
                                    Err(e) => {
                                        println!("Whoops, error detected.\n{}.\n\
                                                  Please, try again...", e);
                                    }
                                }
                            },
                            Err(e) => {
                                println!("Whoops, error detected.\n{}.\n\
                                          Please, try again...", e);
                            }
                        }
                    },
                    Err(e) => {
                        println!("Whoops, error detected.\n{}.\n\
                                  Please, try again...", e);
                    }
                }
            },
            Err(e) => {
                println!("Whoops, error detected.\n{}.\n\
                          Please, try again...", e);
                break
            }
        }
    }
}
