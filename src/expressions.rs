use crate::{literals::LiteralValue, statements::Stmt, tokens::Token};

// https://craftinginterpreters.com/representing-code.html#metaprogramming-the-trees

#[derive(Debug)]
pub enum Expr {
  Literal(LiteralValue),

  // left, operator, right
  Logical(Box<Expr>, Token, Box<Expr>),

  // name, value
  Assign(Token, Box<Expr>),

  // operator, right
  Unary(Token, Box<Expr>),

  // left, operator, right
  Binary(Box<Expr>, Token, Box<Expr>),

  // expr
  Grouping(Box<Expr>),

  // name
  Variable(Token),
}

impl Expr {
  /*
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
      Variable(token) => String::from(format!("var {:?}", token.lexeme)),
    }
  }
  */
}
