use std::convert::TryFrom;

use crate::parser::Node;
use crate::expr::Expr;

fn bind_impl(ctx: &mut Vec<String>, parser_node: &Node) -> Result<Box<Expr>, String> {
    match parser_node {
        Node::Var { var } => {
            let idx = match ctx.iter().rev().enumerate().find(|&x| x.1 == var) {
                // TODO(jez) Better error message when unbound variable
                None => return Err(format!("Unbound variable: {}", var)),
                Some((idx, _)) => idx,
            };

            Ok(Box::new(Expr::Var { var: u32::try_from(idx).unwrap() }))
        }
        Node::App { ref f, ref arg } => {
            Ok(Box::new(Expr::App { f: bind_impl(ctx, f)?, arg: bind_impl(ctx, arg)? }))
        }
        Node::Lam { param, body } => {
            ctx.push(param.clone());
            let result = bind_impl(ctx, body)?;
            ctx.pop();
            Ok(Box::new(Expr::Lam { body: result }))
        }
    }
}

pub fn bind(parser_node: &Node) -> Result<Box<Expr>, String> {
    let mut ctx = vec![];
    bind_impl(&mut ctx, parser_node)
}
