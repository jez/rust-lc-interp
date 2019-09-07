use std::fmt;

use crate::loc::Loc;

#[derive(Debug, Clone)]
pub enum Expr {
    Var {
        loc: Loc,
        var: u32,
    },
    App {
        loc: Loc,
        f: Box<Expr>,
        arg: Box<Expr>,
    },
    Lam {
        loc: Loc,
        body: Box<Expr>,
    },
}

impl Expr {
    fn has_unbound(&self, first_unbound: u32) -> bool {
        let n = first_unbound;
        match *self {
            Expr::Var { loc: _loc, var } => var >= n,
            Expr::App {
                loc: ref _loc,
                ref f,
                ref arg,
            } => f.has_unbound(n) || arg.has_unbound(n),
            Expr::Lam {
                loc: ref _loc,
                ref body,
            } => body.has_unbound(n + 1),
        }
    }

    pub fn sanity_check(&self) {
        debug_assert!(
            !self.has_unbound(0),
            "Expr had unbound variables:\n{:?}",
            self
        );
    }

    fn lift_impl(&mut self, unbound: u32) {
        match self {
            Expr::Var {
                loc: ref _loc,
                ref mut var,
            } if *var >= unbound => *var += 1,
            Expr::Var { .. } => (),
            Expr::App { loc: _loc, f, arg } => {
                f.lift_impl(unbound);
                arg.lift_impl(unbound);
            }
            Expr::Lam { loc: _loc, body } => {
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
            Expr::Var {
                loc: ref _loc,
                ref mut var,
            } if *var == target => {
                *self = *what;
            }
            Expr::Var {
                loc: ref _loc,
                ref mut var,
            } if *var >= target => {
                *var -= 1;
            }
            Expr::Var { .. } => (),
            Expr::App {
                loc: ref _loc,
                ref mut f,
                ref mut arg,
            } => {
                // TODO(jez) Avoid cloning if `f` doesn't actually need `what`
                f.subst(what.clone(), target);
                arg.subst(what, target);
            }
            Expr::Lam {
                loc: ref _loc,
                ref mut body,
            } => {
                what.lift();
                body.subst(what, target + 1)
            }
        }
    }
}

impl fmt::Display for Box<Expr> {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        match **self {
            Expr::Var {
                loc: ref _loc,
                ref var,
            } => write!(out, "#{}", var),
            Expr::App {
                loc: ref _loc,
                ref f,
                ref arg,
            } => write!(out, "({} {})", f, arg),
            Expr::Lam {
                loc: ref _loc,
                ref body,
            } => write!(out, "(\\ -> {})", body),
        }
    }
}
