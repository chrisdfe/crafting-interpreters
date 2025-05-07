use crate::{
  interpreter::{Interpreter, RuntimeErr},
  literals::LiteralValue,
  statements::Stmt,
  tokens::Token,
};
use std::fmt::Debug;

pub trait BeaCallable: Debug {
  fn name(&self) -> String;
  fn arity(&self) -> usize;
  fn call(
    &self,
    interpreter: &mut Interpreter,
    arguments: Vec<LiteralValue>,
  ) -> Result<LiteralValue, RuntimeErr>;
}

impl std::fmt::Display for dyn BeaCallable {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "<fn {}>", self.name())
  }
}

#[derive(Debug)]
pub struct BeaFn {
  _name: String,
  params: Vec<Token>,
  body: Vec<Stmt>,
}

impl BeaCallable for BeaFn {
  fn name(&self) -> String {
    self._name.clone()
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

    interpreter.environment_stack.pop();

    // TODO - ? return values ?
    Ok(LiteralValue::Nil)
  }
}

impl BeaFn {
  pub fn new(name: String, params: Vec<Token>, body: Vec<Stmt>) -> Self {
    Self {
      _name: name,
      params,
      body,
    }
  }
}

#[derive(Debug)]
pub struct BeaBuiltinPrintln;
impl BeaCallable for BeaBuiltinPrintln {
  fn name(&self) -> String {
    "println".to_string()
  }

  fn arity(&self) -> usize {
    1
  }

  fn call(
    &self,
    interpreter: &mut Interpreter,
    arguments: Vec<LiteralValue>,
  ) -> Result<LiteralValue, RuntimeErr> {
    println!("{}", arguments[0].cast_string());
    Ok(LiteralValue::Nil)
  }
}
