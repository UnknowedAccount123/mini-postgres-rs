use crate::lexer::{Lexer, Token};

#[derive(Debug)]
pub enum AST {
    Select { table: String },
    Insert { table: String },
    CreateTable { name: String },
}

pub struct Parser {
    lexer: Lexer,
    current: Token,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let current = lexer.next_token();
        Self { lexer, current }
    }

    fn eat(&mut self) {
        self.current = self.lexer.next_token();
    }

    pub fn parse(&mut self) -> AST {
        match &self.current {
            Token::Select => AST::Select { table: "users".into() },
            Token::Insert => AST::Insert { table: "users".into() },
            Token::Create => AST::CreateTable { name: "users".into() },
            _ => AST::Select { table: "unknown".into() },
        }
    }
}
