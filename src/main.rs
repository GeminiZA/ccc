use std::env;
use std::fs;
use std::fs::write;
use std::io::Result;
use token::Token;

mod generator;
mod lexer;
mod parser;
mod token;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Requied path: usage ccc path");
        return;
    }

    let in_path = &args[1];

    let out_path = &args[2];

    let s = match fs::read_to_string(in_path) {
        Ok(s) => s,
        Err(e) => {
            println!("{:?}", e);
            return;
        }
    };

    let tokens: Vec<Token> = lexer::lex(&s);

    println!("Tokens: {:?}", tokens);

    let program_result = parser::parse_program(&tokens);

    let program = match program_result {
        Ok(prog) => prog,
        Err(e) => {
            eprintln!("Error parsing program: {:?}", e);
            return;
        }
    };

    println!("AST: {:?}", program);

    let s_program = generator::generate(&program);

    match write(out_path, &s_program) {
        Ok(_) => (),
        Err(e) => {
            println!("{:?}", e);
            return;
        }
    }

    println!("{:?}", s_program);
}
