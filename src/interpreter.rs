use crate::ast::Ast;

pub fn interpret(ast: &Ast) -> i32 {
    match ast {
        Ast::Int(value) => *value,
        Ast::Add(lhs, rhs) => interpret(lhs) + interpret(rhs),
        Ast::Subtract(lhs, rhs) => interpret(lhs) - interpret(rhs),
        Ast::Negate(expr) => -interpret(expr),
        Ast::Multiply(lhs, rhs) => interpret(lhs) * interpret(rhs),
        Ast::Divide(lhs, rhs) => interpret(lhs) / interpret(rhs),
    }
}
