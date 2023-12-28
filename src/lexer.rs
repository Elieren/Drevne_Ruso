use crate::token::Token;

pub fn check_quotes(s: &str) -> bool {
    if s.len() < 2 {
        return false;
    }
    let first_char = s.chars().next().unwrap();
    let last_char = s.chars().last().unwrap();
    (first_char == '\'' && last_char == '\'') || (first_char == '\"' && last_char == '\"')
}

pub fn process_variable(variable: &str, result: &mut Vec<Token>) {
    if !variable.is_empty() {
        if variable.parse::<i32>().is_ok() {
            let value = variable.parse::<i32>().unwrap();
            result.push(Token::Number(value));
        } else if variable == "ВОДА_БАЙКАЛА" {
            result.push(Token::Print);
        } else if variable == "ЦАРЬ_БАТЮШКА" {
        
        } else if check_quotes(variable){
            result.push(Token::StringVar(variable.to_string()));
        } else {
            result.push(Token::Variable(variable.to_string()));
        }
    }
}

pub fn lexer(input: &str) -> Vec<Token> {
    let mut result = Vec::new();
    let mut variable = String::new();
    let mut value = String::new();
    let mut recording = false;

    for character in input.chars() {
        match character {
            '(' => {
                process_variable(&variable, &mut result);
                if variable == "ЦАРЬ_БАТЮШКА" {
                    recording = true;
                }
                variable.clear();
                result.push(Token::OpenParenthesis);
            },
            ')' => {
                if recording {
                    result.push(Token::PrintText(value.clone()));
                    value.clear();
                    recording = false;
                }
                result.push(Token::CloseParenthesis);
            },
            '[' => {
                process_variable(&variable, &mut result);
                variable.clear();
                result.push(Token::OpenBracket);
            },
            ']' => {
                process_variable(&variable, &mut result);
                variable.clear();
                result.push(Token::CloseBracket);
            },
            ':' => {
                process_variable(&variable, &mut result);
                variable.clear();
                result.push(Token::Colon);
            },

            '+' | '-' | '=' | ';' | '*' | '/' | '^' | '√' => {
                process_variable(&variable, &mut result);
                variable.clear();
                match character {
                    '+' => result.push(Token::Plus),
                    '-' => result.push(Token::Minus),
                    '=' => result.push(Token::Equal),
                    ';' => result.push(Token::EndOfLine),
                    '*' => result.push(Token::Multiply),
                    '/' => result.push(Token::Divide),
                    '^' => result.push(Token::Exponentiation),
                    '√' => result.push(Token::Sqrt),
                    _ => {},
                }
            },
            _ if character.is_whitespace() => {

                let first_char = match variable.chars().next() {
                    Some(c) => c,
                    None => '_'
                };
                let last_char = match variable.chars().last() {
                    Some(c) => c,
                    None => '_'
                };
                if !(first_char == '\'' || first_char == '\"') {
                    process_variable(&variable, &mut result);
                    variable.clear();
                } else if (first_char == '\'' || first_char == '\"') && (last_char != first_char) {
                    variable.push(' ')
                } else if (first_char == '\'' && last_char == '\'') || (first_char == '\"' && last_char == '\"') {
                    process_variable(&variable, &mut result);
                    variable.clear();
                }
            },
            _ => {
                if recording {
                    value.push(character);
                } else {
                    variable.push(character);
                }
            },
        }
    }

    process_variable(&variable, &mut result);

    result
}
