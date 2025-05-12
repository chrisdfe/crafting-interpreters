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
  pub fn resolve_statements(
    &mut self,
    interpreter: &mut Interpreter,
    statements: &Vec<Stmt>,
  ) -> Result<(), ResolverErr> {
    for statement in statements {
      self.resolve_statement(interpreter, statement)?;
    }

    Ok(())
  }

  pub fn resolve_statement(
    &mut self,
    interpreter: &mut Interpreter,
    statement: &Stmt,
  ) -> Result<(), ResolverErr> {
    //
    #[allow(clippy::all)]
    match statement {
      Stmt::Block(statements) => {
        self.begin_scope();
        self.resolve_statements(interpreter, statements)?;
        self.end_scope();
      }

      Stmt::Var(name, initializer) => {
        self.declare(&name.lexeme);

        if let Some(initializer) = initializer {
          self.resolve_expression(interpreter, initializer)?;
        }

        self.define(&name.lexeme);
      }

      Stmt::Function(name, params, body) => {
        self.declare(name);
        self.define(name);
        self.resolve_function(interpreter, statement)?;
      }

      Stmt::Expr(expression) => {
        self.resolve_expression(interpreter, expression)?;
      }

      Stmt::If(condition, then_branch, else_branch) => {
        self.resolve_expression(interpreter, condition)?;
        self.resolve_statement(interpreter, then_branch)?;

        if let Some(else_branch) = else_branch {
          self.resolve_statement(interpreter, else_branch)?;
        }
      }

      Stmt::Return(keyword, value) => {
        if let Some(value) = value {
          self.resolve_expression(interpreter, value)?;
        }
      }

      Stmt::While(condition, body) => {
        self.resolve_expression(interpreter, condition)?;
        self.resolve_statement(interpreter, body)?;
      }
    }

    Ok(())
  }

  pub fn resolve_expression(
    &mut self,
    interpreter: &mut Interpreter,
    expression: &Expr,
  ) -> Result<(), ResolverErr> {
    match expression {
      Expr::Variable(name) => {
        if let Some(scope) = self.scopes.last() {
          if let Some(value) = scope.get(&name.lexeme) {
            if !(*value) {
              return Err(ResolverErr::new(
                "Can't read local variable in its own initializer.".to_string(),
              ));
            }
          }

          self.resolve_local(interpreter, expression, name);
        }
        todo!()
      }

      Expr::Assign(name, value) => {
        self.resolve_expression(interpreter, value)?;
        self.resolve_local(interpreter, expression, name);
      }

      Expr::Binary(left, operator, right) => {
        self.resolve_expression(interpreter, left)?;
        self.resolve_expression(interpreter, right)?;
      }

      Expr::Call(callee, _, arguments) => {
        self.resolve_expression(interpreter, callee)?;

        for argument in arguments.iter() {
          self.resolve_expression(interpreter, argument)?;
        }
      }

      Expr::Grouping(expression) => {
        self.resolve_expression(interpreter, expression)?;
      }

      Expr::Literal(value) => (),

      Expr::Logical(left, _, right) => {
        self.resolve_expression(interpreter, left)?;
        self.resolve_expression(interpreter, right)?;
      }

      Expr::Unary(_, right) => {
        self.resolve_expression(interpreter, right)?;
      }
    }

    Ok(())
  }

  fn resolve_local(&mut self, interpreter: &mut Interpreter, expression: &Expr, name: &Token) {
    for i in self.scopes.len() - 1..0 {
      let scope = self.scopes.get(i).unwrap();
      if scope.contains_key(&name.lexeme) {
        interpreter.resolve(expression, self.scopes.len() - 1 - i);
      }
    }
  }

  fn resolve_function(
    &mut self,
    interpreter: &mut Interpreter,
    function: &Stmt,
  ) -> Result<(), ResolverErr> {
    let (name, params, body) = match function {
      Stmt::Function(name, params, body) => (name, params, body),
      _ => {
        return Err(ResolverErr::new(
          "Statement was exptect to be a function but was not.".to_string(),
        ))
      }
    };

    self.begin_scope();

    for param in params {
      //
      self.declare(&param.lexeme);
      self.define(&param.lexeme);
    }

    self.resolve_statements(interpreter, body)?;
    self.end_scope();
    Ok(())
  }

  fn begin_scope(&mut self) {
    self.scopes.push(HashMap::new())
  }

  fn end_scope(&mut self) {
    self.scopes.pop();
  }

  fn declare(&mut self, name: &String) {
    if let Some(scope) = self.scopes.last_mut() {
      scope.insert(name.clone(), false);
    }
  }

  fn define(&mut self, name: &String) {
    if let Some(scope) = self.scopes.last_mut() {
      // TODO - dont unwrap, return ResolverErr or whatever
      *scope.get_mut(name).unwrap() = true;
    }
  }
}
