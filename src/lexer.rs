use regex::Regex;

const TOKEN_SPECS: &[(TokenClass, &str)] = &[
    (TokenClass::Whitespace, r"^\s+"),
    (TokenClass::SingleLineComment, r"^//.*"),
    (TokenClass::MultiLineComment, r"^/\*[\s\S]*?\*/"),
    // (TokenClass::StringLiteral, r#"^"[^"]*""#),
    (TokenClass::IntLit, r"^\d+"),
    (TokenClass::PlusPunc, r"^\+"),
    (TokenClass::MinusPunc, r"^\-"),
    (TokenClass::StarPunc, r"^\*"),
    (TokenClass::SlashPunc, r"^/"),
    (TokenClass::LParenPunc, r"^\("),
    (TokenClass::RParenPunc, r"^\)"),
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

#[derive(Debug, Clone)]
pub struct Lexer {
    res: Vec<(TokenClass, Regex)>,
    source: String,
    cursor: usize,
    peek: Option<Token>,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        let res = TOKEN_SPECS
            .iter()
            .map(|&(tc, r)| (tc, Regex::new(r).unwrap()))
            .collect::<Vec<_>>();
        let mut lexer = Self {
            res,
            source,
            cursor: 0,
            peek: None,
        };
        lexer.peek = lexer.token();
        lexer
    }
    fn token(&mut self) -> Option<Token> {
        while self.cursor < self.source.len() {
            let token = self
                .res
                .iter()
                .find_map(|(tc, re)| {
                    re.find(&self.source[self.cursor..]).and_then(|m| {
                        Some(Token {
                            class: *tc,
                            value: m.as_str().to_string(),
                        })
                    })
                })
                .unwrap_or_else(|| {
                    let c = &self.source[self.cursor..].chars().next().unwrap();
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
    pub fn peek(&self) -> Option<Token> {
        self.peek.clone()
    }
    pub fn next(&mut self) {
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
    IntLit,
    PlusPunc,
    MinusPunc,
    StarPunc,
    SlashPunc,
    LParenPunc,
    RParenPunc,
    // Identifier,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub class: TokenClass,
    pub value: String,
}
