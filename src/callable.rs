use crate::{
  control_flow::BeaControlFlow,
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
  pub closure_idx: usize,
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
    let prev_head = interpreter.environment_stack.get_head_idx();

    interpreter.environment_stack.push_child(self.closure_idx)?;

    // add function arguments to scope
    for (idx, param) in self.params.iter().enumerate() {
      interpreter
        .environment_stack
        .define_at_head(&param.lexeme, arguments[idx].clone())?;
    }

    let value = match interpreter.execute_block(&self.body)? {
      BeaControlFlow::Return(value) => value,
      BeaControlFlow::Continue => LiteralValue::Nil,
    };

    interpreter.environment_stack.set_head_idx(prev_head);

    Ok(value)
  }
}

impl BeaFn {
  pub fn new(name: String, params: Vec<Token>, body: Vec<Stmt>, closure_idx: usize) -> Self {
    Self {
      _name: name,
      params,
      body,
      closure_idx,
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
