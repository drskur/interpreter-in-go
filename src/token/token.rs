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

pub fn lookup_ident(ident: String) -> Token {
    match ident.as_str() {
        "let" => Token::Let,
        "fn" => Token::Function,
        _ => Token::Ident(ident)
    }
}