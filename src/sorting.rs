use std::collections::HashMap;
use crate::token::Token;
use meval::eval_str;

pub fn parser(tokens: Vec<Token>, variables: &mut HashMap<String, i32>) {
    let mut current_variable = String::new();
    let mut is_assignment = false;
    let mut expression = Vec::new();

    for token in tokens {
        match token {
            Token::Equal => {
                is_assignment = true;
                expression.clear();
            }
            Token::EndOfLine => {
                if is_assignment {
                    let result = evaluate(expression.clone(), variables);
                    variables.insert(current_variable.clone(), result);
                    is_assignment = false;
                }
                expression.clear();
            }
            _ => {
                if is_assignment {
                    expression.push(token);
                } else if let Token::Variable(v) = token {
                    current_variable = v;
                } else if let Token::Print = token {
                    println!("{}", variables.get(&current_variable).unwrap_or(&0));
                } else if let Token::PrintText(v) = token {
                    println!("{}", v);
                }

            }
        }
    }
}

pub fn evaluate(tokens: Vec<Token>, variables: &HashMap<String, i32>) -> i32 {
    let mut stack: String = String::new();

    for token in tokens {
        match token {
            Token::Number(n) => {
                let n: String = n.to_string();
                stack.push_str(n.as_str());
            },
            Token::Variable(v) => {
                if let Some(&value) = variables.get(&v) {
                    let value: String = value.to_string();
                    stack.push_str(value.as_str());
                } else {
                    panic!("Variable {} not defined", v);
                }
            }
            Token::Plus => {
                stack.push('+');
            }
            Token::Minus => {
                stack.push('-');
            }
            Token::Multiply => {
                stack.push('*');
            }
            Token::Divide => {
                stack.push('/');
            }
            Token::Exponentiation => {
                stack.push('^');
            }
            Token::Sqrt => {
                stack.push_str("sqrt");
            }
            Token::OpenParenthesis => {
                stack.push('(');
            }
            Token::CloseParenthesis => {
                stack.push(')');
            }
            _ => {}
        }
    }

    if let Ok(result) = eval_str(stack) {
        result as i32
    } else {
        0
    }
}
