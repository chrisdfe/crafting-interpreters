use std::rc::Rc;

use crate::{
  callable::{self, BeaFn},
  control_flow::BeaControlFlow,
  environments::EnvironmentStack,
  expressions::Expr,
  literals::LiteralValue,
  statements::Stmt,
  tokens::{Token, TokenType},
};

#[derive(Debug)]
pub struct RuntimeErr {
  pub message: String,
}

impl RuntimeErr {
  pub fn new(message: String) -> Self {
    let message = format!("Runtime Error: {}", message);

    Self { message }
  }
}

fn is_string_literal(literal: &LiteralValue) -> bool {
  matches!(literal, LiteralValue::Str(_))
}

pub struct Interpreter {
  // Note - this differs from the book's implementation (1) a bit by
  //        using a stack we push/pop environments on to/off of,
  //        instead of Environment being a recursive type
  //        and when we create a new environment having to save the previous environment as a local variable (2)
  //        partially because the latter is kind of annoying to do in rust and
  //        partially because it just seems clearer & more like the correct data structure for this.
  //        I might have to change this when I get into functions
  //        (1) https://craftinginterpreters.com/statements-and-state.html#nesting-and-shadowing
  //        (2) https://craftinginterpreters.com/statements-and-state.html#block-syntax-and-semantics
  pub environment_stack: EnvironmentStack,
  current_statement: Option<Stmt>,
}

impl Interpreter {
  pub fn new() -> Self {
    Self {
      environment_stack: EnvironmentStack::new(),
      current_statement: None,
    }
  }

  pub fn interpret(&mut self, stmts: Vec<Stmt>) -> Result<(), RuntimeErr> {
    // First - define globals
    self.environment_stack.define_at_head(
      "println",
      LiteralValue::Fn(Rc::new(callable::builtins::BeaBuiltinPrintln)),
    )?;

    // Next - start interpreting
    for stmt in stmts.iter() {
      self.current_statement = Some(stmt.clone());
      match self.execute_stmt(stmt) {
        Ok(ctrl) => {
          if let BeaControlFlow::Return(value) = ctrl {
            return Ok(());
          }
        }
        Err(err) => return Err(err),
      };
    }

    Ok(())
  }

  pub fn resolve(&mut self, expression: &Expr, distance: usize) {
    todo!()
  }

  // Note - the caller is responsible for pushing to/popping off of the environment stack
  pub fn execute_block(&mut self, statements: &Vec<Stmt>) -> Result<BeaControlFlow, RuntimeErr> {
    for statement in statements {
      match self.execute_stmt(statement)? {
        BeaControlFlow::Continue => (),
        BeaControlFlow::Return(value) => {
          return Ok(BeaControlFlow::Return(value));
        }
      }
    }

    Ok(BeaControlFlow::Continue)
  }

  fn execute_stmt(&mut self, stmt: &Stmt) -> Result<BeaControlFlow, RuntimeErr> {
    use Stmt::*;
    match stmt {
      Block(statements) => {
        //
        self.environment_stack.push();

        let value = self.execute_block(statements)?;

        self.environment_stack.pop()?;

        Ok(value)
      }

      Expr(expr) => match self.evaluate_expr(expr) {
        Err(err) => Err(err),
        Ok(_) => Ok(BeaControlFlow::Continue),
      },

      If(cond, then_branch, else_branch) => {
        let value = self.evaluate_expr(cond)?;
        let value = if value.is_truthy() {
          self.execute_stmt(then_branch)?
        } else if let Some(else_branch) = else_branch {
          self.execute_stmt(else_branch)?
        } else {
          BeaControlFlow::Continue
        };

        Ok(value)
      }

      Return(token, value) => {
        let value = if let Some(value) = value {
          self.evaluate_expr(value)?
        } else {
          LiteralValue::Nil
        };

        Ok(BeaControlFlow::Return(value))
      }

      Var(name, initializer) => {
        let value = match initializer {
          Some(expr) => self.evaluate_expr(expr)?,
          None => LiteralValue::Nil,
        };

        self.environment_stack.define_at_head(&name.lexeme, value)?;

        //
        Ok(BeaControlFlow::Continue)
      }

      While(condition, body) => {
        while (self.evaluate_expr(condition))?.is_truthy() {
          if let BeaControlFlow::Return(value) = self.execute_stmt(body)? {
            return Ok(BeaControlFlow::Return(value));
          }
        }

        Ok(BeaControlFlow::Continue)
      }

      Function(name, params, body) => {
        let head_idx = self.environment_stack.get_head_idx();

        // The closure is a child environment to the current one
        let closure_idx = self.environment_stack.add_detached_child(head_idx)?;

        let fn_definition = BeaFn::new(name.clone(), params.clone(), body.clone(), closure_idx);

        let fn_literal = LiteralValue::Fn(Rc::new(fn_definition));

        self.environment_stack.define_at_head(name, fn_literal)?;

        Ok(BeaControlFlow::Continue)
      }
    }
  }

  fn evaluate_expr(&mut self, expr: &Expr) -> Result<LiteralValue, RuntimeErr> {
    use Expr::*;
    match &expr {
      Literal(value) => Ok(value.clone()),

      Logical(left, operator, right) => {
        //
        let left = self.evaluate_expr(left)?;
        if operator.token_type == TokenType::Or {
          if left.is_truthy() {
            return Ok(left);
          }
        } else if !left.is_truthy() {
          return Ok(left);
        }

        self.evaluate_expr(right)
      }

      Assign(name, expr) => {
        let value = self.evaluate_expr(expr)?;
        self.environment_stack.assign_at_head(name, value.clone())?;
        Ok(value)
      }

      Grouping(expr) => self.evaluate_expr(expr),

      Unary(operator, right) => self.evaluate_unary_expr(operator, right),

      Binary(left, operator, right) => self.evaluate_binary_expr(left, operator, right),

      Variable(name) => {
        let maybe_value = self.environment_stack.get_value_at_head(name)?.cloned();

        let value = if let Some(value) = maybe_value {
          value.clone()
        } else {
          LiteralValue::Nil
        };

        Ok(value)
      }

      Call(callee, paren, arguments) => {
        let callee = self.evaluate_expr(callee)?;

        let callable = match callee {
          LiteralValue::Fn(callable) => callable,
          _ => return Err(self.runtime_err(format!("Invalid fn callee: {}", callee))),
        };

        if arguments.len() != callable.arity() {
          return Err(self.runtime_err(format!(
            "Expected {} arguments but got {}.",
            callable.arity(),
            arguments.len()
          )));
        }

        let evaluated_arguments = arguments
          .iter()
          .map(|arg| self.evaluate_expr(arg))
          .collect::<Result<Vec<LiteralValue>, RuntimeErr>>()?;

        let return_value = callable.call(self, evaluated_arguments)?;

        Ok(return_value)
      }
    }
  }

  fn evaluate_unary_expr(
    &mut self,
    operator: &Token,
    right: &Box<Expr>,
  ) -> Result<LiteralValue, RuntimeErr> {
    let right = self.evaluate_expr(right)?;

    use TokenType::*;
    match &operator.token_type {
      Minus => match right.cast_float() {
        Err(_) => Err(self.runtime_err(format!(
          "Cannot apply unary operator '-' to token {}",
          right
        ))),
        Ok(v) => Ok(LiteralValue::Num(-v)),
      },
      Bang => Ok(right.to_inverse_bool_literal_value()),
      o => Err(self.runtime_err(format!("Unexpected unary operator: {:?}", o))),
    }
  }

  fn evaluate_binary_expr(
    &mut self,
    left: &Box<Expr>,
    operator: &Token,
    right: &Box<Expr>,
  ) -> Result<LiteralValue, RuntimeErr> {
    let left = self.evaluate_expr(left)?;
    let right = self.evaluate_expr(right)?;

    use TokenType::*;
    match &operator.token_type {
      Minus | Star | Slash => {
        let (left, right) = self.parse_floats_from_binary_expr(&left, &right)?;

        match &operator.token_type {
          Minus => Ok(LiteralValue::Num(left - right)),
          Star => Ok(LiteralValue::Num(left * right)),
          Slash => Ok(LiteralValue::Num(left / right)),
          t => Err(self.runtime_err(format!("unexpected token type: {:?}", t))),
        }
      }

      Plus => {
        // attempt string addition
        if is_string_literal(&left) || is_string_literal(&right) {
          //
          let left = &left.cast_string();
          let right = &right.cast_string();

          let value = format!("{}{}", left, right);
          return Ok(LiteralValue::Str(value));
        }

        // fall back to number
        let (left, right) = self.parse_floats_from_binary_expr(&left, &right)?;

        Ok(LiteralValue::Num(left + right))
      }

      Greater | GreaterEqual | Less | LessEqual => {
        let (left, right) = self.parse_floats_from_binary_expr(&left, &right)?;

        let value = match &operator.token_type {
          Greater => left > right,
          GreaterEqual => left >= right,
          Less => left < right,
          LessEqual => left <= right,
          _ => {
            return Err(
              self.runtime_err(format!("Unexpected token type: {:?}", &operator.token_type)),
            )
          }
        };

        let literal_value = if value {
          LiteralValue::True
        } else {
          LiteralValue::False
        };

        Ok(literal_value)
      }

      EqualEqual => {
        let value = left.equals(&right);
        Ok(LiteralValue::from(value))
      }

      BangEqual => {
        let value = !left.equals(&right);
        Ok(LiteralValue::from(value))
      }

      t => Err(self.runtime_err(format!(
        "Unexpected operator in binary expression: '{:?}'",
        t
      ))),
    }
  }

  fn parse_floats_from_binary_expr(
    &self,
    left: &LiteralValue,
    right: &LiteralValue,
  ) -> Result<(f32, f32), RuntimeErr> {
    let left_as_float = match left.cast_float() {
      Err(err) => return Err(self.runtime_err(err)),
      Ok(f) => f,
    };

    let right_as_float = match right.cast_float() {
      Err(err) => return Err(self.runtime_err(err)),
      Ok(f) => f,
    };

    Ok((left_as_float, right_as_float))
  }

  fn runtime_err(&self, message: String) -> RuntimeErr {
    let message = format!("{}\nat statement {:#?}", message, &self.current_statement);
    // let message = format!("{}\nenvironment: {:#?}", message, &self.environment_stack);
    RuntimeErr::new(message)
  }
}
