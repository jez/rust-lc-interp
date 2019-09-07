use std::convert::TryFrom;

use crate::expr::Expr;
use crate::parser::{App, Node};

use crate::global_state::*;

struct BindContext<'a> {
    gs: &'a mut GlobalState,
    bound: Vec<NameRef>,
}

fn bind_impl(ctx: &mut BindContext, parser_node: &Node) -> Result<Box<Expr>, String> {
    match parser_node {
        Node::Var(v) => {
            let var_name = ctx.gs.enter_name(&v.var);
            let idx = match ctx
                .bound
                .iter()
                .rev()
                .enumerate()
                .find(|&x| *x.1 == var_name)
            {
                None => {
                    return Err(format!(
                        "Unbound variable: {} at {}",
                        v.var,
                        v.loc.show(ctx.gs)
                    ))
                }
                Some((idx, _)) => idx,
            };

            Ok(Box::new(Expr::Var {
                loc: v.loc,
                var: u32::try_from(idx).unwrap(),
            }))
        }
        Node::App(App {
            ref loc,
            ref f,
            ref arg,
        }) => {
            let f = bind_impl(ctx, f)?;
            let arg = bind_impl(ctx, arg)?;
            Ok(Box::new(Expr::App { loc: *loc, f, arg }))
        }
        Node::Lam { loc, param, body } => {
            let name = ctx.gs.enter_name(param);
            ctx.bound.push(name);
            let result = bind_impl(ctx, body)?;
            ctx.bound.pop();
            Ok(Box::new(Expr::Lam {
                loc: *loc,
                body: result,
            }))
        }

        Node::Let { .. } => panic!("Should have been removed by desugar"),
    }
}

pub fn bind(gs: &mut GlobalState, parser_node: &Node) -> Result<Box<Expr>, String> {
    // In reality, we would initialize GlobalState in main
    let bound = Vec::new();
    let mut ctx = BindContext { gs, bound };
    let result = bind_impl(&mut ctx, parser_node);
    result
}
