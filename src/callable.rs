use crate::{
  interpreter::{Interpreter, RuntimeErr},
  literals::LiteralValue,
  statements::Stmt,
  tokens::Token,
};
use std::fmt::Debug;

pub trait TheoCallable: Debug {
  fn name(&self) -> &String;
  fn arity(&self) -> usize;
  fn call(
    &self,
    interpreter: &mut Interpreter,
    arguments: Vec<LiteralValue>,
  ) -> Result<LiteralValue, RuntimeErr>;
}

impl std::fmt::Display for dyn TheoCallable {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "<fn {}>", self.name())
  }
}

#[derive(Debug)]
pub struct TheoFn {
  _name: String,
  params: Vec<Token>,
  body: Vec<Stmt>,
}

impl TheoCallable for TheoFn {
  fn name(&self) -> &String {
    &self._name
  }

  fn arity(&self) -> usize {
    self.params.len()
  }

  fn call(
    &self,
    interpreter: &mut Interpreter,
    arguments: Vec<LiteralValue>,
  ) -> Result<LiteralValue, RuntimeErr> {
    interpreter.environment_stack.push();

    // add function arguments to scope
    for (idx, param) in self.params.iter().enumerate() {
      interpreter
        .environment_stack
        .define(&param.lexeme, arguments[idx].clone());
    }

    interpreter.execute_block(&self.body)?;

    interpreter.environment_stack.push();

    // TODO - ? return values ?
    Ok(LiteralValue::Nil)
  }
}

impl TheoFn {
  pub fn new(name: String, params: Vec<Token>, body: Vec<Stmt>) -> Self {
    Self {
      _name: name,
      params,
      body,
    }
  }
}
