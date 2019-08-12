#[macro_use]
extern crate lalrpop_util;

use std::io::prelude::*;
use std::io;
use std::path::PathBuf;

use docopt::Docopt;
use serde::Deserialize;

pub mod loc;
pub mod global_state;
pub mod expr;

pub mod parser;
pub mod desugar;
pub mod bind;
pub mod eval;

use global_state::FileRef;
use global_state::GlobalState;

fn eval_string(gs: &mut GlobalState, line: String) {
    eval_node(gs, parser::parse_string(&line))
}

fn eval_file(gs: &mut GlobalState, file: FileRef) {
    eval_node(gs, parser::parse(gs, file))
}

fn eval_node(gs: &mut GlobalState, parsed: parser::ParseResult) {
    let node = match parsed {
        Err(err) => {
            println!("🛑 {}", err);
            return
        }
        Ok(node) => node,
    };

    let desugared = desugar::desugar(node);

    let expr = match bind::bind(gs, &desugared) {
        Err(err) => {
            println!("🛑 {}", err);
            return
        }
        Ok(expr) => expr,
    };

    println!("   {}", expr);
    eval::eval(expr);
}

fn prompt() {
    print!("λ> ");
    io::stdout().flush().unwrap();
}

enum Options {
    Help,
    FromFile(PathBuf),
    Repl,
}

const USAGE: &'static str = "
Toy interpreter for the lambda calculus

Usage:
  lc-interp [<file>]
  lc-interp --help
";

#[derive(Debug, Deserialize)]
struct DocoptArgs {
    arg_file: Option<String>,
    flag_help: bool,
}

fn parse_args() -> Options {
    let args: DocoptArgs = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    if args.flag_help {
        return Options::Help
    }

    match args.arg_file {
        None => return Options::Repl,
        Some(string) => return Options::FromFile(PathBuf::from(string)),
    }
}

fn main() -> io::Result<()> {
    let options = parse_args();

    match options {
        Options::Help => {
            println!("");
            println!("Toy interpreter for the lambda calculus");
            println!("");
            println!("Usage:");
            println!("  lc-interp");
            println!("  lc-interp <file>");
            println!("  lc-interp --help");
            println!("");
        }
        Options::FromFile(path_buf) => {
            let mut gs = GlobalState::new();
            let file = gs.enter_file(path_buf.as_path())?;
            eval_file(&mut gs, file);
        }
        Options::Repl => {
            let mut gs = GlobalState::new();
            prompt();
            for maybe_line in io::stdin().lock().lines() {
                let line = &maybe_line.unwrap();
                eval_string(&mut gs, line.to_string());
                prompt();
            }
        }
    }
    Ok(())
}
