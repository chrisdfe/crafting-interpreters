use crate::{expressions::Expr, tokens::Token};

#[derive(Debug)]
pub enum Stmt {
  // statements
  Block(Vec<Stmt>),
  Expr(Expr),
  // condition, thenBranch, elseBranch
  If(Expr, Box<Stmt>, Option<Box<Stmt>>),
  Print(Expr),
  // name, initializer
  Var(Token, Option<Expr>),
}
