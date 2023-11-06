use crate::token::Token;

pub fn shunting_yard(tokens: Vec<Token>) -> Vec<Token> {
    let mut output_queue = Vec::new();
    let mut operator_stack = Vec::new();

    for token in tokens {
        match token {
            Token::Number(_) | Token::Variable(_) => output_queue.push(token),
            Token::Plus | Token::Minus | Token::Multiply | Token::Divide | Token::Sqrt | Token::Exponentiation => {
                while let Some(top) = operator_stack.last() {
                    match top {
                        Token::Multiply | Token::Divide | Token::Sqrt | Token::Exponentiation => {
                            output_queue.push(operator_stack.pop().unwrap());
                        }
                        _ => break,
                    }
                }
                operator_stack.push(token);
            }
            Token::OpenParenthesis => operator_stack.push(token),
            Token::CloseParenthesis => {
                while let Some(top) = operator_stack.pop() {
                    match top {
                        Token::OpenParenthesis => break,
                        _ => output_queue.push(top),
                    }
                }
            }
            _ => {}
        }
    }

    while let Some(op) = operator_stack.pop() {
        output_queue.push(op);
    }

    output_queue
}