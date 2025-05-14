use crate::{
  interpreter::{Interpreter, RuntimeErr},
  literals::{LiteralValue, LiteralValueType},
};

use super::callable::{Callable, CallableBody, CallableParam};

// TODO - it would be nice if this was a const but the params Vec makes it difficult
pub fn get_builtin_println() -> Callable {
  Callable {
    name: "println",
    params: vec![CallableParam::new(
      "text".to_string(),
      LiteralValueType::Str,
    )],
    body: CallableBody::Builtin,
    call: call_println,
    closure_idx: None,
  }
}

fn call_println(
  _: &Callable,
  interpreter: &mut Interpreter,
  arguments: Vec<LiteralValue>,
) -> Result<LiteralValue, RuntimeErr> {
  println!("{}", arguments[0].cast_string());
  Ok(LiteralValue::Nil)
}
