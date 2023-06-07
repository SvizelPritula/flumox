use std::{
    fmt::{self, Display, Formatter},
    iter::Peekable,
    str::CharIndices,
};

use crate::EvalError;

#[derive(Debug, Clone, Copy)]
pub enum Token<'a> {
    Word(&'a str),
    Number(u64),
    And,
    Or,
    Plus,
    Dash,
    Colon,
    Dot,
    LeftParen,
    RightParen,
}

pub struct Tokens<'a> {
    string: &'a str,
    chars: Peekable<CharIndices<'a>>,
}

impl<'a> Iterator for Tokens<'a> {
    type Item = Result<Token<'a>, EvalError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let Some((idx, c)) = self.chars.next() else {
                break None;
            };

            break Some(match c {
                c if c.is_whitespace() => continue,
                c @ '0'..='9' => self.parse_number(c),
                c if c.is_alphabetic() => Ok(self.parse_word(idx)),
                '_' => Ok(self.parse_word(idx)),
                '|' => Ok(Token::Or),
                '&' => Ok(Token::And),
                '+' => Ok(Token::Plus),
                '-' => Ok(Token::Dash),
                ':' => Ok(Token::Colon),
                '.' => Ok(Token::Dot),
                '(' => Ok(Token::LeftParen),
                ')' => Ok(Token::RightParen),
                other => Err(EvalError::UnknownChar { char: other }),
            });
        }
    }
}

impl<'a> Tokens<'a> {
    fn parse_number(&mut self, first: char) -> Result<Token<'a>, EvalError> {
        let mut result = first as u64 - '0' as u64;

        loop {
            match self.chars.peek().map(|(_i, c)| c).copied() {
                Some(c @ '0'..='9') => {
                    self.chars.next();

                    let value = c as u64 - '0' as u64;

                    result = result
                        .checked_mul(10)
                        .and_then(|r| r.checked_add(value))
                        .ok_or(EvalError::LiteralOutOfRange)?;
                }
                Some('_') => {
                    self.chars.next();
                }
                Some(_) | None => break,
            };
        }

        Ok(Token::Number(result))
    }

    fn parse_word(&mut self, start: usize) -> Token<'a> {
        loop {
            if let Some((i, c)) = self.chars.peek().copied() {
                if c.is_alphanumeric() || c == '-' || c == '_' {
                    self.chars.next();
                    continue;
                }

                break Token::Word(&self.string[start..i]);
            } else {
                break Token::Word(&self.string[start..]);
            }
        }
    }

    pub fn new(string: &'a str) -> Tokens<'a> {
        Tokens {
            string,
            chars: string.char_indices().peekable(),
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenType {
    Word,
    Number,
    And,
    Or,
    Plus,
    Dash,
    Colon,
    Dot,
    LeftParen,
    RightParen,
    Eof,
}

impl TokenType {
    pub(super) fn new(token: Option<Token>) -> TokenType {
        match token {
            Some(Token::Word(_)) => TokenType::Word,
            Some(Token::Number(_)) => TokenType::Number,
            Some(Token::And) => TokenType::And,
            Some(Token::Or) => TokenType::Or,
            Some(Token::Plus) => TokenType::Plus,
            Some(Token::Dash) => TokenType::Dash,
            Some(Token::Colon) => TokenType::Colon,
            Some(Token::Dot) => TokenType::Dot,
            Some(Token::LeftParen) => TokenType::LeftParen,
            Some(Token::RightParen) => TokenType::RightParen,
            None => TokenType::Eof,
        }
    }
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            TokenType::Word => write!(f, "word"),
            TokenType::Number => write!(f, "number"),
            TokenType::And => write!(f, "'&'"),
            TokenType::Or => write!(f, "'|'"),
            TokenType::Plus => write!(f, "'+'"),
            TokenType::Dash => write!(f, "'-'"),
            TokenType::Colon => write!(f, "':'"),
            TokenType::Dot => write!(f, "'.'"),
            TokenType::LeftParen => write!(f, "'('"),
            TokenType::RightParen => write!(f, "')'"),
            TokenType::Eof => write!(f, "EOF"),
        }
    }
}
