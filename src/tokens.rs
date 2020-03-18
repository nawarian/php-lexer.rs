#[derive(Debug, PartialEq)]
pub enum Token {
    Illegal,
    EndOfFile,

    // Symbols
    Variable(String),
    LNumber(String),

    // Operations
    Assignment,

    // Delimiters
    Semicolon,
    Whitespace,
}
