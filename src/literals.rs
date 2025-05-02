use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum LiteralValue {
  Nil,
  True,
  False,
  Num(f32),
  Str(String),
}

impl Display for LiteralValue {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    use LiteralValue::*;
    match self {
      Nil => write!(f, "nil"),
      True => write!(f, "true"),
      False => write!(f, "false"),
      Str(s) => write!(f, "{}", s),
      Num(n) => write!(f, "{}", n),
    }
  }
}
