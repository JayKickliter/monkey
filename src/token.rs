#[derive(Debug,PartialEq)]
pub enum Token {
    // Special purpose tokens
    Eof,
    Illegal(char),

    // Identifiers and literals
    Ident(String),
    Int(String),

    // Operators
    Assign,
    Asterisk,
    Bang,
    Minus,
    Plus,
    Slash,

    // Comparison
    Gt,
    Lt,
    Eq,
    NotEq,

    // Delimiters
    Comma,
    Semicolon,

    // Grouping
    LBrace,
    LParen,
    RBrace,
    RParen,

    // Keywords
    Else,
    False,
    Function,
    If,
    Let,
    Return,
    True,
}
