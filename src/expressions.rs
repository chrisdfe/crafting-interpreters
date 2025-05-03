use crate::{literals::LiteralValue, tokens::Token};

// https://craftinginterpreters.com/representing-code.html#metaprogramming-the-trees

#[derive(Debug)]
pub enum Expr {
  Literal(LiteralValue),
  // operator, right
  Unary(Token, Box<Expr>),
  // left, operator, right
  Binary(Box<Expr>, Token, Box<Expr>),
  // expr
  Grouping(Box<Expr>),
}

impl Expr {
  pub fn to_string(&self) -> String {
    use Expr::*;
    match &self {
      Literal(value) => value.to_string(),
      Unary(operator, right) => {
        String::from(format!("({} {})", operator.lexeme, right.to_string()))
      }
      Binary(left, operator, right) => String::from(format!(
        "({} {} {})",
        operator.lexeme,
        left.to_string(),
        right.to_string()
      )),
      Grouping(expr) => String::from(format!("(group {})", expr.to_string())),
    }
  }
}
