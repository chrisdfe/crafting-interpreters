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

pub struct Interpreter {}

// I am currently here
// https://craftinginterpreters.com/statements-and-state.html
impl Interpreter {
  pub fn interpret(expr: &Expr) -> String {
    match Self::evaluate(&expr) {
      Ok(output) => output.to_string(),
      Err(err) => err.message,
    }
  }

  fn evaluate(expr: &Expr) -> Result<LiteralValue, EvalErr> {
    use Expr::*;
    use TokenType::*;
    match expr {
      Literal(value) => Ok(value.clone()),
      Grouping(expr) => Self::evaluate(expr),
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
      Binary(left, operator, right) => {
        let left = match Self::evaluate(&left) {
          Err(err) => return Err(err),
          Ok(left) => left,
        };

        let right = match Self::evaluate(&right) {
          Err(err) => return Err(err),
          Ok(right) => right,
        };

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
              t => return eval_err(format!("unexpected token type: {:?}", t)),
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
              _ => return eval_err(format!("Unexpected token type: {:?}", &operator.token_type)),
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
          t => {
            return eval_err(format!(
              "Unexpected operator in binary expression: '{:?}'",
              t
            ))
          }
        }
      }
    }
  }
}

fn parse_floats_from_binary_expr(
  left: &LiteralValue,
  right: &LiteralValue,
) -> Result<(f32, f32), EvalErr> {
  let left_as_float = match left.cast_float() {
    Err(err) => return Err(EvalErr::new(err)),
    Ok(f) => f,
  };

  let right_as_float = match right.cast_float() {
    Err(err) => return Err(EvalErr::new(err)),
    Ok(f) => f,
  };

  Ok((left_as_float, right_as_float))
}

fn is_string_literal(literal: &LiteralValue) -> bool {
  match literal {
    LiteralValue::Str(_) => true,
    _ => false,
  }
}
