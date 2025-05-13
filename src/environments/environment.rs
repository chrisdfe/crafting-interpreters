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

  pub fn can_assign(&self, name: &str) -> bool {
    self.values.contains_key(name)
  }

  pub fn assign(&mut self, name: &str, value: LiteralValue) -> Result<&LiteralValue, RuntimeErr> {
    if !self.can_assign(&name) {
      return Err(RuntimeErr::new(format!(
        "Unable to assign variable: {} value: {}",
        name, value
      )));
    }

    let err_value = value.clone();

    if self
      .values
      .insert(name.to_string().clone(), value)
      .is_some()
    {
      Ok(self.values.get(&name.to_string()).unwrap())
    } else {
      Err(RuntimeErr::new(format!(
        "Unable to assign variable: {} value: {}",
        name, err_value
      )))
    }
  }

  pub fn get(&self, name: &str) -> Option<&LiteralValue> {
    self.values.get(name)
  }
}
