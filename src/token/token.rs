#[derive(PartialEq, Debug)]
pub enum Token {
    Illegal,
    Eof,
    Ident(String),
    Int(i32),
    Assign,
    Plus,
    Comma,
    Semicolon,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    Function,
    Let
}