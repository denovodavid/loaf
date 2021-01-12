use crate::parser::ExprCtx;
use crate::lexer::Token;

#[derive(Debug, Clone)]
pub enum Ast {
    Int(i32),
    Add(Box<Ast>, Box<Ast>),
    Subtract(Box<Ast>, Box<Ast>),
    Negate(Box<Ast>),
    Multiply(Box<Ast>, Box<Ast>),
    Divide(Box<Ast>, Box<Ast>),
}

impl Ast {
    pub fn new(ptree: &Box<ExprCtx>) -> Self {
        match ptree.as_ref() {
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
