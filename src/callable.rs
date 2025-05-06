use crate::{interpreter::Interpreter, literals::LiteralValue};

// TODO - not sure if a trait is what is best here
pub trait Callable: std::fmt::Debug {
  fn arity(&self) -> usize;
  fn call(&self, interpreter: &mut Interpreter, arguments: Vec<LiteralValue>) -> LiteralValue;
}
