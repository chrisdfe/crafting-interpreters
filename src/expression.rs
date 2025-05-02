use std::fmt::Display;

use super::tokens::Token;

// https://craftinginterpreters.com/representing-code.html#metaprogramming-the-trees

pub trait Expr {
  fn to_string(&self) -> String {
    String::from("")
  }
}

// left: &'a Expr<'a>,
// operator: Token,
// right: &'a Expr<'a>,

pub struct BinaryExpression {
  pub left: Box<dyn Expr>,
  pub operator: Token,
  pub right: Box<dyn Expr>,
}

impl Expr for BinaryExpression {
  fn to_string(&self) -> String {
    return String::from(format!(
      "({} {} {})",
      self.operator.lexeme,
      self.left.to_string(),
      self.right.to_string()
    ));
  }
}

pub struct GroupingExpression {
  pub expr: Box<dyn Expr>,
}

impl Expr for GroupingExpression {
  fn to_string(&self) -> String {
    return String::from(format!("(group {})", self.expr.to_string()));
  }
}

#[derive(Debug)]
pub enum LiteralValue {
  Str(String),
  Num(f32),
}

impl Display for LiteralValue {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    use LiteralValue::*;
    match self {
      Str(s) => write!(f, "{}", s),
      Num(n) => write!(f, "{}", n),
    }
  }
}

pub struct LiteralExpression {
  pub value: LiteralValue,
}

impl Expr for LiteralExpression {
  fn to_string(&self) -> String {
    return String::from(format!("{}", self.value));
  }
}

pub struct UnaryExpression {
  pub operator: Token,
  pub right: Box<dyn Expr>,
}

impl Expr for UnaryExpression {
  fn to_string(&self) -> String {
    return String::from(format!(
      "({} {})",
      self.operator.lexeme,
      self.right.to_string()
    ));
  }
}
