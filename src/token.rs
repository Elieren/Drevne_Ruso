#[derive(Debug)]
#[derive(Clone)]
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
    StringVar(String)
}


pub enum TokenType {
    Int(i32),
    String(String)
}