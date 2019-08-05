// TODO(jez) How to add location information?
#[derive(Debug, Clone)]
pub enum Node {
    Var { var: String },
    App { f: Box<Node>, arg: Box<Node> },
    Lam { param: String, body: Box<Node> },
}
