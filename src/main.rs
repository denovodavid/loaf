#[macro_use]
extern crate lazy_static;

mod ast;
mod interpreter;
mod lexer;
mod parser;

const SOURCE_CODE: &str = r###"
911;
// 1+1;
// (((((1 + 1)))));
// -1 + (2 + 3); // (-(1)) + (2 + 3)
// -1 * 2 + 3; // (-(1 * 2)) + 3
// -1 + 2 * 3; // (-(1)) + (2 * 3)
"###;

fn main() {
    let lexer = lexer::Lexer::new(SOURCE_CODE.to_string());
    for token in lexer {
        println!("{:#?}", token);
    }
    let lexer = lexer::Lexer::new(SOURCE_CODE.to_string());
    let mut parser = parser::Parser::new(lexer);
    let stmt_list_ctx = parser.parse();
    println!("{:#?}", stmt_list_ctx);
    let stmt_list = ast::StmtList::new(&stmt_list_ctx);
    println!("{:?}", stmt_list);
    interpreter::stmt_list(&stmt_list);
}
