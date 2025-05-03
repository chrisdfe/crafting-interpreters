use crate::expressions::Expr;

pub enum Stmt {
  Expr(Expr),
  Print(Expr),
}
