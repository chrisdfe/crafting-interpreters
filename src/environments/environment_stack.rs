use crate::{interpreter::RuntimeErr, literals::LiteralValue, tokens::Token};

use super::{Environment, EnvironmentIdx};

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

  pub fn get_head_idx(&self) -> EnvironmentIdx {
    self.head_idx
  }

  pub fn set_head_idx(&mut self, new_head_idx: EnvironmentIdx) {
    self.head_idx = new_head_idx;
  }

  /// Adds a child environment to parent_idx but does NOT update head environment
  /// Returns the index of the new environment
  pub fn add_detached_child(&mut self, parent_idx: usize) -> Result<EnvironmentIdx, RuntimeErr> {
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

  /// Pushes a new environment onto the head with its parent set to parent_idx
  /// Returns the index of the new child
  pub fn push_child(&mut self, parent_idx: EnvironmentIdx) -> Result<EnvironmentIdx, RuntimeErr> {
    if self.environments.get(parent_idx).is_none() {
      return Err(RuntimeErr::new(format!(
        "unable to add new environment: parent at idx '{}' not found",
        parent_idx
      )));
    }

    self.push();
    let head = self.get_env_at_head_mut_or_err()?;
    head.parent_idx = Some(parent_idx);

    Ok(self.head_idx)
  }

  /// Pushes a new environment onto the head
  /// Returns the index of the new environment
  pub fn push(&mut self) -> EnvironmentIdx {
    let prev_head_idx = self.head_idx;
    self.head_idx = self.environments.len();
    let env = Environment::new(self.head_idx, Some(prev_head_idx));

    // TODO - instead of just pushing here, try finding the first idx in self.environments that is None
    self.environments.push(Some(env));

    // unwrap is safe here because we literally just added this env
    self.head_idx
  }

  /// Clears head environment & sets head idx to its parent
  pub fn pop(&mut self) -> Result<(), RuntimeErr> {
    // Prevent popping the global env off
    // TODO - keep a tally of how many not-none environments there are
    //        i.e this is going to be wrong most of the time
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
  pub fn remove_at_idx(&mut self, idx: EnvironmentIdx) -> Result<(), RuntimeErr> {
    let env = self.get_env_by_idx_or_err(idx)?;
    let idx = env.idx;
    self.environments[idx] = None;

    // Recursively remove children environments as well
    for child_env in self.get_env_idxes_by_parent_idx(idx) {
      self.remove_at_idx(child_env)?;
    }

    Ok(())
  }

  /// Defines a variable in the env at idx
  pub fn define_at_idx(
    &mut self,
    idx: EnvironmentIdx,
    name: &str,
    value: LiteralValue,
  ) -> Result<(), RuntimeErr> {
    let env = self.get_env_by_idx_mut_or_err(idx)?;
    env.define(name, value);
    Ok(())
  }

  /// Defines a variable at the head environment
  pub fn define_at_head(&mut self, name: &str, value: LiteralValue) -> Result<(), RuntimeErr> {
    self.define_at_idx(self.head_idx, name, value)
  }

  /// Assigns a variable in the env at idx
  pub fn assign_at_idx(
    &mut self,
    idx: EnvironmentIdx,
    name: &Token,
    value: LiteralValue,
  ) -> Result<&LiteralValue, RuntimeErr> {
    if let Some(idx) = self.get_first_assignable_idx(idx, name)? {
      let environment = self.get_env_by_idx_mut_or_err(idx)?;
      environment.assign(name, value)
    } else {
      Err(RuntimeErr::new(format!(
        "Undefined variable: {}",
        name.lexeme
      )))
    }
  }

  /// Assigns a variable at the head env
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
    env_idx: EnvironmentIdx,
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

  /// Searches recursively for a variable
  pub fn get_value_at_head(&self, name: &Token) -> Result<Option<&LiteralValue>, RuntimeErr> {
    self.get_value_in_env(self.head_idx, name)
  }

  pub fn get_env_by_idx_or_err(&self, idx: usize) -> Result<&Environment, RuntimeErr> {
    if let Some(maybe_env) = self.environments.get(idx) {
      if let Some(env) = maybe_env {
        return Ok(env);
      }
    }

    Err(RuntimeErr::new(format!(
      "No environment found at idx {}",
      idx
    )))
  }

  pub fn get_env_by_idx_mut_or_err(&mut self, idx: usize) -> Result<&mut Environment, RuntimeErr> {
    if let Some(maybe_env) = self.environments.get_mut(idx) {
      if let Some(env) = maybe_env {
        return Ok(env);
      }
    }

    Err(RuntimeErr::new(format!(
      "No environment found at idx {}",
      idx
    )))
  }

  fn get_env_at_head_mut_or_err(&mut self) -> Result<&mut Environment, RuntimeErr> {
    self.get_env_by_idx_mut_or_err(self.head_idx)
  }

  // follow the environments up the tree, returning the idx of the first one that can_assign the value
  fn get_first_assignable_idx(
    &self,
    idx: usize,
    name: &Token,
  ) -> Result<Option<usize>, RuntimeErr> {
    let environment = self.get_env_by_idx_or_err(idx)?;
    if environment.can_assign(name) {
      Ok(Some(environment.idx))
    } else if let Some(parent_idx) = environment.parent_idx {
      self.get_first_assignable_idx(parent_idx, name)
    } else {
      Ok(None)
    }
  }

  fn get_env_idxes_by_parent_idx(&self, target_parent_idx: usize) -> Vec<usize> {
    self
      .environments
      .iter()
      .filter_map(|maybe_env| maybe_env.as_ref())
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
