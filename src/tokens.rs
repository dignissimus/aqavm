#[derive(Debug)]
pub enum Token {
    Name(String),
    StringLiteral(String),
    HexLiteral(String),
    IntegerLiteral(String),
    LeftBracket,
    RightBracket,
    Comma,
    Whitespace,
    EndOfFile
}