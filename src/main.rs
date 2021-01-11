use core::panic;

use regex::Regex;

const SOURCE_CODE: &str = r###"
-1 + (2 + 3) // (-(1)) + (2 + 3)
// -1 * 2 + 3 // (-(1 * 2)) + 3
// -1 + 2 * 3 // (-(1)) + (2 * 3)
"###;

fn main() {
    // let mut tokenizer = Tokenizer::new();
    // while let Some(token) = tokenizer.peek() {
    //     tokenizer.next();
    //     println!("{:#?}", token);
    // }
    let tokenizer = Tokenizer::new();
    let mut parser = Parser::new(tokenizer);
    println!("{:#?}", parser.expr());
}

const TOKEN_SPECS: &[(TokenClass, &str)] = &[
    (TokenClass::Whitespace, r"^\s+"),
    (TokenClass::SingleLineComment, r"^//.*"),
    (TokenClass::MultiLineComment, r"^/\*[\s\S]*?\*/"),
    // (TokenClass::StringLiteral, r#"^"[^"]*""#),
    (TokenClass::IntegerLiteral, r"^\d+"),
    (TokenClass::PlusPunctuator, r"^\+"),
    (TokenClass::MinusPunctuator, r"^\-"),
    (TokenClass::StarPunctuator, r"^\*"),
    (TokenClass::SlashPunctuator, r"^/"),
    (TokenClass::OpenParenPunctuator, r"^\("),
    (TokenClass::ClosedParenPunctuator, r"^\)"),
    // (TokenClass::SemiColonPunctuator, r"^;"),
    // (
    //     TokenClass::Punctuator(Punctuator::Unknown),
    //     r"^[\{\}()\.\+\-=/\*;]",
    // ),
    // (
    //     TokenClass::Identifier,
    //     // [\p{Emoji}\p{Emoji_Component}&&[^#\*]]
    //     r"^[_\p{L}[\p{Emoji}\p{Emoji_Component}&&[^#\*]]][_\p{L}[\p{Emoji}\p{Emoji_Component}&&[^#\*]\p{N}]]*",
    // ),
];

pub struct Tokenizer {
    res: Vec<(TokenClass, Regex)>,
    cursor: usize,
    peek: Option<Token>,
}

impl Tokenizer {
    fn new() -> Self {
        let res = TOKEN_SPECS
            .iter()
            .map(|&(tc, r)| (tc, Regex::new(r).unwrap()))
            .collect::<Vec<_>>();
        let mut tokenizer = Self {
            res,
            cursor: 0,
            peek: None,
        };
        tokenizer.peek = tokenizer.token();
        tokenizer
    }
    fn token(&mut self) -> Option<Token> {
        while self.cursor < SOURCE_CODE.len() {
            let token = self
                .res
                .iter()
                .find_map(|(tc, re)| {
                    re.find(&SOURCE_CODE[self.cursor..]).and_then(|m| {
                        Some(Token {
                            class: *tc,
                            value: m.as_str().to_string(),
                        })
                    })
                })
                .unwrap_or_else(|| {
                    let c = &SOURCE_CODE[self.cursor..].chars().next().unwrap();
                    panic!(
                        "Invalid token, \"{}\" ({}), at position {}.",
                        c,
                        c.escape_unicode(),
                        self.cursor
                    )
                });
            return match token.class {
                TokenClass::Whitespace
                | TokenClass::SingleLineComment
                | TokenClass::MultiLineComment => {
                    self.eat(&token);
                    self.token()
                }
                _ => Some(token),
            };
        }
        None
    }
    fn peek(&self) -> Option<Token> {
        self.peek.clone()
    }
    fn next(&mut self) {
        if let Some(token) = self.peek() {
            self.eat(&token);
            self.peek = self.token();
        }
    }
    fn eat(&mut self, token: &Token) {
        self.cursor = self.cursor + token.value.len();
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TokenClass {
    Whitespace,
    SingleLineComment,
    MultiLineComment,
    // StringLiteral,
    IntegerLiteral,
    PlusPunctuator,
    MinusPunctuator,
    StarPunctuator,
    SlashPunctuator,
    OpenParenPunctuator,
    ClosedParenPunctuator,
    // Identifier,
}

#[derive(Debug, Clone)]
pub struct Token {
    class: TokenClass,
    value: String,
}

#[derive(Debug, Clone)]
pub enum ParseTree2 {
    Token(Token),
    Block(Vec<ParseTree2>),
}

#[derive(Debug, Clone)]
pub struct ParseTree(Option<Token>, Vec<ParseTree>);

impl ParseTree {
    pub fn is_empty(&self) -> bool {
        self.0.is_none() && self.1.is_empty()
    }
}

pub struct Parser {
    tokenizer: Tokenizer,
}

impl Parser {
    pub fn new(tokenizer: Tokenizer) -> Self {
        Self { tokenizer }
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
        self.terminal(TokenClass::IntegerLiteral)
    }
    fn plus(&mut self) -> Option<ParseTree> {
        self.terminal(TokenClass::PlusPunctuator)
    }
    fn minus(&mut self) -> Option<ParseTree> {
        self.terminal(TokenClass::MinusPunctuator)
    }
    fn multiply(&mut self) -> Option<ParseTree> {
        self.terminal(TokenClass::StarPunctuator)
    }
    fn divide(&mut self) -> Option<ParseTree> {
        self.terminal(TokenClass::SlashPunctuator)
    }
    fn open_paren(&mut self) -> Option<ParseTree> {
        self.terminal(TokenClass::OpenParenPunctuator)
    }
    fn closed_paren(&mut self) -> Option<ParseTree> {
        self.terminal(TokenClass::ClosedParenPunctuator)
    }
    fn terminal(&mut self, token_class: TokenClass) -> Option<ParseTree> {
        self.tokenizer.peek().and_then(|token| {
            if token.class == token_class {
                self.tokenizer.next();
                Some(ParseTree(Some(token), vec![]))
            } else {
                None
            }
        })
    }
}
