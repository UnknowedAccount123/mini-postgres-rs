#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Select,
    Insert,
    Create,
    Table,
    From,
    Where,
    Identifier(String),
    Number(i64),
    Comma,
    Star,
    LeftParen,
    RightParen,
    Equals,
    Semicolon,
    EOF,
}

pub struct Lexer {
    input: Vec<char>,
    pos: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self { input: input.chars().collect(), pos: 0 }
    }

    fn peek(&self) -> Option<char> {
        self.input.get(self.pos).copied()
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.peek();
        self.pos += 1;
        ch
    }

    pub fn next_token(&mut self) -> Token {
        while let Some(ch) = self.peek() {
            if ch.is_whitespace() {
                self.advance();
                continue;
            }

            if ch.is_ascii_alphabetic() {
                let mut ident = String::new();
                while let Some(c) = self.peek() {
                    if c.is_ascii_alphanumeric() {
                        ident.push(c);
                        self.advance();
                    } else {
                        break;
                    }
                }

                return match ident.to_lowercase().as_str() {
                    "select" => Token::Select,
                    "insert" => Token::Insert,
                    "create" => Token::Create,
                    "table" => Token::Table,
                    "from" => Token::From,
                    "where" => Token::Where,
                    _ => Token::Identifier(ident),
                };
            }

            match ch {
                ',' => { self.advance(); return Token::Comma; }
                '*' => { self.advance(); return Token::Star; }
                '(' => { self.advance(); return Token::LeftParen; }
                ')' => { self.advance(); return Token::RightParen; }
                '=' => { self.advance(); return Token::Equals; }
                ';' => { self.advance(); return Token::Semicolon; }
                _ => { self.advance(); }
            }
        }

        Token::EOF
    }
}
