use crate::lexer::Token;
use crate::parser::{ExprCtx, StmtCtx, StmtListCtx};

#[derive(Debug, Clone)]
pub struct StmtList(pub Vec<Stmt>);

impl StmtList {
    pub fn new(stmt_list_ctx: &StmtListCtx) -> Self {
        Self(stmt_list_ctx.0.iter().map(Stmt::new).collect())
    }
}

#[derive(Debug, Clone)]
pub struct Stmt(pub Expr);

impl Stmt {
    pub fn new(stmt_ctx: &StmtCtx) -> Self {
        Self(Expr::new(&stmt_ctx.0))
    }
}

#[derive(Debug, Clone)]
pub enum Expr {
    Int(i32),
    Add(Box<Expr>, Box<Expr>),
    Subtract(Box<Expr>, Box<Expr>),
    Negate(Box<Expr>),
    Multiply(Box<Expr>, Box<Expr>),
    Divide(Box<Expr>, Box<Expr>),
}

impl Expr {
    pub fn new(expr_ctx: &Box<ExprCtx>) -> Self {
        match expr_ctx.as_ref() {
            ExprCtx::Int(token) => Self::int(token),
            ExprCtx::Block(_, expr, _) => Self::new(expr),
            ExprCtx::Add(lhs, _, rhs) => Self::add(lhs, rhs),
            ExprCtx::Subtract(lhs, _, rhs) => Self::subtract(lhs, rhs),
            ExprCtx::Negate(_, expr) => Self::negate(expr),
            ExprCtx::Multiply(lhs, _, rhs) => Self::multiply(lhs, rhs),
            ExprCtx::Divide(lhs, _, rhs) => Self::divide(lhs, rhs),
        }
    }
    fn int(token: &Token) -> Self {
        Self::Int(token.value.parse().unwrap())
    }
    fn add(lhs: &Box<ExprCtx>, rhs: &Box<ExprCtx>) -> Self {
        Self::Add(Box::new(Self::new(lhs)), Box::new(Self::new(rhs)))
    }
    fn subtract(lhs: &Box<ExprCtx>, rhs: &Box<ExprCtx>) -> Self {
        Self::Subtract(Box::new(Self::new(lhs)), Box::new(Self::new(rhs)))
    }
    fn negate(expr: &Box<ExprCtx>) -> Self {
        Self::Negate(Box::new(Self::new(expr)))
    }
    fn multiply(lhs: &Box<ExprCtx>, rhs: &Box<ExprCtx>) -> Self {
        Self::Multiply(Box::new(Self::new(lhs)), Box::new(Self::new(rhs)))
    }
    fn divide(lhs: &Box<ExprCtx>, rhs: &Box<ExprCtx>) -> Self {
        Self::Divide(Box::new(Self::new(lhs)), Box::new(Self::new(rhs)))
    }
}
