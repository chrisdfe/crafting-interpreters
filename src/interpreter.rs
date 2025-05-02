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

struct Interpreter {}

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
      Binary(left, operator, right) => {
        let left = match Self::evaluate(left) {
          Err(err) => return Err(err),
          Ok(left) => left,
        };

        let right = match Self::evaluate(right) {
          Err(err) => return Err(err),
          Ok(right) => right,
        };

        match &operator.token_type {
          Minus => {
            let left_as_float = match left.cast_float() {
              Err(err) => return eval_err(err),
              Ok(f) => f,
            };

            let right_as_float = match right.cast_float() {
              Err(err) => return eval_err(err),
              Ok(f) => f,
            };

            let result = left_as_float - right_as_float;

            Ok(LiteralValue::Num(result))
          }
          // I left off here:
          // https://craftinginterpreters.com/evaluating-expressions.html#evaluating-binary-operators
          Star => return eval_err(format!("TODO")),
          Slash => return eval_err(format!("TODO")),
          Plus => return eval_err(format!("TODO")),
          t => {
            return eval_err(format!(
              "Unexpected operator in binary expression: '{:?}'",
              t
            ))
          }
        }
      }
      _ => {
        println!("TODO");
        return eval_err(format!("TODO"));
      }
    }
  }
}
