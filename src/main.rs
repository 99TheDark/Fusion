pub mod lexer;
pub mod location;
pub mod read;
pub mod tokens;

fn main() {
    let code = read::read_file("code.txt");

    let mut lexer = lexer::Lexer::new(code);
    lexer.lex();
}
