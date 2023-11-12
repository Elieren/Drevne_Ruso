use std::collections::HashMap;
use std::env;
use std::fs;

mod lexer;
mod token;
mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 1 {
        println!("Пожалуйста, укажите имя файла в качестве аргумента");
        return;
    }
    let filename = &args[1];
    // let filename = "src\\drevnerus.dr";
    let mut input = fs::read_to_string(filename)
        .expect("Не удалось прочитать файл");

    input = input.replace("\n", " ");

    let tokens = lexer::lexer(&input);
    // println!("{:?}", tokens);
    let mut variables_int = HashMap::new();
    let mut variables_string = HashMap::new();
    parser::parser(tokens, &mut variables_int, &mut variables_string);
}
