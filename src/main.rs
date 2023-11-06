use std::collections::HashMap;
use std::env;
use std::fs;

mod lexer;
mod token;
mod shunting_yard;
mod sorting;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 1 {
        println!("Пожалуйста, укажите имя файла в качестве аргумента");
        return;
    }
    let filename = &args[1];
    let mut input = fs::read_to_string(filename)
        .expect("Не удалось прочитать файл");

    input = input.replace("\n", " ");

    let tokens = lexer::lexer(&input);
    let mut variables = HashMap::new();
    sorting::parser(tokens, &mut variables);
}
