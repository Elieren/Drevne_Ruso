#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub enum Token {
    OpenParenthesis,
    CloseParenthesis,
    Plus,
    Minus,
    Equal,
    Print,
    Number(i32),
    Variable(String),
    EndOfLine,
    PrintText(String),
    Multiply,
    Divide,
    Exponentiation,
    Sqrt,
    StringVar(String),
    OpenBracket,
    CloseBracket,
    Colon
}

#[derive(Debug)]
pub enum TokenType {
    Int(i32),
    String(String)
}