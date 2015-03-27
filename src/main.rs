#![feature(path_ext)]
#![feature(io)]
extern crate "rustc-serialize" as rustc_serialize;
extern crate docopt;

#[macro_use]
mod lexer;
#[macro_use]
mod expr;
#[macro_use]
mod parser;
mod scope;

use std::io::{self, Write, Read, BufReader};
use std::path::Path;
use std::fs::{File, PathExt};
use docopt::Docopt;
use parser::Parser;
use scope::Scope;

static USAGE: &'static str = "
Usage:
    lust [options] <expression>
    lust options [<expression>]

Options:
    -f <file_path>, --file <file_path>          Evaluate expresions from file
    -i, --interactive                           Run REPL session
";

#[derive(RustcDecodable, Debug)]
struct CliArgs {
    arg_expr: Option<String>,
    flag_file: Option<String>,
    flag_interactive: bool,
}

#[cfg_attr(test, allow(dead_code))]
fn main() {
    let args: CliArgs = Docopt::new(USAGE).and_then(|d| d.decode()).unwrap_or_else(|e| e.exit());
    let ref mut root_scope = Scope::new_std();
    let ref mut stdin = io::stdin();
    let ref mut stdout = io::stdout();
    let mut last_eval_expr = None;

    if let Some(ref flag_file) = args.flag_file {
        let path = Path::new(flag_file);
        if path.exists() && path.is_file() {
            if let Ok(file) = File::open(&path) {
                let chars = BufReader::new(file).chars().filter_map(|c| c.ok());
                last_eval_expr = Parser::new(chars)
                    .filter_map(|e| e.ok())
                    .map(|e| e.expand(root_scope).and_then(|e| e.eval(root_scope)).ok().unwrap())
                    .last();
            }
        }
    }

    if let Some(ref arg_expr) = args.arg_expr {
        last_eval_expr = Parser::new(arg_expr.chars())
            .filter_map(|e| e.ok())
            .map(|e| e.expand(root_scope).and_then(|e| e.eval(root_scope)).ok().unwrap())
            .last();
    }

    if args.flag_interactive {
        loop {
            print!("-> ");
            stdout.flush().ok();
            let ref mut buf = String::new();
            match stdin.read_line(buf) {
                Ok(_) => {
                    match Parser::new(buf.chars()).next() {
                        Some(Ok(ref expr)) => {
                            match expr.expand(root_scope).and_then(|e| e.eval(root_scope)) {
                                Ok(ref res) => {
                                    println!("{}", res);
                                },
                                Err(e) => {
                                    println!("Whoops, error detected.\n{}.\n\
                                              Please, try again...", e);
                                }
                            }
                        },
                        Some(Err(e)) => {
                            println!("Whoops, error detected.\n{}.\n\
                                      Please, try again...", e);
                        },
                        None => {
                            println!("Empty input.\n\
                                      Please, try again...");
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
    } else if let Some(ref expr) = last_eval_expr {
        println!("{}", expr);
    }
}
