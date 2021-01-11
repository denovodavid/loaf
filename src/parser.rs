use crate::lexer;

#[derive(Debug, Clone)]
pub enum ParseTree2 {
    Token(lexer::Token),
    Block(Vec<ParseTree2>),
}

#[derive(Debug, Clone)]
pub struct ParseTree(Option<lexer::Token>, Vec<ParseTree>);

impl ParseTree {
    pub fn is_empty(&self) -> bool {
        self.0.is_none() && self.1.is_empty()
    }
}

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
                .or_else(|| self.expr1_2())
                .or_else(|| self.expr1_3())
                .and_then(|mut tree| {
                    if tree.is_empty() {
                        tree = term
                    } else {
                        tree.1.insert(0, term);
                    }
                    Some(tree)
                })
        })
    }
    fn expr1_1(&mut self) -> Option<ParseTree> {
        self.plus().and_then(|plus| {
            let expr = self.expr().expect("Expected expression");
            Some(ParseTree(None, vec![plus, expr]))
        })
    }
    fn expr1_2(&mut self) -> Option<ParseTree> {
        self.minus().and_then(|minus| {
            let expr = self.expr().expect("Expected expression");
            Some(ParseTree(None, vec![minus, expr]))
        })
    }
    fn expr1_3(&mut self) -> Option<ParseTree> {
        Some(ParseTree(None, vec![]))
    }
    fn term(&mut self) -> Option<ParseTree> {
        self.term1()
            .or_else(|| self.term2())
            .or_else(|| self.term3())
    }
    fn term1(&mut self) -> Option<ParseTree> {
        self.int().and_then(|int| {
            self.term1_1()
                .or_else(|| self.term1_2())
                .or_else(|| self.term1_3())
                .and_then(|mut tree| {
                    if tree.is_empty() {
                        tree = int
                    } else {
                        tree.1.insert(0, int);
                    }
                    Some(tree)
                })
        })
    }
    fn term1_1(&mut self) -> Option<ParseTree> {
        self.multiply().and_then(|multiply| {
            let term = self.term().expect("Expected term");
            Some(ParseTree(None, vec![multiply, term]))
        })
    }
    fn term1_2(&mut self) -> Option<ParseTree> {
        self.divide().and_then(|divide| {
            let term = self.term().expect("Expected term");
            Some(ParseTree(None, vec![divide, term]))
        })
    }
    fn term1_3(&mut self) -> Option<ParseTree> {
        Some(ParseTree(None, vec![]))
    }
    fn term2(&mut self) -> Option<ParseTree> {
        self.open_paren().and_then(|open_paren| {
            let expr = self.expr().expect("Expected expression");
            let closed_paren = self.closed_paren().expect("Expected closed parenthesis");
            Some(ParseTree(None, vec![open_paren, expr, closed_paren]))
        })
    }
    fn term3(&mut self) -> Option<ParseTree> {
        self.minus().and_then(|minus| {
            let term = self.term().expect("Expected term");
            Some(ParseTree(None, vec![minus, term]))
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
                Some(ParseTree(Some(token), vec![]))
            } else {
                None
            }
        })
    }
}
