use std::collections::HashMap;

use crate::{interpreter::Interpreter, statements::Stmt};

pub struct Resolver {
  scopes: Vec<HashMap<String, bool>>,
}

impl Resolver {
  pub fn resolve_block_statement(&mut self, statement: &Stmt) {
    let statements = match statement {
      Stmt::Block(statements) => statements,
      _ => return,
    };

    self.begin_scope();

    for statement in statements.iter() {
      self.resolve_statement(statement);
    }

    self.end_scope();
  }

  pub fn resolve_statement(&mut self, statement: &Stmt) {
    //
    todo!()
  }

  pub fn resolve_expression(&mut self, interpreter: &mut Interpreter) {
    //
    todo!()
  }

  fn begin_scope(&mut self) {
    self.scopes.push(HashMap::new())
  }

  fn end_scope(&mut self) {
    self.scopes.pop();
  }
}
