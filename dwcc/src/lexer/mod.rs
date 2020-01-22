mod cursor;
mod utils;

use crate::lexer::cursor::Cursor;

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    OpenBrace,
    CloseBrace,
    OpenParen,
    CloseParen,
    Semicolon,
    KeywordInt,
    KeywordReturn,
    Identifier,
    Integer,
    Error,
    None,
}

pub struct Token {
    kind: TokenKind,
    len: usize,
}
