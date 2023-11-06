use std::collections::HashMap;
use crate::token::Token;
use crate::shunting_yard::shunting_yard;

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
                    let rpn = shunting_yard(expression.clone());
                    let result = evaluate(rpn, variables);
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
    let mut stack = Vec::new();
    let mut x: bool = true;

    for token in tokens {
        match token {
            Token::Number(n) => if x {
                stack.push(n);
            } else {
                stack.insert(0, n);
            },
            Token::Variable(v) => {
                if let Some(&value) = variables.get(&v) {
                    stack.push(value);
                } else {
                    panic!("Variable {} not defined", v);
                }
            }
            Token::Plus => {
                let rhs = stack[0];
                stack.remove(0);
                let lhs = stack[0];
                stack.remove(0);
                stack.insert(0, lhs + rhs);
                x = false;
            }
            Token::Minus => {
                let lhs = stack[0];
                stack.remove(0);
                let rhs = stack[0];
                stack.remove(0);
                stack.push(lhs - rhs);
                x = false;
            }
            Token::Multiply => {
                let rhs = stack[0];
                stack.remove(0);
                let lhs = stack[0];
                stack.remove(0);
                stack.push(lhs * rhs);
                x = false;
            }
            Token::Divide => {
                let lhs = stack[0];
                stack.remove(0);
                let rhs = stack[0];
                stack.remove(0);
                stack.push(lhs / rhs);
                x = false;
            }
            Token::Exponentiation => {
                let lhs = stack[0];
                stack.remove(0);
                let rhs = stack[0];
                stack.remove(0);
                stack.insert(0, lhs.pow(rhs as u32));
                x = false;
            }
            Token::Sqrt => {
                let n = stack[0];
                stack.remove(0);
                stack.insert(0, (n as f64).sqrt() as i32);
                x = false;
            }
            _ => {}
        }
    }

    stack.pop().unwrap()
}
