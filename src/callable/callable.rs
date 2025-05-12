use std::fmt::Debug;

use crate::{
  interpreter::{Interpreter, RuntimeErr},
  literals::LiteralValue,
};

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
