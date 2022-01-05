use crate::token::token::Token;

pub struct Lexer {
    pub input: Vec<char>,
    pub position: usize,
    pub read_position: usize,
    pub ch: Option<char>
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect::<Vec<_>>(),
            position: 0,
            read_position: 0,
            ch: None
        }
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
        self.read_char();

        match self.ch {
            Some('=') => Token::Assign,
            Some(';') => Token::Semicolon,
            Some('(') => Token::Lparen,
            Some(')') => Token::Rparen,
            Some(',') => Token::Comma,
            Some('+') => Token::Plus,
            Some('{') => Token::Lbrace,
            Some('}') => Token::Rbrace,
            _ => Token::Eof,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::lexer::Lexer;
    use crate::token::token::Token;

    #[test]
    fn test_next_token() {
        let input = "=+(){},;";

        let tests = vec![
            Token::Assign,
            Token::Plus,
            Token::Lparen,
            Token::Rparen,
            Token::Lbrace,
            Token::Rbrace,
            Token::Comma,
            Token::Semicolon,
            Token::Eof
        ];

        let mut l = Lexer::new(input);

        for i in 0..tests.len() {
            let tok = l.next_token();

            let tt = &tests[i];

            assert_eq!(&tok, tt);
        }
    }
}