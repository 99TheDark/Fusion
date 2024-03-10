pub mod ast;
pub mod lexer;
pub mod location;
pub mod parser;
pub mod read;
pub mod tokens;

use lexer::Lexer;
use parser::Parser;

fn main() {
    let code = read::read_file("code.txt");

    let mut lexer = Lexer::new(code);
    lexer.lex();

    let tokens = lexer.filter();

    let mut parser = Parser::new(&tokens);
    parser.parse();
}
