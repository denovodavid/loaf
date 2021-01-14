use crate::ast::{Expr, Stmt, StmtList};

pub fn stmt_list(sl: &StmtList) {
    sl.0.iter().for_each(stmt)
}

pub fn stmt(s: &Stmt) {
    println!("{}", expr(&s.0))
}

pub fn expr(e: &Expr) -> i32 {
    match e {
        Expr::Int(value) => *value,
        Expr::Add(lhs, rhs) => expr(lhs) + expr(rhs),
        Expr::Subtract(lhs, rhs) => expr(lhs) - expr(rhs),
        Expr::Negate(rhs) => -expr(rhs),
        Expr::Multiply(lhs, rhs) => expr(lhs) * expr(rhs),
        Expr::Divide(lhs, rhs) => expr(lhs) / expr(rhs),
    }
}
