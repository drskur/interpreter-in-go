use crate::token::token::{lookup_ident, Token};

pub struct Lexer {
    pub input: Vec<char>,
    pub position: usize,
    pub read_position: usize,
    pub ch: Option<char>
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut lexer = Lexer {
            input: input.chars().collect::<Vec<_>>(),
            position: 0,
            read_position: 0,
            ch: None
        };
        lexer.read_char();

        lexer
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = None
        } else {
            self.ch = Some(self.input[self.read_position])
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {

        self.skip_whitespace();

        let token = match self.ch {
            Some('=') => Token::Assign,
            Some(';') => Token::Semicolon,
            Some('(') => Token::Lparen,
            Some(')') => Token::Rparen,
            Some(',') => Token::Comma,
            Some('+') => Token::Plus,
            Some('{') => Token::Lbrace,
            Some('}') => Token::Rbrace,
            Some(ch) => {
                if is_letter(ch) {
                    let ident = self.read_identifier();
                    lookup_ident(ident)
                } else {
                    Token::Illegal
                }
            }
            None => Token::Eof,
        };

        self.read_char();

        token
    }

    pub fn read_identifier(&mut self) -> String {
        let position = self.position;

        while let Some(ch) = self.ch {
            if is_letter(ch) {
                self.read_char()
            } else {
                break;
            }
        }

        self.input[position..self.position].iter().collect::<String>()
    }

    pub fn skip_whitespace(&mut self) {
        while let Some(ch) = self.ch {
            if ch == ' ' || ch == '\t' || ch == '\n' || ch == '\r' {
                self.read_char();
            } else {
                break
            }
        }
    }
}

fn is_letter(ch: char) -> bool {
    ch >= 'a' && ch <= 'z' || ch >= 'A' && ch <= 'Z' || ch == '_'
}

#[cfg(test)]
mod tests {
    use crate::lexer::lexer::Lexer;
    use crate::token::token::Token;

    #[test]
    fn test_next_token() {
        let input = r#"let five = 5;
let ten = 10;

let add = fn(x, y) {
    x + 6;
};

let result = add(five, ten);
"#;

        let tests = vec![
            Token::Let,
            Token::Ident("five".to_string()),
            Token::Assign,
            Token::Int(5)
        ];

        let mut l = Lexer::new(input);

        for i in 0..tests.len() {
            let tok = l.next_token();

            let tt = &tests[i];

            assert_eq!(&tok, tt);
        }
    }
}