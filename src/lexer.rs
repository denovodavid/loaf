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
    (TokenClass::SemiColonPunc, r"^;"),
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

lazy_static! {
    static ref RES: Vec<(TokenClass, Regex)> = TOKEN_SPECS
        .iter()
        .map(|&(tc, r)| (tc, Regex::new(r).unwrap()))
        .collect();
}

#[derive(Debug, Clone)]
pub struct Lexer {
    source: String,
    cursor: usize,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Self { source, cursor: 0 }
    }
    fn next_token(&mut self) -> Option<Token> {
        RES.iter().find_map(|(tc, re)| {
            re.find(&self.source[self.cursor..]).and_then(|m| {
                Some(Token {
                    class: *tc,
                    value: m.as_str().to_string(),
                })
            })
        })
    }
    fn lex(&mut self) -> Option<Token> {
        if self.cursor < self.source.len() {
            let token = match self.next_token() {
                Some(token) => token,
                None => match &self.source[self.cursor..].chars().next() {
                    Some(c) => panic!(
                        "Invalid token, \"{}\" ({}), at position {}.",
                        c,
                        c.escape_unicode(),
                        self.cursor
                    ),
                    None => panic!("Unknown invalid token at position {}.", self.cursor),
                },
            };
            self.cursor = self.cursor + token.value.len();
            match token.class {
                TokenClass::Whitespace
                | TokenClass::SingleLineComment
                | TokenClass::MultiLineComment => self.lex(),
                _ => Some(token),
            }
        } else {
            None
        }
    }
}

impl Iterator for Lexer {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        self.lex()
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
    SemiColonPunc,
    // Identifier,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub class: TokenClass,
    pub value: String,
}
