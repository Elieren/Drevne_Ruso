use std::collections::HashMap;
// use std::result;
use crate::token::Token;
use crate::token::TokenType;
use meval::eval_str;

pub fn parser(tokens: Vec<Token>, variables: &mut HashMap<String, i32>, variables_string: &mut HashMap<String, String>) {
    let mut current_variable = String::new();
    let mut is_assignment = false;
    let mut expression = Vec::new();
    let mut is_indexing = false;
    let mut index_expression = Vec::new();

    for token in tokens {
        // println!("{:?}", token);
        match token {
            Token::Equal => {
                is_assignment = true;
                expression.clear();
            }
            Token::EndOfLine => {
                if is_assignment {
                    // println!("{:?}", expression);
                    let mut contains_string_var: bool = false;
                    let temp_val: Token;
                    let mut temp_vec = Vec::new();
                    let mut k: bool = true;

                    let mut result: TokenType = TokenType::Int(1);
                    if expression.len() == 1 {
                        if let Some(Token::Variable(_)) = expression.first() {
                            if variables_string.contains_key(&current_variable) {
                                contains_string_var = true;
                                let s: String = variables_string.get(&current_variable).unwrap_or(&"None".to_string()).to_string();
                                temp_val = Token::StringVar(s);
                                temp_vec.push(temp_val);
                            } else if variables.contains_key(&current_variable) {
                                contains_string_var = false;
                                let s: i32 = *variables.get(&current_variable).unwrap_or(&0);
                                temp_val = Token::Number(s);
                                temp_vec.push(temp_val);
                            } else {
                                contains_string_var = expression.iter().any(|token| matches!(token, Token::StringVar(_)));
                                if contains_string_var {
                                    result = TokenType::String(evaluate_str(expression.clone(), variables_string, index_expression.clone()));
                                    index_expression.clear();
            
                                } else {
                                    result = TokenType::Int(evaluate(expression.clone(), variables));
                                }
                                k = false;
                            }
                            
                            if k {
                                if contains_string_var {
                                    result = TokenType::String(evaluate_str(temp_vec.clone(), variables_string, index_expression.clone()));
                                    index_expression.clear();
            
                                } else {
                                    result = TokenType::Int(evaluate(temp_vec.clone(), variables));
                                }
                            }
                        } else {
                            contains_string_var = expression.iter().any(|token| matches!(token, Token::StringVar(_)));
                            if contains_string_var {
                                result = TokenType::String(evaluate_str(expression.clone(), variables_string, index_expression.clone()));
                                index_expression.clear();
        
                            } else {
                                result = TokenType::Int(evaluate(expression.clone(), variables));
                            }
                        }


                    } else {
                        contains_string_var = expression.iter().any(|token| matches!(token, Token::StringVar(_)));

                        if contains_string_var {
                            result = TokenType::String(evaluate_str(expression.clone(), variables_string, index_expression.clone()));
                            index_expression.clear();
    
                        } else {
                            result = TokenType::Int(evaluate(expression.clone(), variables));
                        }
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
            },
            Token::OpenBracket => {
                is_indexing = true;
                index_expression.clear();
            },
            Token::CloseBracket => {
                if is_indexing {
                    // println!("{:?}", index_expression);
                    is_indexing = false;
                }
            }
            _ => {
                if is_assignment && !is_indexing {
                    expression.push(token);
                } else if is_indexing {
                    index_expression.push(token);
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
pub fn evaluate_str(tokens: Vec<Token>, variables_string: &mut HashMap<String, String>, index_expression: Vec<Token>) -> String {
    let mut x: String = String::new();
    let mut z: bool = false;
    
    for token in tokens {
        match token {
            Token::StringVar(v) => {
                if z {
                    x.push_str(&v[1..v.len()-1])
                } else {
                    if index_expression.is_empty() {
                        x = String::from(&v[1..v.len()-1])
                    } else {
                        x = slice(v.clone(), index_expression.clone())
                    }
                }
            },
            Token::Plus => z = true,
            Token::Variable(v) => {
                let mut s: String = variables_string.get(&v).unwrap_or(&"None".to_string()).to_string();
                if !index_expression.is_empty() {
                    s = slice(s.clone(), index_expression.clone())
                }
                if z {
                    x.push_str(&s)
                } else {
                    x = String::from(&s)
                }
            }
            _ => {}
        }
    }
    
    x
}

pub fn slice(x: String, mut index_expression: Vec<Token>) -> String {
    let len_text = x.len();
    let mut z = Vec::new();

    // println!("{:?}", index_expression);
    index_expression.retain(|token| *token != Token::Colon);
    // println!("{:?}", index_expression);

    for i in &index_expression {
        match i {
            Token::Number(v) => {
                if (*v <= len_text as i32) && (*v >= 0) {
                    z.push(*v as usize);
                } else {
                    panic!();
                }
            },
            _ => {},
        }
    }

    let y = &x[z[0]..z[1]];
    index_expression.clear();
    y.to_string()

}