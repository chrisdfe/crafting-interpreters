#![allow(
  //
  dead_code,
  unused_variables,
  clippy::borrowed_box
)]

use std::{fs, io::Write, process::ExitCode};

mod callable;
mod cli;
mod control_flow;
mod environments;
mod expressions;
mod interpreter;
mod literals;
mod parser;
mod scanner;
mod statements;
mod tokens;

use cli::get_args;
use interpreter::Interpreter;
use parser::Parser;
use scanner::Scanner;

fn main() -> ExitCode {
  let args = get_args();

  if let Some(filename) = args.filename {
    //
    match read_file(filename) {
      Ok(()) => (),
      Err(msg) => println!("{}", msg),
    };
  } else {
    interpret_interactive();
  }

  ExitCode::SUCCESS
}

fn interpret_interactive() {
  let mut interpreter = Interpreter::new();

  loop {
    print!(">");
    let mut input = String::new();
    let _ = std::io::stdout().flush();

    std::io::stdin()
      .read_line(&mut input)
      .unwrap_or_else(|_| panic!("{}", format!("Unable to read line '{}'", &input).to_owned()));

    interpret_input(input, &mut interpreter);
  }
}

fn read_file(filename: String) -> Result<(), String> {
  let input = match fs::read_to_string(&filename) {
    Err(_) => return Err(format!("CLI error: couldn't read file {}", filename)),
    Ok(contents) => contents,
  };

  if input.is_empty() {
    return Err(format!("File '{}' is empty.", &filename));
  }

  let mut interpreter = Interpreter::new();
  interpret_input(input, &mut interpreter);
  Ok(())
}

fn interpret_input(input: String, interpreter: &mut Interpreter) {
  let tokens = Scanner::scan(input);

  // for token in tokens.iter() {
  //   println!("{:?}", &token);
  // }

  let statements = match Parser::parse(tokens) {
    Ok(statements) => statements,
    Err(err) => {
      println!("{}", err.message);
      return;
    }
  };

  match interpreter.interpret(statements) {
    Ok(_) => (),
    Err(err) => {
      println!("{}", err.message);
    }
  }
}
