use std::collections::HashMap;

use crate::{interpreter::RuntimeErr, literals::LiteralValue, tokens::Token};

pub type EnvironmentIndex = usize;

#[derive(Debug)]
pub struct Environment {
  parent_idx: Option<EnvironmentIndex>,
  // TODO - this feels dumb, but I can't derive PartialEq because of LiteralValue::Fn
  idx: usize,
  values: HashMap<String, LiteralValue>,
}

impl Environment {
  fn new(idx: usize, parent_idx: Option<usize>) -> Self {
    Self {
      idx,
      parent_idx,
      values: HashMap::new(),
    }
  }

  pub fn get_idx(&self) -> usize {
    self.idx
  }

  pub fn get_parent_idx(&self) -> &Option<usize> {
    &self.parent_idx
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

#[derive(Debug)]
pub struct EnvironmentStack {
  environments: Vec<Option<Environment>>,
  head_idx: usize,
}

impl EnvironmentStack {
  pub fn new() -> Self {
    let global_env = Some(Environment::new(0, None));

    Self {
      environments: vec![global_env],
      head_idx: 0,
    }
  }

  pub fn get_head_idx(&self) -> usize {
    self.head_idx
  }

  pub fn set_head_idx(&mut self, new_head_idx: usize) {
    self.head_idx = new_head_idx;
  }

  pub fn get_parent(&self, env: &Environment) -> Option<&Environment> {
    if let Some(parent_idx) = env.parent_idx {
      if let Some(env) = self.environments.get(parent_idx) {
        if let Some(env) = env {
          return Some(env);
        }
      }
    }

    None
  }

  /// Adds a child environment to parent_idx, but does NOT update head environment
  pub fn add_detached_child(&mut self, parent_idx: usize) -> Result<usize, RuntimeErr> {
    if self.environments.get(parent_idx).is_none() {
      return Err(RuntimeErr::new(format!(
        "unable to add new environment: parent at idx '{}' not found",
        parent_idx
      )));
    }

    let idx = self.environments.len();
    let env = Environment::new(idx, Some(parent_idx));
    // TODO - instead of just pushing here, try finding the first idx in self.environments that is None
    self.environments.push(Some(env));

    Ok(idx)
  }

  /// Pushes a new environment onto the head, with its parent set to parent_idx
  pub fn push_child(&mut self, parent_idx: usize) -> Result<&Environment, RuntimeErr> {
    if self.environments.get(parent_idx).is_none() {
      return Err(RuntimeErr::new(format!(
        "unable to add new environment: parent at idx '{}' not found",
        parent_idx
      )));
    }

    self.push();
    let head = self.get_env_at_head_mut_or_err()?;
    head.parent_idx = Some(parent_idx);

    Ok(self.get_env_by_idx_or_err(self.head_idx)?)
  }

  // Pushes a new environment onto the head
  pub fn push(&mut self) -> &Environment {
    let prev_head_idx = self.head_idx;
    self.head_idx = self.environments.len();
    let env = Environment::new(self.head_idx, Some(prev_head_idx));

    // TODO - instead of just pushing here, try finding the first idx in self.environments that is None
    self.environments.push(Some(env));

    // unwrap is safe here because we literally just added this env
    self.get_env_by_idx_or_err(self.head_idx).unwrap()
  }

  /// Clears head environment & sets head idx to its parent
  pub fn pop(&mut self) -> Result<(), RuntimeErr> {
    // Prevent popping the global env off
    if self.environments.len() > 1 {
      let head_idx = self.head_idx;
      // TODO - is this always going to be correct?
      // set head to env's parent
      let new_head_idx = {
        let env = self.get_env_by_idx_or_err(self.head_idx)?;
        // this will only panic if we're popping the global env, in which case we should panic
        env.parent_idx.unwrap()
      };

      self.remove_at_idx(head_idx)?;

      self.head_idx = new_head_idx;
    }

    Ok(())
  }

  /// Clears (i.e sets to None)
  pub fn remove_at_idx(&mut self, idx: usize) -> Result<(), RuntimeErr> {
    let env = self.get_env_by_idx_or_err(idx)?;
    let idx = env.idx;
    self.environments[idx] = None;

    // Recursively remove children environments as well
    for child_env in self.get_env_idxes_by_parent_idx(idx) {
      self.remove_at_idx(child_env)?;
    }

    Ok(())
  }

  pub fn define_at_idx(
    &mut self,
    idx: usize,
    name: &str,
    value: LiteralValue,
  ) -> Result<(), RuntimeErr> {
    let env = self.get_env_by_idx_mut_or_err(idx)?;
    env.define(name, value);
    Ok(())
  }

  pub fn define_at_head(&mut self, name: &str, value: LiteralValue) -> Result<(), RuntimeErr> {
    self.define_at_idx(self.head_idx, name, value)
  }

  pub fn assign_at_idx(
    &mut self,
    idx: usize,
    name: &Token,
    value: LiteralValue,
  ) -> Result<&LiteralValue, RuntimeErr> {
    if let Some(idx) = self.get_first_assignable_idx(idx, name, &value)? {
      let environment = self.get_env_by_idx_mut_or_err(idx)?;
      environment.assign(name, value)
    } else {
      Err(RuntimeErr::new(format!(
        "Undefined variable: {}",
        name.lexeme
      )))
    }
  }

  pub fn assign_at_head(
    &mut self,
    name: &Token,
    value: LiteralValue,
  ) -> Result<&LiteralValue, RuntimeErr> {
    let idx = self.head_idx;
    self.assign_at_idx(idx, name, value)
  }

  /// Searches for a value, starting with env, and following the tree up
  /// looking for it until we've reached the global namespace.
  /// Only returns a RuntimeErr if environment at idx is None -
  /// if the value isn't found it will Return Ok(None)
  pub fn get_value_in_env(
    &self,
    env_idx: usize,
    name: &Token,
  ) -> Result<Option<&LiteralValue>, RuntimeErr> {
    let env = self.get_env_by_idx_or_err(env_idx)?;
    let value = if let Some(value) = env.get(name) {
      Some(value)
    } else if let Some(parent_idx) = env.parent_idx {
      self.get_value_in_env(parent_idx, name)?
    } else {
      None
    };

    Ok(value)
  }

  pub fn get_value_at_head(&self, name: &Token) -> Result<Option<&LiteralValue>, RuntimeErr> {
    self.get_value_in_env(self.head_idx, name)
  }

  pub fn get_env_by_idx_or_err(&self, idx: usize) -> Result<&Environment, RuntimeErr> {
    if let Some(maybe_env) = self.environments.get(idx) {
      if let Some(env) = maybe_env {
        return Ok(env);
      }
    }

    return Err(RuntimeErr::new(format!(
      "No environment found at idx {}",
      idx
    )));
  }

  pub fn get_env_by_idx_mut_or_err(&mut self, idx: usize) -> Result<&mut Environment, RuntimeErr> {
    if let Some(maybe_env) = self.environments.get_mut(idx) {
      if let Some(env) = maybe_env {
        return Ok(env);
      }
    }

    return Err(RuntimeErr::new(format!(
      "No environment found at idx {}",
      idx
    )));
  }

  fn get_env_at_head_or_err(&self) -> Result<&Environment, RuntimeErr> {
    self.get_env_by_idx_or_err(self.head_idx)
  }

  fn get_env_at_head_mut_or_err(&mut self) -> Result<&mut Environment, RuntimeErr> {
    self.get_env_by_idx_mut_or_err(self.head_idx)
  }

  // follow the environments up the tree, returning the idx of the first one that can_assign the value
  fn get_first_assignable_idx(
    &self,
    idx: usize,
    name: &Token,
    value: &LiteralValue,
  ) -> Result<Option<usize>, RuntimeErr> {
    let environment = self.get_env_by_idx_or_err(idx)?;
    if environment.can_assign(name) {
      Ok(Some(environment.idx))
    } else if let Some(parent_idx) = environment.parent_idx {
      self.get_first_assignable_idx(parent_idx, name, value)
    } else {
      Ok(None)
    }
  }

  fn get_env_idxes_by_parent_idx(&self, target_parent_idx: usize) -> Vec<usize> {
    self
      .environments
      .iter()
      .filter(|maybe_env| maybe_env.is_some())
      // safe because we just filtered out Nones
      .map(|maybe_env| maybe_env.as_ref().unwrap())
      .filter(|env| {
        if let Some(parent_idx) = env.parent_idx {
          parent_idx == target_parent_idx
        } else {
          false
        }
      })
      .map(|env| env.idx)
      .collect()
  }
}
