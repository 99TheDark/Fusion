pub mod ast;
pub mod lexer;
pub mod location;
pub mod parser;
pub mod read;
pub mod tokens;

fn main() {
    let code = read::read_file("code.txt");

    let mut lexer = lexer::Lexer::new(code);
    lexer.lex();

    let tokens = lexer.filter();

    let mut parser = parser::Parser::new(&tokens);
    parser.parse();

    println!("{:#?}", tokens);
}
