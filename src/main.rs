mod lexer;
// mod parser;
mod parser2;
mod ast;

const SOURCE_CODE: &str = r###"
// 911
// 1+1
// (((((1 + 1)))))
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
    let mut parser = parser2::Parser::new(lexer);
    let ptree = parser.expr().unwrap();
    // println!("{:#?}", ptree);
    let ast = ast::Ast::new(&ptree);
    println!("{:?}", ast);
}

