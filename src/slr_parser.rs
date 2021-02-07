use crate::lexer::{Token, TokenClass};
use std::iter::Peekable;

#[derive(Debug, Clone)]
pub struct S(Box<E>);

#[derive(Debug, Clone)]
pub enum E {
    E0(Box<E>, Box<T>),
    E1(Box<T>),
}

#[derive(Debug, Clone)]
pub enum T {
    T0(Box<T>, Box<F>),
    T1(Box<F>),
}

#[derive(Debug, Clone)]
pub struct F(u32);

#[derive(Debug, Clone)]
pub enum Production {
    S(S),
    E(E),
    T(T),
    F(F),
    Token(Token),
}

#[derive(Debug, Clone)]
pub struct SlrParser<I>
where
    I: Iterator<Item = Token>,
{
    lexer: Peekable<I>,
    stack: Vec<Production>,
    state: u8,
    cursor: usize,
}

impl<I> SlrParser<I>
where
    I: Iterator<Item = Token>,
{
    pub fn new(lexer: I) -> Self {
        Self {
            lexer: lexer.peekable(),
            stack: vec![],
            state: 0,
            cursor: 0,
        }
    }
    pub fn parse(mut self) -> S {
        loop {
            match self.state {
                0 => {
                    if let Some(int) = self.int() {
                        self.shift(int, 4);
                    } else {
                        let state = match self.stack.get(self.cursor) {
                            Some(Production::E(_)) => 1,
                            Some(Production::T(_)) => 2,
                            Some(Production::F(_)) => 3,
                            _ => panic!("Unexpected production"),
                        };
                        self.goto(state);
                    }
                }
                1 => match self.lexer.peek() {
                    Some(Token {
                        class: TokenClass::PlusPunc,
                        ..
                    }) => {
                        let token = self.lexer.next().unwrap();
                        self.shift(token, 5);
                    }
                    _ => match self.stack.get(self.cursor) {
                        Some(Production::Token(Token {
                            class: TokenClass::PlusPunc,
                            ..
                        })) => self.goto(5),
                        _ => {
                            // reduce
                            let e = match self.stack.pop() {
                                Some(Production::E(e)) => e,
                                _ => panic!("Expected E production"),
                            };
                            let s = S(Box::new(e));
                            return s;
                            // self.stack.push(Production::S(s));
                            // self.state = 0;
                            // self.cursor = 0;
                            // break;
                        }
                    },
                },
                2 => match self.lexer.peek() {
                    Some(Token {
                        class: TokenClass::StarPunc,
                        ..
                    }) => {
                        let token = self.lexer.next().unwrap();
                        self.shift(token, 6);
                    }
                    _ => match self.stack.get(self.cursor) {
                        Some(Production::Token(Token {
                            class: TokenClass::StarPunc,
                            ..
                        })) => self.goto(6),
                        _ => {
                            // reduce
                            let t = match self.stack.pop() {
                                Some(Production::T(t)) => t,
                                _ => panic!("Expected T production"),
                            };
                            let e = E::E1(Box::new(t));
                            self.stack.push(Production::E(e));
                            self.state = 0;
                            self.cursor = 0;
                        }
                    },
                },
                3 => match self.stack.pop() {
                    Some(Production::F(f)) => {
                        // reduce
                        let t = T::T1(Box::new(f));
                        self.stack.push(Production::T(t));
                        self.state = 0;
                        self.cursor = 0;
                    }
                    _ => panic!("Expected F production"),
                },
                4 => match self.stack.pop() {
                    Some(Production::Token(Token {
                        class: TokenClass::IntLit,
                        value,
                    })) => {
                        // reduce
                        let f = F(value.parse().unwrap());
                        self.stack.push(Production::F(f));
                        self.state = 0;
                        self.cursor = 0;
                    }
                    _ => panic!("Expected int token production"),
                },
                5 => match self.lexer.peek() {
                    Some(Token {
                        class: TokenClass::IntLit,
                        ..
                    }) => {
                        let token = self.lexer.next().unwrap();
                        self.shift(token, 4);
                    }
                    _ => match self.stack.get(self.cursor) {
                        Some(Production::T(_)) => self.goto(7),
                        Some(Production::F(_)) => self.goto(3),
                        _ => panic!("Unexpected production"),
                    },
                },
                6 => match self.lexer.peek() {
                    Some(Token {
                        class: TokenClass::IntLit,
                        ..
                    }) => {
                        let token = self.lexer.next().unwrap();
                        self.shift(token, 4);
                    }
                    _ => match self.stack.get(self.cursor) {
                        Some(Production::F(_)) => self.goto(8),
                        _ => panic!("Expected F production"),
                    },
                },
                7 => match self.lexer.peek() {
                    Some(Token {
                        class: TokenClass::StarPunc,
                        ..
                    }) => {
                        let token = self.lexer.next().unwrap();
                        self.shift(token, 6);
                    }
                    _ => match self.stack.get(self.cursor) {
                        Some(Production::Token(Token {
                            class: TokenClass::StarPunc,
                            ..
                        })) => self.goto(6),
                        _ => {
                            // reduce
                            let t = match self.stack.pop() {
                                Some(Production::T(t)) => t,
                                _ => panic!("Expected T production"),
                            };
                            self.stack.pop();
                            let e = match self.stack.pop() {
                                Some(Production::E(e)) => e,
                                _ => panic!("Expected E production"),
                            };
                            let e_prod = Production::E(E::E0(Box::new(e), Box::new(t)));
                            self.stack.push(e_prod);
                            self.state = 0;
                            self.cursor = 0;
                        }
                    },
                },
                8 => {
                    // reduce
                    let f = match self.stack.pop() {
                        Some(Production::F(f)) => f,
                        _ => panic!("Expected F production"),
                    };
                    self.stack.pop();
                    let t = match self.stack.pop() {
                        Some(Production::T(t)) => t,
                        _ => panic!("Expected T production"),
                    };
                    let t_prod = Production::T(T::T0(Box::new(t), Box::new(f)));
                    self.stack.push(t_prod);
                    self.state = 0;
                    self.cursor = 0;
                }
                _ => panic!("Invalid state {}", self.state),
            };
        }
    }
    fn shift(&mut self, token: Token, state: u8) {
        self.stack.push(Production::Token(token));
        self.state = state;
    }
    fn goto(&mut self, state: u8) {
        self.state = state;
        self.cursor += 1;
    }
    fn reduce(&mut self) {}
    fn int(&mut self) -> Option<Token> {
        self.terminal(TokenClass::IntLit)
    }
    fn plus(&mut self) -> Option<Token> {
        self.terminal(TokenClass::PlusPunc)
    }
    fn minus(&mut self) -> Option<Token> {
        self.terminal(TokenClass::MinusPunc)
    }
    fn multiply(&mut self) -> Option<Token> {
        self.terminal(TokenClass::StarPunc)
    }
    fn divide(&mut self) -> Option<Token> {
        self.terminal(TokenClass::SlashPunc)
    }
    fn l_paren(&mut self) -> Option<Token> {
        self.terminal(TokenClass::LParenPunc)
    }
    fn r_paren(&mut self) -> Option<Token> {
        self.terminal(TokenClass::RParenPunc)
    }
    fn semicolon(&mut self) -> Option<Token> {
        self.terminal(TokenClass::SemiColonPunc)
    }
    fn terminal(&mut self, token_class: TokenClass) -> Option<Token> {
        // TODO: stable in 1.50
        // self.lexer.next_if(|token| token.class == token_class)
        match self.lexer.peek() {
            Some(token) if token.class == token_class => self.lexer.next(),
            _ => None,
        }
    }
}
