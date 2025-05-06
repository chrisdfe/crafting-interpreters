use std::collections::HashMap;

use crate::{interpreter::RuntimeErr, literals::LiteralValue, tokens::Token};

pub struct Environment {
  values: HashMap<String, LiteralValue>,
}

impl Environment {
  fn new() -> Self {
    Self {
      values: HashMap::new(),
    }
  }

  fn define(&mut self, name: &str, value: LiteralValue) {
    self.values.insert(name.to_string().clone(), value);
  }

  fn can_assign(&self, name: &Token) -> bool {
    self.values.contains_key(&name.lexeme)
  }

  fn assign(&mut self, name: &Token, value: LiteralValue) -> Result<&LiteralValue, RuntimeErr> {
    if !self.can_assign(name) {
      return Err(RuntimeErr::new(format!(
        "Unable to assign variable: {} value: {}",
        name.lexeme, name.literal
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

  fn get(&self, name: &Token) -> Option<&LiteralValue> {
    self.values.get(&name.lexeme)
  }
}

pub struct EnvironmentStack {
  stack: Vec<Environment>,
}

impl EnvironmentStack {
  pub fn new() -> Self {
    // this first env = global env
    Self {
      stack: vec![Environment::new()],
    }
  }

  pub fn push(&mut self) -> &Environment {
    self.stack.push(Environment::new());
    // unwrap is safe here because we literally just added this env
    self.stack.last().unwrap()
  }

  pub fn pop(&mut self) -> Option<Environment> {
    // Prevent popping the global env off
    if self.stack.len() > 1 {
      self.stack.pop()
    } else {
      None
    }
  }

  pub fn define(&mut self, name: &str, value: LiteralValue) {
    let current = self.current_mut().unwrap();
    current.define(name, value);
  }

  pub fn define_global(&mut self, name: &str, value: LiteralValue) {
    let globals = self.globals_mut().unwrap();
    globals.define(name, value);
  }

  pub fn assign(&mut self, name: &Token, value: LiteralValue) -> Result<&LiteralValue, RuntimeErr> {
    for env in self.stack.iter_mut().rev() {
      if env.can_assign(name) {
        env.assign(name, value).unwrap();
        return Ok(env.get(name).unwrap());
      }
    }

    Err(RuntimeErr::new(format!(
      "Undefined variable: {}",
      name.lexeme
    )))
  }

  pub fn assign_global(
    &mut self,
    name: &Token,
    value: LiteralValue,
  ) -> Result<&LiteralValue, RuntimeErr> {
    let globals = self.globals_mut()?;
    globals.assign(name, value)?;
    Ok(globals.get(name).unwrap())
  }

  pub fn globals_mut(&mut self) -> Result<&mut Environment, RuntimeErr> {
    if let Some(env) = self.stack.first_mut() {
      Ok(env)
    } else {
      Err(RuntimeErr::new(String::from(
        "Somehow ended up with no global environment.",
      )))
    }
  }

  pub fn current_mut(&mut self) -> Result<&mut Environment, RuntimeErr> {
    if let Some(env) = self.stack.last_mut() {
      Ok(env)
    } else {
      Err(RuntimeErr::new(String::from(
        "Somehow ended up with an empty environment stack.",
      )))
    }
  }

  pub fn get(&self, name: &Token) -> Result<&LiteralValue, RuntimeErr> {
    //
    for env in self.stack.iter().rev() {
      if let Some(value) = env.get(name) {
        return Ok(value);
      }
    }

    Err(RuntimeErr::new(format!(
      "Undefined variable: '{}'",
      &name.lexeme
    )))
  }
}
