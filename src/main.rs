use std::{fs, io::Write, process::ExitCode};

mod environments;
mod expressions;
mod interpreter;
mod literals;
mod parser;
mod scanner;
mod statements;
mod tokens;

use interpreter::Interpreter;
use parser::Parser;
use scanner::Scanner;

fn main() -> ExitCode {
  let args: Vec<String> = std::env::args().collect();

  /*
  // TODO - handle both cargo run & regular script running
  // with cargo run, 'run' is arg 1
  if args.len() > 2 {
    println!("usage: lox [script]");
    return ExitCode::from(64);
  }

  // args[0] will be 'run', so args[1] is the one we want
  if args.len() < 2 {
    println!("filename arg required");
    return ExitCode::from(64);
  }

  let _ = match read_file(&args[1]) {
    Err(_) => {
      println!("Couldn't read file '{}'", &args[1]);
      return ExitCode::FAILURE;
    }
    Ok(_) => 1,
  };
  */

  let mut interpreter = Interpreter::new();

  loop {
    print!(">");
    let mut input = String::new();
    let _ = std::io::stdout().flush();

    std::io::stdin()
      .read_line(&mut input)
      .expect(&format!("Unable to read line '{}'", &input).to_owned());

    // println!("{}", result);
    let tokens = Scanner::scan(input);

    let statements = Parser::parse(tokens);
    match interpreter.interpret(statements) {
      Err(err) => {
        println!("{}", err.message);
      }
      _ => (),
    }
  }

  ExitCode::SUCCESS
}

fn read_file(file_name: &String) -> Result<(), String> {
  let source = match fs::read_to_string(file_name) {
    Err(_) => return Err(String::from("couldn't read file {file_name}")),
    Ok(contents) => contents,
  };

  println!("scanning.");
  // let scanner = Scanner::scan(source);

  // if scanner.errors.len() > 0 {
  //   println!("scanner errors: ");
  //   for error in scanner.errors {
  //     println!(
  //       "'{}' at line {} column {}",
  //       error.message, error.line, error.column,
  //     )
  //   }
  //   return Err(String::from("couldn't scan file"));
  // }

  /*
  println!("scanner tokens: ");
  for token in scanner.tokens.iter() {
    match token {
      _ => {
        println!("{:?}", &token);
      }
    }
  }
  */
  /*
  let mut parser = Parser::new(scanner.tokens);

  println!("\nparsing.");

  let ast = match parser.parse() {
    Err(err) => return Err(format!("Parsing error: {}", err.full_text())),
    Ok(ast) => ast,
  };

  Interpreter::interpret(&ast); */

  let mut interpreter = Interpreter::new();

  Ok(())
}
