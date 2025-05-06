use crate::{expressions::Expr, tokens::Token};

#[derive(Debug, Clone)]
pub enum Stmt {
  // statements
  Block(Vec<Stmt>),

  // expression
  Expr(Expr),

  // name, params, body
  Function(String, Vec<Token>, Vec<Stmt>),

  // condition, thenBranch, elseBranch
  If(Expr, Box<Stmt>, Option<Box<Stmt>>),

  // expression
  Print(Expr),

  // name, initializer
  Var(Token, Option<Expr>),

  // condition, body
  While(Box<Expr>, Box<Stmt>),
}

impl Stmt {
  pub fn noop_block() -> Self {
    Stmt::Block(Vec::new())
  }
}
