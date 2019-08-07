use crate::loc::Loc;

#[derive(Debug, Clone)]
pub enum Node {
    Var { loc: Loc, var: String },
    App { loc: Loc, f: Box<Node>, arg: Box<Node> },
    Lam { loc: Loc, param: String, body: Box<Node> },
}
