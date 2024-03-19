pub mod ast;
pub mod checker;
pub mod error;
pub mod lexer;
pub mod location;
pub mod parser;
pub mod read;
pub mod scope;
pub mod tokens;
pub mod types;

use std::rc::Rc;

use checker::Checker;
use lexer::{source_lines, Lexer};
use parser::Parser;

fn main() {
    let code = Rc::new(read::read_file("code.txt"));
    let lines = source_lines(Rc::clone(&code));

    let mut lexer = Lexer::new(Rc::clone(&code));
    lexer.lex();

    let tokens = lexer.filter();

    let mut parser = Parser::new(Rc::clone(&lines), &tokens);
    parser.parse();

    parser.prog.print();

    let mut checker = Checker::new(Rc::clone(&lines), parser.prog);
    checker.check();

    // checker.prog.print()
}
