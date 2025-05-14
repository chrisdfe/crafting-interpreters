use std::fmt::Debug;

use crate::{
  interpreter::{Interpreter, RuntimeErr},
  literals::{LiteralValue, LiteralValueType},
  statements::Stmt,
  tokens::Token,
};

pub type CallFn =
  fn(&Callable, &mut Interpreter, Vec<LiteralValue>) -> Result<LiteralValue, RuntimeErr>;

#[derive(Debug, PartialEq)]
pub enum CallableBody {
  Builtin,
  Statements(Vec<Stmt>),
}

#[derive(Debug, PartialEq)]
pub struct CallableParam {
  pub name: String,
  pub value_type: LiteralValueType,
}

impl CallableParam {
  pub fn new(name: String, value_type: LiteralValueType) -> Self {
    Self { name, value_type }
  }
}

#[derive(Debug, PartialEq)]
pub struct Callable {
  pub name: &'static str,
  // TODO - using this type here might make error reporting worse
  pub params: Vec<CallableParam>,
  pub body: CallableBody,
  pub closure_idx: Option<usize>,
  pub call: CallFn,
}

// pub trait BeaCallable: Debug {
//   fn name(&self) -> String;
//   fn arity(&self) -> usize;
//   fn call(
//     &self,
//     interpreter: &mut Interpreter,
//     arguments: Vec<LiteralValue>,
//   ) -> Result<LiteralValue, RuntimeErr>;
// }

// impl std::fmt::Display for dyn BeaCallable {
//   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//     write!(f, "<fn {}>", self.name())
//   }
// }
