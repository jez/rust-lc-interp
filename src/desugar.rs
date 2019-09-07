use crate::parser::{App, Lam, Node};

pub fn desugar(mut parser_node: Box<Node>) -> Box<Node> {
    *parser_node = match *parser_node {
        Node::Var(var) => Node::Var(var),

        Node::App(mut app) => {
            app.f = desugar(app.f);
            app.arg = desugar(app.arg);
            Node::App(app)
        }

        Node::Lam(mut lam) => {
            lam.body = desugar(lam.body);
            Node::Lam(lam)
        }

        Node::Let(let_) => {
            let what = desugar(let_.what);
            let in_where = desugar(let_.in_where);

            Node::App(App {
                loc: let_.loc,
                f: Box::new(Node::Lam(Lam {
                    loc: let_.loc,
                    param: let_.bind,
                    body: in_where,
                })),
                arg: what,
            })
        }
    };
    parser_node
}
