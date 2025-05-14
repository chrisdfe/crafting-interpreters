use crate::{
  control_flow::BeaControlFlow,
  interpreter::{Interpreter, RuntimeErr},
  literals::LiteralValue,
  statements::Stmt,
};

use super::callable::{Callable, CallableBody, CallableParam};

pub fn create_bea_fn(
  name: &'static str,
  params: Vec<CallableParam>,
  body: Vec<Stmt>,
  closure_idx: Option<usize>,
) -> Callable {
  Callable {
    name,
    params,
    closure_idx,
    body: CallableBody::Statements(body),
    call: call_bea_fn,
  }
}

fn call_bea_fn(
  callable: &Callable,
  interpreter: &mut Interpreter,
  arguments: Vec<LiteralValue>,
) -> Result<LiteralValue, RuntimeErr> {
  let prev_head = interpreter.environment_stack.get_head_idx();

  let closure_idx = match callable.closure_idx {
    Some(closure_idx) => closure_idx,
    None => {
      return Err(RuntimeErr::new(format!(
        "Unable to call Fn with empty closure index: {}",
        callable.name
      )))
    }
  };

  let body = match &callable.body {
    CallableBody::Statements(body) => body,
    CallableBody::Builtin => {
      return Err(RuntimeErr::new(format!(
        "Expected list of statements in callable body, found Builtin in: {}",
        callable.name
      )))
    }
  };

  // TODO - here - verity the correct number and type of arguments have been passed in

  interpreter.environment_stack.push_child(closure_idx)?;

  // add function arguments to scope
  for (idx, param) in callable.params.iter().enumerate() {
    interpreter
      .environment_stack
      .define_at_head(&param.name, arguments[idx].clone())?;
  }

  let value = match interpreter.execute_block(&body)? {
    BeaControlFlow::Return(value) => value,
    BeaControlFlow::Continue => LiteralValue::Nil,
  };

  interpreter.environment_stack.set_head_idx(prev_head);

  Ok(value)
}
