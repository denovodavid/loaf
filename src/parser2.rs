use crate::lexer;

#[derive(Debug, Clone)]
pub enum ParseTree {
    Token(lexer::Token),
    Block(Vec<ParseTree>),
}

#[derive(Debug, Clone)]
pub struct Parser {
    lexer: lexer::Lexer,
}

impl Parser {
    pub fn new(lexer: lexer::Lexer) -> Self {
        Self { lexer }
    }
    pub fn expr(&mut self) -> Option<ParseTree> {
        self.expr1()
    }
    pub fn expr1(&mut self) -> Option<ParseTree> {
        self.term().and_then(|term| {
            self.expr1_1()
                .or(self.expr1_2())
                .or(self.expr1_3())
                .and_then(|tree| match tree {
                    ParseTree::Block(mut block) if !block.is_empty() => {
                        block.insert(0, term);
                        Some(ParseTree::Block(block))
                    }
                    _ => Some(term),
                })
        })
    }
    fn expr1_1(&mut self) -> Option<ParseTree> {
        self.plus().and_then(|plus| {
            let expr = self.expr().expect("Expected expression");
            Some(ParseTree::Block(vec![plus, expr]))
        })
    }
    fn expr1_2(&mut self) -> Option<ParseTree> {
        self.minus().and_then(|minus| {
            let expr = self.expr().expect("Expected expression");
            Some(ParseTree::Block(vec![minus, expr]))
        })
    }
    fn expr1_3(&mut self) -> Option<ParseTree> {
        Some(ParseTree::Block(vec![]))
    }
    fn term(&mut self) -> Option<ParseTree> {
        self.term1()
            .or(self.term2())
            .or(self.term3())
    }
    fn term1(&mut self) -> Option<ParseTree> {
        self.int().and_then(|int| {
            self.term1_1()
                .or(self.term1_2())
                .or(self.term1_3())
                .and_then(|tree| match tree {
                    ParseTree::Block(mut block) => {
                        block.insert(0, int);
                        Some(ParseTree::Block(block))
                    }
                    _ => Some(int),
                })
        })
    }
    fn term1_1(&mut self) -> Option<ParseTree> {
        self.multiply().and_then(|multiply| {
            let term = self.term().expect("Expected term");
            Some(ParseTree::Block(vec![multiply, term]))
        })
    }
    fn term1_2(&mut self) -> Option<ParseTree> {
        self.divide().and_then(|divide| {
            let term = self.term().expect("Expected term");
            Some(ParseTree::Block(vec![divide, term]))
        })
    }
    fn term1_3(&mut self) -> Option<ParseTree> {
        Some(ParseTree::Block(vec![]))
    }
    fn term2(&mut self) -> Option<ParseTree> {
        self.open_paren().and_then(|open_paren| {
            let expr = self.expr().expect("Expected expression");
            let closed_paren = self.closed_paren().expect("Expected closed parenthesis");
            Some(ParseTree::Block(vec![open_paren, expr, closed_paren]))
        })
    }
    fn term3(&mut self) -> Option<ParseTree> {
        self.minus().and_then(|minus| {
            let term = self.term().expect("Expected term");
            Some(ParseTree::Block(vec![minus, term]))
        })
    }
    fn int(&mut self) -> Option<ParseTree> {
        self.terminal(lexer::TokenClass::IntegerLiteral)
    }
    fn plus(&mut self) -> Option<ParseTree> {
        self.terminal(lexer::TokenClass::PlusPunctuator)
    }
    fn minus(&mut self) -> Option<ParseTree> {
        self.terminal(lexer::TokenClass::MinusPunctuator)
    }
    fn multiply(&mut self) -> Option<ParseTree> {
        self.terminal(lexer::TokenClass::StarPunctuator)
    }
    fn divide(&mut self) -> Option<ParseTree> {
        self.terminal(lexer::TokenClass::SlashPunctuator)
    }
    fn open_paren(&mut self) -> Option<ParseTree> {
        self.terminal(lexer::TokenClass::OpenParenPunctuator)
    }
    fn closed_paren(&mut self) -> Option<ParseTree> {
        self.terminal(lexer::TokenClass::ClosedParenPunctuator)
    }
    fn terminal(&mut self, token_class: lexer::TokenClass) -> Option<ParseTree> {
        self.lexer.peek().and_then(|token| {
            if token.class == token_class {
                self.lexer.next();
                Some(ParseTree::Token(token))
            } else {
                None
            }
        })
    }
}
