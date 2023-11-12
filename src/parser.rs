use std::collections::HashMap;
// use std::result;
use crate::token::Token;
use crate::token::TokenType;
use meval::eval_str;

pub fn parser(tokens: Vec<Token>, variables: &mut HashMap<String, i32>, variables_string: &mut HashMap<String, String>) {
    let mut current_variable = String::new();
    let mut is_assignment = false;
    let mut expression = Vec::new();

    for token in tokens {
        // println!("{:?}", token);
        match token {
            Token::Equal => {
                is_assignment = true;
                expression.clear();
            }
            Token::EndOfLine => {
                if is_assignment {
                    let result: TokenType;
                    let contains_string_var = expression.iter().any(|token| matches!(token, Token::StringVar(_)));
                    if contains_string_var {
                        result = TokenType::String(evaluate_str(expression.clone()));
                    } else {
                        result = TokenType::Int(evaluate(expression.clone(), variables));
                    }

                    match result {
                        TokenType::Int(v) => {
                            if variables_string.contains_key(&current_variable) {
                                variables_string.remove(&current_variable);
                            }
                            variables.insert(current_variable.clone(), v);
                        },
                        TokenType::String(v) => {
                            if variables.contains_key(&current_variable) {
                                variables.remove(&current_variable);
                            }
                            variables_string.insert(current_variable.clone(), v);
                        }
                    }
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
                    if variables.contains_key(&current_variable) {
                        println!("{}", variables.get(&current_variable).unwrap_or(&0));
                    } else if variables_string.contains_key(&current_variable) {
                        println!("{}", variables_string.get(&current_variable).unwrap_or(&"None".to_string()));
                    }
                } else if let Token::PrintText(v) = token {
                    println!("{}", v);
                }

            }
        }
    }
}

// Math
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

// String
pub fn evaluate_str(tokens: Vec<Token>) -> String {
    let mut x: String = String::new();
    let mut z: bool = false;
    
    for token in tokens {
        match token {
            Token::StringVar(v) => {
                if z {
                    x.push_str(&v[1..v.len()-1])
                } else {
                    x = String::from(&v[1..v.len()-1])
                }
            },
            Token::Plus => z = true,
            _ => {}
        }
    }

    x
}