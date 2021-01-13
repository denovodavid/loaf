mod ast;
mod lexer;
mod parser;
mod interpreter;

const SOURCE_CODE: &str = r###"
// 911
// 1+1
// (((((1 + 1)))))
-1 + (2 + 3) // (-(1)) + (2 + 3)
// -1 * 2 + 3 // (-(1 * 2)) + 3
// -1 + 2 * 3 // (-(1)) + (2 * 3)
"###;

fn main() {
    let mut lexer = lexer::Lexer::new(SOURCE_CODE.to_string());
    while let Some(token) = lexer.peek() {
        lexer.next();
        println!("{:#?}", token);
    }
    let lexer = lexer::Lexer::new(SOURCE_CODE.to_string());
    let mut parser = parser::Parser::new(lexer);
    let ptree = parser.e().unwrap();
    println!("{:#?}", ptree);
    let ast = ast::Ast::new(&Box::new(ptree));
    println!("{:?}", ast);
    println!("{:?}", interpreter::interpret(&ast));
}
