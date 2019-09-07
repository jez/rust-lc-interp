use crate::parser::Node;

pub fn desugar(mut parser_node: Box<Node>) -> Box<Node> {
    match *parser_node {
        Node::Var { .. } => (),

        Node::App { loc, f, arg } => {
            let f = desugar(f);
            let arg = desugar(arg);
            *parser_node = Node::App { loc, f, arg };
        }

        Node::Lam { loc, param, body } => {
            let body = desugar(body);
            *parser_node = Node::Lam { loc, param, body };
        }

        Node::Let {
            loc,
            bind,
            what,
            in_where,
        } => {
            let what = desugar(what);
            let in_where = desugar(in_where);

            *parser_node = Node::App {
                loc,
                f: Box::new(Node::Lam {
                    loc,
                    param: bind,
                    body: in_where,
                }),
                arg: what,
            }
        }
    }

    parser_node
}
