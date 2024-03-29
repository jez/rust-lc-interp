use crate::expr::Expr;
use std::fmt;

#[derive(Debug)]
enum TryStep {
    Step,
    Val,
}

impl fmt::Display for TryStep {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TryStep::Step => write!(out, "->"),
            TryStep::Val => write!(out, "=>"),
        }
    }
}

fn trystep(mut expr: Box<Expr>) -> (TryStep, Box<Expr>) {
    match *expr {
        Expr::Var { .. } => (TryStep::Val, expr),

        Expr::App { loc, f, arg } => {
            let f = match trystep(f) {
                (TryStep::Val, f) => f,
                (TryStep::Step, f) => {
                    *expr = Expr::App { loc, f, arg };
                    return (TryStep::Step, expr);
                }
            };

            let arg = match trystep(arg) {
                (TryStep::Val, arg) => arg,
                (TryStep::Step, arg) => {
                    *expr = Expr::App { loc, f, arg };
                    return (TryStep::Step, expr);
                }
            };

            match *f {
                Expr::Lam {
                    loc: _loc,
                    mut body,
                } => {
                    body.subst(arg, 0);
                    *expr = *body;
                    return (TryStep::Step, expr);
                }
                Expr::Var { .. } => (),
                Expr::App { .. } => (),
            }

            *expr = Expr::App { loc, f, arg };
            (TryStep::Val, expr)
        }

        Expr::Lam { loc, body } => {
            let (result, body) = trystep(body);
            *expr = Expr::Lam { loc, body };
            (result, expr)
        }
    }
}

pub fn eval(expr: Box<Expr>) -> Box<Expr> {
    expr.sanity_check();
    let (result, expr) = trystep(expr);
    println!("{} {}", result, expr);

    match result {
        TryStep::Step => eval(expr),
        TryStep::Val => {
            expr.sanity_check();
            expr
        }
    }
}
