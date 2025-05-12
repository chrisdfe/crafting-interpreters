use std::collections::HashMap;

use crate::{interpreter::RuntimeErr, literals::LiteralValue, tokens::Token};

pub type EnvironmentIdx = usize;

#[derive(Debug)]
pub struct Environment {
  pub parent_idx: Option<EnvironmentIdx>,
  // TODO - this feels dumb, but I can't derive PartialEq because of LiteralValue::Fn
  pub idx: usize,
  values: HashMap<String, LiteralValue>,
}

impl Environment {
  pub fn new(idx: usize, parent_idx: Option<usize>) -> Self {
    Self {
      idx,
      parent_idx,
      values: HashMap::new(),
    }
  }

  pub fn define(&mut self, name: &str, value: LiteralValue) {
    self.values.insert(name.to_string().clone(), value);
  }

  pub fn can_assign(&self, name: &Token) -> bool {
    self.values.contains_key(&name.lexeme)
  }

  pub fn assign(&mut self, name: &Token, value: LiteralValue) -> Result<&LiteralValue, RuntimeErr> {
    if !self.can_assign(name) {
      return Err(RuntimeErr::new(format!(
        "Unable to assign variable: {} value: {}",
        name.lexeme, value
      )));
    }

    if self.values.insert(name.lexeme.clone(), value).is_some() {
      Ok(self.values.get(&name.lexeme).unwrap())
    } else {
      Err(RuntimeErr::new(format!(
        "Unable to assign variable: {} value: {}",
        name.lexeme, name.literal
      )))
    }
  }

  pub fn get(&self, name: &Token) -> Option<&LiteralValue> {
    self.values.get(&name.lexeme)
  }
}
