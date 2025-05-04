use std::collections::HashMap;

use crate::{
  interpreter::{runtime_err, RuntimeErr},
  literals::LiteralValue,
  tokens::Token,
};

pub struct Environment {
  enclosing: Option<Box<Environment>>,
  values: HashMap<String, LiteralValue>,
}

impl Environment {
  pub fn new(enclosing: Option<Box<Environment>>) -> Self {
    Self {
      enclosing,
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
    } else if let Some(enclosing) = &mut self.enclosing {
      enclosing.assign(name, value)
    } else {
      runtime_err(format!("Undefined variable: {}", name.lexeme))
    }
  }

  pub fn get(&self, name: &Token) -> Result<LiteralValue, RuntimeErr> {
    if let Some(value) = self.values.get(&name.lexeme) {
      Ok(value.clone())
    } else if let Some(enclosing) = &self.enclosing {
      enclosing.get(name)
    } else {
      runtime_err(format!("Undefined variable: {}", name.lexeme))
    }
  }
}
