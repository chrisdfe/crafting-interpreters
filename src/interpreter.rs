use crate::{
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
    Self { message }
  }
}

pub fn runtime_err(message: String) -> Result<LiteralValue, RuntimeErr> {
  Err(RuntimeErr::new(message))
}

fn parse_floats_from_binary_expr(
  left: &LiteralValue,
  right: &LiteralValue,
) -> Result<(f32, f32), RuntimeErr> {
  let left_as_float = match left.cast_float() {
    Err(err) => return Err(RuntimeErr::new(err)),
    Ok(f) => f,
  };

  let right_as_float = match right.cast_float() {
    Err(err) => return Err(RuntimeErr::new(err)),
    Ok(f) => f,
  };

  Ok((left_as_float, right_as_float))
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
  environment_stack: EnvironmentStack,
}

impl Interpreter {
  pub fn new() -> Self {
    Self {
      environment_stack: EnvironmentStack::new(),
    }
  }

  pub fn interpret(&mut self, stmts: Vec<Stmt>) -> Result<(), RuntimeErr> {
    for stmt in stmts.iter() {
      match self.execute_stmt(stmt) {
        Ok(_) => (),
        Err(err) => return Err(err),
      };
    }

    Ok(())
  }

  fn execute_stmt(&mut self, stmt: &Stmt) -> Result<(), RuntimeErr> {
    use Stmt::*;
    match stmt {
      Block(statements) => {
        //
        self.execute_block(statements)?;
        Ok(())
      }

      Expr(expr) => match self.evaluate_expr(expr) {
        Err(err) => Err(err),
        Ok(_) => Ok(()),
      },

      If(cond, then_branch, else_branch) => {
        let value = self.evaluate_expr(cond)?;
        if value.is_truthy() {
          self.execute_stmt(then_branch)?;
        } else if let Some(else_branch) = else_branch {
          self.execute_stmt(else_branch)?;
        };

        Ok(())
      }

      Print(expr) => {
        let value = self.evaluate_expr(expr)?;

        println!("{}", value.cast_string());

        Ok(())
      }

      Var(name, initializer) => {
        let value = match initializer {
          Some(expr) => self.evaluate_expr(expr)?,
          None => LiteralValue::Nil,
        };

        self.environment_stack.define(&name.lexeme, value);

        //
        Ok(())
      }

      While(condition, body) => {
        while (self.evaluate_expr(condition))?.is_truthy() {
          self.execute_stmt(body)?;
        }

        Ok(())
      }

      // name, params, body
      Function(name, params, body) => {
        //
        Ok(())
      }
    }
  }

  fn execute_block(&mut self, statements: &Vec<Stmt>) -> Result<(), RuntimeErr> {
    //
    self.environment_stack.push();

    //
    for statement in statements {
      self.execute_stmt(statement)?;
    }

    let _ = self.environment_stack.pop();
    Ok(())
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
        self.environment_stack.assign(name, value).cloned()
      }

      Grouping(expr) => self.evaluate_expr(expr),

      Unary(operator, right) => self.evaluate_unary_expr(operator, right),

      Binary(left, operator, right) => self.evaluate_binary_expr(left, operator, right),

      Variable(value) => self.environment_stack.get(value).cloned(),

      Call(callee, paren, arguments) => {
        let callee = self.evaluate_expr(callee)?;

        todo!()
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
        Err(_) => runtime_err(format!(
          "Cannot apply unary operator '-' to token {}",
          right
        )),
        Ok(v) => Ok(LiteralValue::Num(-v)),
      },
      Bang => Ok(right.to_inverse_bool_literal_value()),
      o => runtime_err(format!("Unexpected unary operator: {:?}", o)),
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
        let (left, right) = match parse_floats_from_binary_expr(&left, &right) {
          Err(err) => return Err(err),
          Ok((left, right)) => (left, right),
        };

        match &operator.token_type {
          Minus => Ok(LiteralValue::Num(left - right)),
          Star => Ok(LiteralValue::Num(left * right)),
          Slash => Ok(LiteralValue::Num(left / right)),
          t => runtime_err(format!("unexpected token type: {:?}", t)),
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

        let (left, right) = match parse_floats_from_binary_expr(&left, &right) {
          Err(err) => return Err(err),
          Ok((left, right)) => (left, right),
        };

        Ok(LiteralValue::Num(left + right))
      }
      Greater | GreaterEqual | Less | LessEqual => {
        let (left, right) = match parse_floats_from_binary_expr(&left, &right) {
          Err(err) => return Err(err),
          Ok((left, right)) => (left, right),
        };

        let value = match &operator.token_type {
          Greater => left > right,
          GreaterEqual => left >= right,
          Less => left < right,
          LessEqual => left <= right,
          _ => return runtime_err(format!("Unexpected token type: {:?}", &operator.token_type)),
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
        let value = left.equals(&right);
        Ok(LiteralValue::from(!value))
      }
      t => runtime_err(format!(
        "Unexpected operator in binary expression: '{:?}'",
        t
      )),
    }
  }
}
