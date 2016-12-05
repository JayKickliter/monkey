#[derive(Debug,PartialEq)]
pub enum Token {
    // Special purpose tokens
    Illegal(char),
    Eof,

    // Identifiers and literals
    Ident(String),
    Int(String),

    // Operators
    Assign,
    Plus,

    // Delimiters
    Comma,
    Semicolon,

    // Grouping
    LParen,
    RParen,
    LBrace,
    RBrace,

    // Keywords
    Function,
    Let,
}
