extern crate rustc_serialize;
extern crate docopt;
#[macro_use]
extern crate lust;

use std::io::{self, Write, Read};
use std::path::Path;
use std::fs::{File, metadata};
use docopt::Docopt;
use lust::Parser;
use lust::State;

macro_rules! try_ok {
    ($e:expr) => (match $e {
        Ok(res) => {
            res
        },
        Err(err) => {
            return println!("Whoops, error detected.\n{}.\n\
                             Please, try again...", err)
        }
    })
}

static USAGE: &'static str = "
Usage:
    lust [options] [<expr>]

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
    let args = try_ok!(Docopt::new(USAGE).and_then(|d| d.decode::<CliArgs>()));
    let ref mut state = State::new("user".to_string());
    let mut last_evaled = None;

    if let Some(ref flag_file) = args.flag_file {
        let path = Path::new(flag_file);
        let md = metadata(path);
        if is_file_exists!(md) {
            if is_file!(md) {
                let mut file = try_ok!(File::open(&path));
                let ref mut buf = String::new();
                try_ok!(file.read_to_string(buf));
                for parsed_expr in Parser::new(buf.chars()) {
                    last_evaled = Some(try_ok!(state.eval(&try_ok!(parsed_expr))));
                }
            } else {
                return println!("Whoops, error detected.\n\
                                 Specified path is not a file.\n\
                                 Please, specify existing file.");
            }
        } else {
            return println!("Whoops, error detected.\n\
                             File doesn't exist.\n\
                             Please, specify existing file.");
        }
    }

    if let Some(ref arg_expr) = args.arg_expr {
        for parsed_expr in Parser::new(arg_expr.chars()) {
            last_evaled = Some(try_ok!(state.eval(&try_ok!(parsed_expr))))
        }
    }


    if args.flag_interactive {
        let mut stdin = io::stdin();
        let mut stdout = io::stdout();

        loop {
            print!("-> ");
            stdout.flush().ok();
            let ref mut buf = String::new();
            if try_ok!(stdin.read_line(buf)) > 0 {
                for expr in Parser::new(buf.chars()) {
                    match expr {
                        Ok(parsed_expr) => {
                            match state.eval(&parsed_expr) {
                                Ok(res) => {
                                    println!("{}", res);
                                },
                                Err(err) => {
                                    println!("Whoops, error detected.\n{}.\n\
                                              Please, try again...", err);
                                }
                            }
                        },
                        Err(err) => {
                            println!("Whoops, error detected.\n{}.\n\
                                      Please, try again...", err)
                        },
                    }
                }
            } else {
                return println!("\nHope you enjoyed.\nSee you...");
            }
        }
    } else if let Some(ref expr) = last_evaled {
        println!("{}", expr);
    }
}
