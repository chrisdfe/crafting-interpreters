use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum LiteralValue {
  Nil,
  True,
  False,
  Num(f32),
  Str(String),
}

impl Display for LiteralValue {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    use LiteralValue::*;
    match self {
      Nil => write!(f, "nil"),
      True => write!(f, "true"),
      False => write!(f, "false"),
      Str(s) => write!(f, "{}", s),
      Num(n) => write!(f, "{}", n),
    }
  }
}

const INVALID_FLOAT_CAST: &'static str = "Invalid cast to float.";

impl From<bool> for LiteralValue {
  fn from(value: bool) -> Self {
    if value {
      LiteralValue::True
    } else {
      LiteralValue::False
    }
  }
}

impl LiteralValue {
  // TODO - more rust-y way of doing this
  pub fn to_string(&self) -> String {
    use LiteralValue::*;
    match &self {
      Nil => String::from("nil"),
      True => String::from("true"),
      False => String::from("false"),
      Num(n) => format!("{}", n),
      Str(s) => s.clone(),
    }
  }

  pub fn cast_float(&self) -> Result<f32, String> {
    use LiteralValue::*;
    match &self {
      Nil => Err(format!("Unable to cast '{}' to float", self.to_string())),
      True => Ok(1.),
      False => Ok(0.),
      Num(n) => Ok(*n),
      Str(s) => Err(format!("Unable to cast '{}' to float", self.to_string())),
    }
  }

  pub fn to_bool_literal_value(&self) -> LiteralValue {
    LiteralValue::from(self.is_truthy())
  }

  pub fn to_inverse_bool_literal_value(&self) -> LiteralValue {
    LiteralValue::from(!self.is_truthy())
  }

  pub fn is_truthy(&self) -> bool {
    use LiteralValue::*;
    match &self {
      Nil => false,
      False => false,
      _ => true,
    }
  }

  pub fn is_falsey(&self) -> bool {
    !self.is_truthy()
  }
}
