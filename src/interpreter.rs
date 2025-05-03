use crate::{expressions::Expr, literals::LiteralValue, tokens::TokenType};

struct EvalErr {
  message: String,
}

impl EvalErr {
  fn new(message: String) -> Self {
    Self { message }
  }
}

fn eval_err(message: String) -> Result<LiteralValue, EvalErr> {
  Err(EvalErr::new(message))
}

fn parse_floats_from_binary_expr(left: &Expr, right: &Expr) -> Result<(f32, f32), EvalErr> {
  let left_as_literal = match left {
    Expr::Literal(left) => left,
    // TODO - more descriptive error message
    _ => return Err(EvalErr::new(String::from("Invalid left-hand operand type"))),
  };

  let right_as_literal = match right {
    Expr::Literal(right) => right,
    // TODO - more descriptive error message
    _ => return Err(EvalErr::new(String::from("Invalid left-hand operand type"))),
  };

  let left_as_float = match left_as_literal.cast_float() {
    Err(err) => return Err(EvalErr::new(err)),
    Ok(f) => f,
  };

  let right_as_float = match right_as_literal.cast_float() {
    Err(err) => return Err(EvalErr::new(err)),
    Ok(f) => f,
  };

  Ok((left_as_float, right_as_float))
}

struct Interpreter {}

// I left off here:
// https://craftinginterpreters.com/evaluating-expressions.html#evaluating-binary-operators
impl Interpreter {
  pub fn evaluate(expr: &Expr) -> Result<LiteralValue, EvalErr> {
    use Expr::*;
    use TokenType::*;
    match expr {
      Unary(operator, right) => {
        let right = match Self::evaluate(right) {
          Err(err) => return Err(err),
          Ok(right) => right,
        };

        match &operator.token_type {
          Minus => match right.cast_float() {
            Err(_) => {
              return eval_err(format!(
                "Cannot apply unary operator '-' to token {}",
                right
              ))
            }
            Ok(v) => Ok(LiteralValue::Num(-v)),
          },
          Bang => Ok(right.to_inverse_bool_literal_value()),
          o => eval_err(format!("Unexpected unary operator: {:?}", o)),
        }
      }
      Binary(left, operator, right) => match &operator.token_type {
        Minus | Star | Slash => {
          let (left, right) = match parse_floats_from_binary_expr(left, right) {
            Err(err) => return Err(err),
            Ok((left, right)) => (left, right),
          };

          match &operator.token_type {
            Minus => Ok(LiteralValue::Num(left - right)),
            Star => Ok(LiteralValue::Num(left * right)),
            Slash => Ok(LiteralValue::Num(left / right)),
            t => return eval_err(format!("unexpected token type: {:?}", t)),
          }
        }
        Plus => {
          // TODO - string addition
          let (left, right) = match parse_floats_from_binary_expr(left, right) {
            Err(err) => return Err(err),
            Ok((left, right)) => (left, right),
          };

          Ok(LiteralValue::Num(left + right))
        }
        t => {
          return eval_err(format!(
            "Unexpected operator in binary expression: '{:?}'",
            t
          ))
        }
      },
      _ => {
        println!("TODO");
        return eval_err(format!("TODO"));
      }
    }
  }
}
