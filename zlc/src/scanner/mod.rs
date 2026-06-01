use anyhow::{Result, bail};
use std::mem;
use token::{Literal, LiteralKind, Span, Token, TokenKind};

use crate::scanner::keywords::KEYWORDS;

mod keywords;
mod token;

pub struct Scanner<'a> {
    source: &'a str,
    tokens: Vec<Token>,

    start: usize, // Start of current token
    current: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            tokens: vec![],
            start: 0,
            current: 0,
        }
    }

    pub fn scan(&mut self) -> Result<Vec<Token>> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.add_token(TokenKind::EndOfFile);
        Ok(mem::take(&mut self.tokens))
    }

    fn scan_token(&mut self) -> Result<()> {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenKind::OpenParen),
            ')' => self.add_token(TokenKind::CloseParen),
            '{' => self.add_token(TokenKind::OpenCurly),
            '}' => self.add_token(TokenKind::CloseCurly),
            ';' => self.add_token(TokenKind::Semicolon),
            ' ' => {}
            '\n' => {}
            '\r' => {}
            _ => {
                if c.is_numeric() {
                    self.scan_number();
                } else if c.is_alphabetic() {
                    self.scan_identifier();
                } else {
                    bail!("Unexpected character: {}", c);
                }
            }
        }

        Ok(())
    }

    fn scan_number(&mut self) -> Result<()> {
        while self.peek().is_numeric() {
            self.advance();
        }
        let lexeme = &self.source[self.start..self.current];

        match lexeme.parse::<i32>() {
            Ok(_) => self.add_token(TokenKind::Literal(Literal {
                kind: LiteralKind::Integer,
            })),
            Err(_) => bail!("Invalid integer literal: {}", lexeme),
        }

        Ok(())
    }

    fn scan_identifier(&mut self) {
        while self.peek().is_alphanumeric() {
            self.advance();
        }

        let substring = &self.source[self.start..self.current];

        let kind = KEYWORDS
            .get(substring)
            .copied()
            .unwrap_or(TokenKind::Identifier);

        self.add_token(kind);
    }

    fn add_token(&mut self, kind: TokenKind) {
        self.tokens.push(Token {
            kind,
            span: Span {
                start: self.start,
                end: self.current,
            },
        });
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        let c = self.source[self.current..].chars().next().unwrap();
        self.current += c.len_utf8();
        c
    }

    /// Look at the next character without consuming it
    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        return self.source[self.current..].chars().next().unwrap();
    }
}
