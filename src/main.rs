use std::{fs, process::ExitCode};

mod scanner;

fn main() -> ExitCode {
  let args: Vec<String> = std::env::args().collect();

  println!("len: {}", args.len());

  // TODO - handle both cargo run & regular script running
  // with cargo run, 'run' is arg 1
  if args.len() > 2 {
    println!("usage: lox [script]");
    return ExitCode::from(64);
  }

  // args[0] will be 'run', so args[1] is the one we want
  if args.len() == 2 {
    let _ = match read_file(&args[1]) {
      Err(_) => {
        return ExitCode::FAILURE;
      }
      Ok(_) => 1,
    };
  }

  ExitCode::SUCCESS
}

fn read_file(file_name: &String) -> Result<(), String> {
  let contents = match fs::read_to_string(file_name) {
    Err(_) => return Err(String::from("couldn't read file {file_name}")),
    Ok(contents) => contents,
  };

  println!("contnets: {contents}");
  let _tokens = scanner::scan(contents);

  Ok(())
}
