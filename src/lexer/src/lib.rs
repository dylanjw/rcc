mod cursor;
use utils;

use crate::cursor::Cursor;

#[derive(Debug, PartialEq, Eq)]
pub enum TokenKind {
    OpenBrace,
    CloseBrace,
    OpenParen,
    CloseParen,
    Semicolon,
    Ident,
    Integer,
    Error,
    Whitespace,
    None,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Token {
    kind: TokenKind,
    len: usize,
}


impl Cursor<'_> {
    fn next_token(&mut self) -> Token {
        let first_char = self.bump().unwrap();
        let token_kind = match first_char {
            c if utils::is_whitespace(c) => TokenKind::Whitespace,
            // single char tokenkinds
            '{' => TokenKind::OpenBrace,
            '}' => TokenKind::CloseBrace,
            '(' => TokenKind::OpenParen,
            ')' => TokenKind::CloseParen,
            ';' => TokenKind::Semicolon,

            c if utils::is_id_start(c) => {
                self.eat_ident();
                TokenKind::Ident
            },
            c if utils::is_numeric(c) => {
                self.eat_int();
                TokenKind::Integer
            },
            _ => TokenKind::None
        };
        Token { kind: token_kind, len: self.len_consumed() }
    }

    fn eat_while<F>(&mut self, mut predicate: F) -> usize
    where
        F: FnMut(char) -> bool,
    {
        let mut eaten: usize = 0;
        while predicate(self.first()) {
            eaten += 1;
            self.bump();
        }

        eaten

    }

    fn eat_ident(&mut self) -> usize {
        self.bump();
        self.eat_while(utils::is_id_continue)
    }

    fn eat_int(&mut self) -> usize {
        self.bump();
        self.eat_while(utils::is_numeric)
    }
}

fn pop_first_token(input: &str) -> Token {
    let mut c = Cursor::new(input);
    c.next_token()
}

pub fn tokenize(mut input: &str) -> impl Iterator<Item = Token> + '_ {
    std::iter::from_fn(move || {
        if input.is_empty() {
            return None;
        }
        let token = pop_first_token(input);
        input = &input[token.len..];
        Some(token)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_token_single_char_tokens() {
        let test_input = "{}();";
        let mut token_iter = tokenize(test_input);
        assert_eq!(Token { kind: TokenKind::OpenBrace, len: 1 }, token_iter.next().unwrap());
        assert_eq!(Token { kind: TokenKind::CloseBrace, len: 1}, token_iter.next().unwrap());
        assert_eq!(Token { kind: TokenKind::OpenParen, len: 1 }, token_iter.next().unwrap());
        assert_eq!(Token { kind: TokenKind::CloseParen, len: 1 }, token_iter.next().unwrap());
        assert_eq!(Token { kind: TokenKind::Semicolon, len: 1 }, token_iter.next().unwrap());

    }
    #[test]
    fn test_integer_tokens() {
        let test_input = "{101}(22)42;";
        let mut token_iter = tokenize(test_input);
        assert_eq!(Token { kind: TokenKind::OpenBrace, len: 1 }, token_iter.next().unwrap());
        assert_eq!(Token { kind: TokenKind::Integer, len: 3 }, token_iter.next().unwrap());
        assert_eq!(Token { kind: TokenKind::CloseBrace, len: 1}, token_iter.next().unwrap());
        assert_eq!(Token { kind: TokenKind::OpenParen, len: 1 }, token_iter.next().unwrap());
        assert_eq!(Token { kind: TokenKind::Integer, len: 2}, token_iter.next().unwrap());
        assert_eq!(Token { kind: TokenKind::CloseParen, len: 1 }, token_iter.next().unwrap());
        assert_eq!(Token { kind: TokenKind::Integer, len: 2}, token_iter.next().unwrap());
        assert_eq!(Token { kind: TokenKind::Semicolon, len: 1 }, token_iter.next().unwrap());
    }
    #[test]
    fn test_indentifier_tokens() {
        let test_input = "none bob int";
        let mut token_iter = tokenize(test_input);
        assert_eq!(Token { kind: TokenKind::Ident, len: 4 }, token_iter.next().unwrap());
        assert_eq!(Token { kind: TokenKind::Whitespace, len: 1 }, token_iter.next().unwrap());
        assert_eq!(Token { kind: TokenKind::Ident, len: 3 }, token_iter.next().unwrap());
        assert_eq!(Token { kind: TokenKind::Whitespace, len: 1 }, token_iter.next().unwrap());
        assert_eq!(Token { kind: TokenKind::Ident, len: 3}, token_iter.next().unwrap());
    }
}
