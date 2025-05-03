use crate::{expressions::Expr, tokens::Token};

pub enum Stmt {
  Expr(Expr),
  Print(Expr),
  // name, initializer
  Var(Token, Option<Expr>),
}
