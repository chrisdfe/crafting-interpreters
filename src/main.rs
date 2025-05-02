use std::{fs, process::ExitCode};

mod expressions;
mod literals;
mod parser;
mod scanner;
mod tokens;

use parser::Parser;
use scanner::Scanner;

fn main() -> ExitCode {
  let args: Vec<String> = std::env::args().collect();

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

  ExitCode::SUCCESS
}

fn read_file(file_name: &String) -> Result<(), String> {
  let source = match fs::read_to_string(file_name) {
    Err(_) => return Err(String::from("couldn't read file {file_name}")),
    Ok(contents) => contents,
  };

  println!("scanning.");
  let scanner = Scanner::scan(source);

  if scanner.errors.len() > 0 {
    println!("scanner errors: ");
    for error in scanner.errors {
      println!(
        "'{}' at line {} column {}",
        error.message, error.line, error.column,
      )
    }
    return Err(String::from("couldn't scan file"));
  }

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

  let mut parser = Parser::new(scanner.tokens);

  println!("\nparsing.");

  match parser.parse() {
    Ok(expr) => {
      println!("parsing successful");
      println!("{}", expr.to_string());
    }
    Err(err) => {
      println!("Parsing error: {}", err.full_text());
    }
  }

  Ok(())
}
