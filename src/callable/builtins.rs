use crate::{
  interpreter::{Interpreter, RuntimeErr},
  literals::LiteralValue,
};

use super::callable::BeaCallable;

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
