use std::collections::HashMap;

use crate::{
  interpreter::{runtime_err, RuntimeErr},
  literals::LiteralValue,
  tokens::Token,
};

pub struct Environment {
  values: HashMap<String, LiteralValue>,
}

impl Environment {
  pub fn new() -> Self {
    Self {
      values: HashMap::new(),
    }
  }

  pub fn define(&mut self, name: String, value: LiteralValue) {
    self.values.insert(name, value);
  }

  pub fn assign(&mut self, name: &Token, value: LiteralValue) -> Result<LiteralValue, RuntimeErr> {
    if self.values.contains_key(&name.lexeme) {
      self.values.insert(name.lexeme.clone(), value.clone());
      Ok(value)
    } else {
      runtime_err(format!("Undefined variable: {}", name.lexeme))
    }
  }

  pub fn get(&self, name: &Token) -> Result<LiteralValue, RuntimeErr> {
    match self.values.get(&name.lexeme) {
      Some(value) => Ok(value.clone()),
      None => runtime_err(format!("Undefined variable: {}", name.lexeme)),
    }
  }
}
