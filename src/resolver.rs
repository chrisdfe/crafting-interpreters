use std::collections::HashMap;

use crate::{expressions::Expr, interpreter::Interpreter, statements::Stmt, tokens::Token};

pub struct ResolverErr {
  pub message: String,
}

impl ResolverErr {
  // TODO - token as well
  pub fn new(message: String) -> Self {
    Self { message }
  }
}

pub type Scope = HashMap<String, bool>;

pub struct Resolver {
  scopes: Vec<Scope>,
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
    match statement {
      Stmt::Var(name, initializer) => {
        self.declare(name);

        if let Some(initializer) = initializer {
          self.resolve_expression(initializer);
        }

        self.define(name);
      }
      _ => (),
    }
  }

  pub fn resolve_expression(&mut self, expression: &Expr) -> Result<(), ResolverErr> {
    match expression {
      Expr::Variable(name) => {
        if let Some(scope) = self.scopes.last() {
          if let Some(value) = scope.get(&name.lexeme) {
            if *value == false {
              return Err(ResolverErr::new(
                "Can't read local variable in its own initializer.".to_string(),
              ));
            }
          }

          todo!("resolveLocal")
        }
        todo!()
      }
      _ => Ok(()),
    }
  }

  fn begin_scope(&mut self) {
    self.scopes.push(HashMap::new())
  }

  fn end_scope(&mut self) {
    self.scopes.pop();
  }

  fn declare(&mut self, name: &Token) {
    if let Some(scope) = self.scopes.last_mut() {
      scope.insert(name.lexeme.clone(), false);
    }
  }

  fn define(&mut self, name: &Token) {
    if let Some(scope) = self.scopes.last_mut() {
      // TODO - dont unwrap, return ResolverErr or whatever
      *scope.get_mut(&name.lexeme).unwrap() = true;
    }
  }
}
