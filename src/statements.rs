use crate::{expressions::Expr, tokens::Token};

#[derive(Debug)]
pub enum Stmt {
  Expr(Expr),
  Print(Expr),
  // name, initializer
  Var(Token, Option<Expr>),
}
