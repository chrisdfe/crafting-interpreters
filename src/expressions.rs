use crate::{literals::LiteralValue, tokens::Token};

#[derive(Debug, Clone, PartialEq)]
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

  // callee, left paren (1), arguments
  // (1) for error reporting
  Call(Box<Expr>, Token, Vec<Expr>),

  // expr
  Grouping(Box<Expr>),

  // name
  Variable(Token),
}
