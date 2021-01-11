use crate::lexer::{Token, TokenClass};
use crate::parser2::*;

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
    pub fn new(ptree: &ParseTree) -> Self {
        match ptree {
            ParseTree::Token(token) => match token.class {
                TokenClass::IntegerLiteral => Ast::Int(token.value.parse().unwrap()),
                _ => panic!(),
            },
            ParseTree::Block(block) => Self::block(&block)
                .or(Self::int(&block))
                .or(Self::add(&block))
                .or(Self::subtract(&block))
                .or(Self::negate(&block))
                .or(Self::multiply(&block))
                .or(Self::divide(&block))
                .unwrap(),
        }
    }
    fn block(block: &[ParseTree]) -> Option<Ast> {
        match block {
            [ParseTree::Token(Token {
                class: TokenClass::OpenParenPunctuator,
                ..
            }), ptree, ParseTree::Token(Token {
                class: TokenClass::ClosedParenPunctuator,
                ..
            })] => Some(Self::new(ptree)),
            _ => None,
        }
    }
    fn int(block: &[ParseTree]) -> Option<Ast> {
        match block {
            [ParseTree::Token(Token {
                class: TokenClass::IntegerLiteral,
                value,
            })] => Some(Ast::Int(value.parse().unwrap())),
            _ => None,
        }
    }
    fn add(block: &[ParseTree]) -> Option<Ast> {
        match block {
            [lhs, ParseTree::Token(Token {
                class: TokenClass::PlusPunctuator,
                ..
            }), rhs] => Some(Ast::Add(
                Box::new(Self::new(lhs)),
                Box::new(Self::new(rhs)),
            )),
            _ => None,
        }
    }
    fn subtract(block: &[ParseTree]) -> Option<Ast> {
        match block {
            [lhs, ParseTree::Token(Token {
                class: TokenClass::MinusPunctuator,
                ..
            }), rhs] => Some(Ast::Subtract(
                Box::new(Self::new(lhs)),
                Box::new(Self::new(rhs)),
            )),
            _ => None,
        }
    }
    fn negate(block: &[ParseTree]) -> Option<Ast> {
        match block {
            [ParseTree::Token(Token {
                class: TokenClass::MinusPunctuator,
                ..
            }), rhs] => Some(Ast::Negate(Box::new(Self::new(rhs)))),
            _ => None,
        }
    }
    fn multiply(block: &[ParseTree]) -> Option<Ast> {
        match block {
            [lhs, ParseTree::Token(Token {
                class: TokenClass::StarPunctuator,
                ..
            }), rhs] => Some(Ast::Multiply(
                Box::new(Self::new(lhs)),
                Box::new(Self::new(rhs)),
            )),
            _ => None,
        }
    }
    fn divide(block: &[ParseTree]) -> Option<Ast> {
        match block {
            [lhs, ParseTree::Token(Token {
                class: TokenClass::SlashPunctuator,
                ..
            }), rhs] => Some(Ast::Divide(
                Box::new(Self::new(lhs)),
                Box::new(Self::new(rhs)),
            )),
            _ => None,
        }
    }
}
