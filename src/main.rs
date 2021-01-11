mod lexer;
mod parser;

const SOURCE_CODE: &str = r###"
-1 + (2 + 3) // (-(1)) + (2 + 3)
// -1 * 2 + 3 // (-(1 * 2)) + 3
// -1 + 2 * 3 // (-(1)) + (2 * 3)
"###;

fn main() {
    // let mut lexer = lexer::Lexer::new(SOURCE_CODE.to_string());
    // while let Some(token) = lexer.peek() {
    //     lexer.next();
    //     println!("{:#?}", token);
    // }
    let lexer = lexer::Lexer::new(SOURCE_CODE.to_string());
    let mut parser = parser::Parser::new(lexer);
    println!("{:#?}", parser.expr());
}

