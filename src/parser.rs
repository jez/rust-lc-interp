lalrpop_mod!(pub lc);

use crate::global_state::{File, FileRef, GlobalState};
use crate::loc::Loc;

#[derive(Debug, Clone)]
pub struct Var {
    pub loc: Loc,
    pub var: String,
}

#[derive(Debug, Clone)]
pub struct App {
    pub loc: Loc,
    pub f: Box<Node>,
    pub arg: Box<Node>,
}

#[derive(Debug, Clone)]
pub struct Lam {
    pub loc: Loc,
    pub param: String,
    pub body: Box<Node>,
}

#[derive(Debug, Clone)]
pub struct Let {
    pub loc: Loc,
    pub bind: String,
    pub what: Box<Node>,
    pub in_where: Box<Node>,
}

#[derive(Debug, Clone)]
pub enum Node {
    Var(Var),
    App(App),
    Lam(Lam),
    Let(Let),
}

pub type ParseResult = Result<Box<Node>, String>;

pub fn parse_string(string: &str) -> ParseResult {
    let parser = lc::NodeParser::new();
    match parser.parse(File::no_file(), string) {
        Err(err) => Err(format!("{}", err)),
        Ok(node) => Ok(node),
    }
}

pub fn parse(gs: &GlobalState, file: FileRef) -> ParseResult {
    let parser = lc::NodeParser::new();
    match parser.parse(file, file.data(gs).contents()) {
        // TODO(jez) Structured errors
        Err(err) => Err(format!("{}", err)),
        Ok(node) => Ok(node),
    }
}
