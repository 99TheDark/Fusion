pub mod ast;
pub mod error;
pub mod lexer;
pub mod location;
pub mod parser;
pub mod read;
pub mod tokens;

use std::rc::Rc;

use lexer::Lexer;
use parser::Parser;

fn main() {
    let code = Rc::new(read::read_file("code.txt"));

    let mut lexer = Lexer::new(Rc::clone(&code));
    lexer.lex();

    let tokens = lexer.filter();

    let mut parser = Parser::new(Rc::clone(&code), &tokens);
    parser.parse();
}
