use std::env;
use std::fs;
use std::fs::write;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
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
    let in_path = PathBuf::from(in_path);

    let file_name = match in_path.file_name() {
        Some(name) => name.to_string_lossy().into_owned(),
        None => {
            eprintln!("Error: input path has no file name");
            return;
        }
    };

    let dot_pos = match file_name.find('.') {
        Some(i) => i,
        None => {
            return;
        }
    };

    let program_name: String = file_name.chars().take(dot_pos).collect();

    let mut out_path = String::from(&program_name);
    out_path.push_str(".s");

    let s = match fs::read_to_string(&in_path) {
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

    match write(&out_path, &s_program) {
        Ok(_) => (),
        Err(e) => {
            println!("{:?}", e);
            return;
        }
    }

    println!("Program: {:?}", s_program);

    // let dir = in_path.parent().unwrap_or(Path::new(""));
    // let gcc_out_path = dir.join(&program_name);

    // let gcc_output = Command::new("gcc")
    // .arg(&out_path)
    // .arg("-o")
    // .arg(&gcc_out_path)
    // .output()
    // .expect("Failed to execute gcc");

    // if !gcc_output.status.success() {
    // eprintln!(
    // "gcc failed with output: \n{}",
    // String::from_utf8_lossy(&gcc_output.stderr)
    // );
    // return;
    // }

    // let rm_output = Command::new("rm")
    // .arg(&out_path)
    // .output()
    // .expect("Failed to execute rm");

    // if !rm_output.status.success() {
    // eprintln!(
    // "Failed to delete assembly file: rm failed with: \n{}",
    // String::from_utf8_lossy(&rm_output.stderr)
    // );
    // }

    // println!("Successfully compiled:");
}
