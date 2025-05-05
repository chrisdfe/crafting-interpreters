use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
  #[arg(short, long)]
  pub filename: Option<String>,
}

pub fn get_args() -> Args {
  Args::parse()
}
