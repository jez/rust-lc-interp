use std::io;
use std::io::prelude::*;

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub lc);

pub mod ast;
pub mod eval;

fn eval_one(parser: &lc::ParserNodeParser, line: String) {
    let parser_node = match parser.parse(&line) {
        Err(err) => {
            println!("🛑 {:?}", err);
            return
        }
        Ok(parser_node) => parser_node,
    };
    println!("Parsed: {:?}", &parser_node);

    let expr = match ast::bind(&parser_node) {
        Err(err) => {
            println!("🛑 {:?}", err);
            return
        }
        Ok(expr) => expr,
    };
    println!("Bound:  {:?}", expr);

    println!("Eval trace:");
    println!("   {}", expr);
    eval::eval(expr);
}

fn prompt() {
    print!("\nλ> ");
    io::stdout().flush().unwrap();
}

fn main() {
    let parser = lc::ParserNodeParser::new();

    eval_one(&parser, r"(\x -> x) (\x -> x)".to_string());

    println!("Expect unbound variable:");
    eval_one(&parser, r"(\x -> x) x".to_string());

    let false_ = r"(\t -> \f -> f)";
    let true_  = r"(\t -> \f -> t)";
    let if_    = r"(\x -> \t -> \f -> x t f)";
    eval_one(&parser, format!("{} {} {}", if_, true_, false_).to_string());

    let _zero = r"(\s -> \z -> z)";
    let one   = r"(\s -> \z -> s z)";
    let two   = r"(\s -> \z -> s (s z))";

    let add   = r"(\m -> \n -> \s -> \z -> m s (n s z))";
    eval_one(&parser, format!("{} {} {}", add, one, two).to_string());

    prompt();
    for maybe_line in io::stdin().lock().lines() {
        let line = &maybe_line.unwrap();
        eval_one(&parser, line.to_string());
        prompt();
    }
}
