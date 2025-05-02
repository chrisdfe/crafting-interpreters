use std::fmt::Display;

use super::tokens::Token;

// https://craftinginterpreters.com/representing-code.html#metaprogramming-the-trees

pub enum Expr {
  // left, operator, right
  Binary(Box<Expr>, Token, Box<Expr>),
  // expr
  Grouping(Box<Expr>),
  Literal(LiteralValue),
  // operator, right
  Unary(Token, Box<Expr>),
}

impl Expr {
  pub fn to_string(&self) -> String {
    use Expr::*;
    match &self {
      Binary(left, operator, right) => String::from(format!(
        "({} {} {})",
        operator.lexeme,
        left.to_string(),
        right.to_string()
      )),
      Grouping(expr) => String::from(format!("(group {})", expr.to_string())),
      Literal(value) => value.to_string(),
      Unary(operator, right) => {
        String::from(format!("({} {})", operator.lexeme, right.to_string()))
      }
    }
  }
}

#[derive(Debug)]
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
