use std::{fs, process::ExitCode};

mod expressions;
mod parser;
mod scanner;
mod tokens;

use expression::*;
use scanner::Scanner;
use tokens::{Token, TokenType};

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

  let expr = BinaryExpression {
    left: Box::new(UnaryExpression {
      operator: Token {
        token_type: TokenType::Minus,
        lexeme: String::from("-"),
        line: 1,
      },
      right: Box::new(LiteralExpression {
        value: LiteralValue::Num(123.),
      }),
    }),
    operator: Token {
      token_type: TokenType::Star,
      lexeme: String::from("*"),
      line: 1,
    },
    right: Box::new(GroupingExpression {
      expr: Box::new(LiteralExpression {
        value: LiteralValue::Num(45.67),
      }),
    }),
  };

  println!("{}", expr.to_string());

  // let _ = match read_file(&args[1]) {
  //   Err(_) => {
  //     return ExitCode::FAILURE;
  //   }
  //   Ok(_) => 1,
  // };

  ExitCode::SUCCESS
}

fn read_file(file_name: &String) -> Result<(), String> {
  let source = match fs::read_to_string(file_name) {
    Err(_) => return Err(String::from("couldn't read file {file_name}")),
    Ok(contents) => contents,
  };

  let scanner = Scanner::scan(source);

  if scanner.errors.len() > 0 {
    println!("finished with errors: ");
    for error in scanner.errors {
      println!(
        "'{}' at line {} column {}",
        error.message, error.line, error.column,
      )
    }
  }

  println!("tokens: ");
  for token in scanner.tokens {
    match token {
      _ => {
        println!("{:?}", &token.token_type);
      }
    }
  }

  Ok(())
}
