use std::fmt;
use std::convert::TryFrom;

// TODO(jez) How to add location information?
#[derive(Debug, Clone)]
pub enum ParserNode {
    Var { var: String },
    App { f: Box<ParserNode>, arg: Box<ParserNode> },
    Lam { param: String, body: Box<ParserNode> },
}

#[derive(Debug, Clone)]
pub enum Expr {
    Var { var: u32 },
    App { f: Box<Expr>, arg: Box<Expr> },
    Lam { body: Box<Expr> },
}

fn bind_impl(ctx: &mut Vec<String>, parser_node: &ParserNode) -> Result<Box<Expr>, String> {
    match parser_node {
        ParserNode::Var { var } => {
            let idx = match ctx.iter().rev().enumerate().find(|&x| x.1 == var) {
                // TODO(jez) Better error message when unbound variable
                None => return Err(format!("Unbound variable: {}", var)),
                Some((idx, _)) => idx,
            };

            Ok(Box::new(Expr::Var { var: u32::try_from(idx).unwrap() }))
        }
        ParserNode::App { ref f, ref arg } => {
            Ok(Box::new(Expr::App { f: bind_impl(ctx, f)?, arg: bind_impl(ctx, arg)? }))
        }
        ParserNode::Lam { param, body } => {
            ctx.push(param.clone());
            let result = bind_impl(ctx, body)?;
            ctx.pop();
            Ok(Box::new(Expr::Lam { body: result }))
        }
    }
}

pub fn bind(parser_node: &ParserNode) -> Result<Box<Expr>, String> {
    let mut ctx = vec![];
    bind_impl(&mut ctx, parser_node)
}

impl Expr {
    fn has_unbound(&self, first_unbound: u32) -> bool {
        let n = first_unbound;
        match *self {
            Expr::Var { var } => var >= n,
            Expr::App { ref f, ref arg } => f.has_unbound(n) || arg.has_unbound(n),
            Expr::Lam { ref body } => body.has_unbound(n + 1),
        }
    }

    pub fn sanity_check(&self) {
        debug_assert!(!self.has_unbound(0), "Expr had unbound variables:\n{:?}", self);
    }

    fn lift_impl(&mut self, unbound: u32) {
        match self {
            Expr::Var { ref mut var } if *var >= unbound => *var += 1,
                Expr::Var { .. } => (),
                Expr::App { f, arg } => {
                    f.lift_impl(unbound);
                    arg.lift_impl(unbound);
                }
            Expr::Lam { body } => {
                body.lift_impl(unbound + 1);
            }
        }
    }

    pub fn lift(&mut self) {
        let unbound = 0;
        self.lift_impl(unbound);
    }

    pub fn subst(&mut self, mut what: Box<Expr>, target: u32) {
        match self {
            Expr::Var { ref mut var } if *var == target => {
                *self = *what;
            }
            Expr::Var { ref mut var } if *var >= target => {
                *var -= 1;
            }
            Expr::Var { .. } => (),
            Expr::App { ref mut f, ref mut arg } => {
                // Optimization: avoid cloning if `f` doesn't actually need `what`
                f.subst(what.clone(), target);
                arg.subst(what, target);
            }
            Expr::Lam { ref mut body } => {
                what.lift();
                body.subst(what, target + 1)
            }
        }
    }
}

impl fmt::Display for Box<Expr> {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        match **self {
            Expr::Var { ref var } => write!(out, "#{}", var),
            Expr::App { ref f, ref arg } => write!(out, "({} {})", f, arg),
            Expr::Lam { ref body } => write!(out, "(\\ -> {})", body),
        }
    }
}
