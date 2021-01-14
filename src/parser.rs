use crate::lexer::{Lexer, Token, TokenClass};

#[derive(Debug, Clone)]
pub struct StmtListCtx(pub Vec<StmtCtx>);

#[derive(Debug, Clone)]
pub struct StmtCtx(pub Box<ExprCtx>, pub Token);

#[derive(Debug, Clone)]
pub enum ExprCtx {
    Int(Token),
    Block(Token, Box<ExprCtx>, Token),
    Add(Box<ExprCtx>, Token, Box<ExprCtx>),
    Subtract(Box<ExprCtx>, Token, Box<ExprCtx>),
    Negate(Token, Box<ExprCtx>),
    Multiply(Box<ExprCtx>, Token, Box<ExprCtx>),
    Divide(Box<ExprCtx>, Token, Box<ExprCtx>),
}

#[derive(Debug, Clone)]
pub struct Parser {
    lexer: Lexer,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        Self { lexer }
    }
    pub fn sl(&mut self) -> StmtListCtx {
        let mut stmt_list = vec![];
        while let Some(stmt) = self.s() {
            stmt_list.push(stmt);
        }
        StmtListCtx(stmt_list)
    }
    pub fn s(&mut self) -> Option<StmtCtx> {
        self.e().and_then(|expr| {
            let semi = self.semicolon().expect("Expected semicolon");
            Some(StmtCtx(Box::new(expr), semi))
        })
    }
    pub fn e(&mut self) -> Option<ExprCtx> {
        self.e1()
    }
    pub fn e1(&mut self) -> Option<ExprCtx> {
        self.t().and_then(|term| {
            if let Some(plus) = self.plus() {
                let expr = self.e().expect("Expected expression");
                Some(ExprCtx::Add(Box::new(term), plus, Box::new(expr)))
            } else if let Some(minus) = self.minus() {
                let expr = self.e().expect("Expected expression");
                Some(ExprCtx::Subtract(Box::new(term), minus, Box::new(expr)))
            } else {
                Some(term)
            }
        })
    }
    fn t(&mut self) -> Option<ExprCtx> {
        self.t1().or(self.t2()).or(self.t3())
    }
    fn t1(&mut self) -> Option<ExprCtx> {
        self.int().and_then(|int| {
            if let Some(multiply) = self.multiply() {
                let term = self.t().expect("Expected term");
                Some(ExprCtx::Multiply(Box::new(int), multiply, Box::new(term)))
            } else if let Some(divide) = self.divide() {
                let term = self.t().expect("Expected term");
                Some(ExprCtx::Divide(Box::new(int), divide, Box::new(term)))
            } else {
                Some(int)
            }
        })
    }
    fn t2(&mut self) -> Option<ExprCtx> {
        self.l_paren().and_then(|l_paren| {
            let expr = self.e().expect("Expected expression");
            let r_paren = self.r_paren().expect("Expected right parenthesis");
            Some(ExprCtx::Block(l_paren, Box::new(expr), r_paren))
        })
    }
    fn t3(&mut self) -> Option<ExprCtx> {
        self.minus().and_then(|minus| {
            let term = self.t().expect("Expected term");
            Some(ExprCtx::Negate(minus, Box::new(term)))
        })
    }
    fn int(&mut self) -> Option<ExprCtx> {
        self.terminal(TokenClass::IntLit)
            .and_then(|value| Some(ExprCtx::Int(value)))
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
        self.lexer.peek().and_then(|token| {
            if token.class == token_class {
                self.lexer.next();
                Some(token)
            } else {
                None
            }
        })
    }
}
