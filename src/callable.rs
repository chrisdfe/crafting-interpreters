use crate::{interpreter::Interpreter, literals::LiteralValue, statements::Stmt, tokens::Token};
use std::fmt::Debug;

pub trait TheoCallable: Debug {
  fn name(&self) -> &String;
  fn arity(&self) -> usize;
  fn call(&self, interpreter: &mut Interpreter, arguments: Vec<LiteralValue>) -> LiteralValue;
}

impl std::fmt::Display for dyn TheoCallable {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "<fn {}>", self.name())
  }
}

#[derive(Debug)]
pub struct TheoFn {
  _name: String,
  _arity: usize,
  params: Vec<Token>,
  body: Vec<Stmt>,
}

impl TheoCallable for TheoFn {
  fn name(&self) -> &String {
    &self._name
  }

  fn arity(&self) -> usize {
    self._arity
  }

  fn call(&self, interpreter: &mut Interpreter, arguments: Vec<LiteralValue>) -> LiteralValue {
    todo!()
  }
}

impl TheoFn {
  pub fn new(name: String, params: Vec<Token>, body: Vec<Stmt>) -> Self {
    Self {
      _name: name,
      _arity: 0,
      params,
      body,
    }
  }
}
