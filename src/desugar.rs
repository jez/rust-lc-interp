use crate::parser::Node;
use crate::parser::Node::*;

pub fn desugar(mut parser_node: Box<Node>) -> Box<Node> {
    match *parser_node {
        Var { .. } => (),

        App { loc, f, arg } => {
            let f = desugar(f);
            let arg = desugar(arg);
            *parser_node = App { loc, f, arg };
        }

        Lam { loc, param, body } => {
            let body = desugar(body);
            *parser_node = Lam { loc, param, body };
        }

        Let { loc, bind, what, in_where } => {
            let what = desugar(what);
            let in_where = desugar(in_where);

            *parser_node = App {
                loc,
                f: Box::new(Lam { loc, param: bind, body: in_where }),
                arg: what,
            }
        }
    }

    parser_node
}
