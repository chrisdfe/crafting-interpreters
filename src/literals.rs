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
  pub fn cast_string(&self) -> String {
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
      // True => Ok(1.),
      // False => Ok(0.),
      Num(n) => Ok(*n),
      _ => Err(format!("Unable to cast '{}' to number", self.to_string())),
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

  pub fn is_string(&self) -> bool {
    if let LiteralValue::Str(_) = &self {
      true
    } else {
      false
    }
  }

  pub fn equals(&self, other: &LiteralValue) -> bool {
    use LiteralValue::*;
    match &self {
      Nil => match &other {
        Nil => true,
        _ => false,
      },
      True => match &other {
        True => true,
        _ => false,
      },
      False => match &other {
        False => true,
        _ => false,
      },
      Num(a) => match &other {
        Num(b) => a == b,
        _ => false,
      },
      Str(a) => match &other {
        Str(b) => a == b,
        _ => false,
      },
    }
  }
}
