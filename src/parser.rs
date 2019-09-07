lalrpop_mod!(pub lc);

use crate::global_state::{File, FileRef, GlobalState};
use crate::loc::Loc;

#[derive(Debug, Clone)]
pub enum Node {
    Var {
        loc: Loc,
        var: String,
    },
    App {
        loc: Loc,
        f: Box<Node>,
        arg: Box<Node>,
    },
    Lam {
        loc: Loc,
        param: String,
        body: Box<Node>,
    },
    Let {
        loc: Loc,
        bind: String,
        what: Box<Node>,
        in_where: Box<Node>,
    },
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
