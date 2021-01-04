use regex::Regex;

const SOURCE_CODE: &str = r###"
// hello, I'm a ❤️ 1 line "comment"
34 "54" 234 5

{trueH}

☝️

🧑..🧝

👋👍👍🏻

👍🏿👍

❤

咊加®©

❤️j

=
+ - * /

/*
and i'm many comments, 45 "567" anything goes here!
*/
{} let {}let
76 } jorseman y7er_gf34 🚀7h 7gd _dhfg ⚙️
"###;

#[derive(Debug, Copy, Clone)]
pub enum TokenName {
    Whitespace,
    SingleLineComment,
    MultiLineComment,
    StringLiteral,
    IntegerLiteral,
    Punctuator,
    Identifier,
}

#[derive(Debug, Clone)]
pub struct Token {
    name: TokenName,
    value: String,
}

fn main() {
    let specs = [
        (TokenName::Whitespace, r"^\s+"),
        (TokenName::SingleLineComment, r"^//.*"),
        (TokenName::MultiLineComment, r"^/\*[\s\S]*?\*/"),
        (TokenName::StringLiteral, r#"^"[^"]*""#),
        (TokenName::IntegerLiteral, r"^\d+"),
        (TokenName::Punctuator, r"^[\{\}\.\+\-=/\*]"),
        (
            TokenName::Identifier,
            // [\p{Emoji}\p{Emoji_Component}&&[^#*]]
            r"^[\p{L}_[\p{Emoji}\p{Emoji_Component}&&[^#*]]][\p{L}\p{N}_[\p{Emoji}\p{Emoji_Component}&&[^#*]]]*",
        ),
    ];
    let res = specs
        .iter()
        .map(|&(tn, r)| (tn, Regex::new(r).unwrap()))
        .collect::<Vec<_>>();
    let mut cursor = 0usize;
    while cursor < SOURCE_CODE.len() {
        let token = res
            .iter()
            .find_map(|(tn, re)| {
                re.find(&SOURCE_CODE[cursor..]).and_then(|m| {
                    Some(Token {
                        name: *tn,
                        value: m.as_str().to_string(),
                    })
                })
            })
            .unwrap_or_else(|| {
                let c = &SOURCE_CODE[cursor..]
                    .chars()
                    .next()
                    .unwrap();
                panic!("Invalid token, \"{}\" ({}), at position {}.", c, c.escape_unicode(), cursor)
            });
        cursor = cursor + token.value.len();
        if !matches!(
            token.name,
            TokenName::Whitespace | TokenName::SingleLineComment | TokenName::MultiLineComment
        ) {
            println!("{:?}", token);
        }
    }
}
