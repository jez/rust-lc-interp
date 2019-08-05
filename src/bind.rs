use std::convert::TryFrom;

use crate::parser::Node;
use crate::expr::Expr;

use crate::global_state::*;

struct BindContext {
    gs: GlobalState,
    bound: Vec<NameRef>,
}

fn bind_impl(ctx: &mut BindContext, parser_node: &Node) -> Result<Box<Expr>, String> {
    match parser_node {
        Node::Var { var } => {
            let var_name = ctx.gs.enter_name(var);
            let idx = match ctx.bound.iter().rev().enumerate().find(|&x| *x.1 == var_name) {
                // TODO(jez) Better error message when unbound variable
                None => return Err(format!("Unbound variable: {}", var)),
                Some((idx, _)) => idx,
            };

            Ok(Box::new(Expr::Var { var: u32::try_from(idx).unwrap() }))
        }
        Node::App { ref f, ref arg } => {
            let f = bind_impl(ctx, f)?;
            let arg = bind_impl(ctx, arg)?;
            Ok(Box::new(Expr::App { f, arg }))
        }
        Node::Lam { param, body } => {
            let name = ctx.gs.enter_name(param);
            ctx.bound.push(name);
            let result = bind_impl(ctx, body)?;
            ctx.bound.pop();
            Ok(Box::new(Expr::Lam { body: result }))
        }
    }
}

pub fn bind(parser_node: &Node) -> Result<Box<Expr>, String> {
    // In reality, we would initialize GlobalState in main
    let gs = GlobalState::new();
    let bound = Vec::new();
    let mut ctx = BindContext { gs, bound };
    bind_impl(&mut ctx, parser_node)
}
