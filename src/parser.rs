use crate::{
  expressions::Expr,
  literals::LiteralValue,
  tokens::{Token, TokenType},
};

pub type ParseResult = Result<Box<Expr>, ParseErr>;

pub struct ParseErr {
  pub token: Token,
  pub message: String,
}

impl ParseErr {
  pub fn create(token: &Token, message: String) -> Self {
    ParseErr {
      token: token.clone(),
      message,
    }
  }

  pub fn full_text(&self) -> String {
    format!(
      "{} At token '{}' on line {} column {}",
      &self.message, &self.token.lexeme, &self.token.line, &self.token.column
    )
  }
}

// Next:
// https://craftinginterpreters.com/evaluating-expressions.html
pub struct Parser {
  tokens: Vec<Token>,
  current: usize,
}

// TODO - "synchronize" method
// https://craftinginterpreters.com/parsing-expressions.html#synchronizing-a-recursive-descent-parser
// it's not clear to me how this is supposed to be used right now - the book says we'll return to this
impl Parser {
  pub fn new(tokens: Vec<Token>) -> Self {
    Self { tokens, current: 0 }
  }

  pub fn parse(&mut self) -> ParseResult {
    self.parse_expression()
  }

  fn parse_expression(&mut self) -> ParseResult {
    self.parse_equality()
  }

  fn parse_equality(&mut self) -> ParseResult {
    let mut expr = match self.parse_comparison() {
      Err(err) => return Err(err),
      Ok(expr) => expr,
    };

    use TokenType::*;
    while self.current_token_matches_one_of(vec![BangEqual, EqualEqual]) {
      let operator = self.consume_current_token();

      let right = match self.parse_comparison() {
        Err(err) => return Err(err),
        Ok(expr) => expr,
      };

      expr = Box::new(Expr::Binary(expr, operator.clone(), right));
    }

    Ok(expr)
  }

  fn parse_comparison(&mut self) -> ParseResult {
    let mut expr = match self.parse_term() {
      Err(err) => return Err(err),
      Ok(expr) => expr,
    };

    use TokenType::*;
    while self.current_token_matches_one_of(vec![Greater, GreaterEqual, Less, LessEqual]) {
      let operator = self.consume_current_token();

      let right = match self.parse_term() {
        Err(err) => return Err(err),
        Ok(expr) => expr,
      };

      expr = Box::new(Expr::Binary(expr, operator.clone(), right))
    }

    Ok(expr)
  }

  fn parse_term(&mut self) -> ParseResult {
    let mut expr = match self.parse_factor() {
      Err(err) => return Err(err),
      Ok(expr) => expr,
    };

    use TokenType::*;
    while self.current_token_matches_one_of(vec![Minus, Plus]) {
      let operator = self.consume_current_token();

      let right = match self.parse_factor() {
        Err(err) => return Err(err),
        Ok(expr) => expr,
      };

      expr = Box::new(Expr::Binary(expr, operator.clone(), right));
    }

    Ok(expr)
  }

  fn parse_factor(&mut self) -> ParseResult {
    let mut expr = match self.parse_unary() {
      Err(err) => return Err(err),
      Ok(expr) => expr,
    };

    use TokenType::*;
    while self.current_token_matches_one_of(vec![Slash, Star]) {
      let operator = self.consume_current_token();

      let right = match self.parse_unary() {
        Err(err) => return Err(err),
        Ok(expr) => expr,
      };

      expr = Box::new(Expr::Binary(expr, operator, right));
    }

    Ok(expr)
  }

  fn parse_unary(&mut self) -> ParseResult {
    use TokenType::*;
    if self.current_token_matches_one_of(vec![Bang, Minus]) {
      let operator = self.consume_current_token();

      let right = match self.parse_unary() {
        Err(err) => return Err(err),
        Ok(expr) => expr,
      };

      Ok(Box::new(Expr::Unary(operator, right)))
    } else {
      self.parse_primary()
    }
  }

  fn parse_primary(&mut self) -> ParseResult {
    use TokenType::*;

    let current_token = self.current_token();

    let expr = match &current_token.token_type {
      False => Some(Expr::Literal(LiteralValue::False)),
      True => Some(Expr::Literal(LiteralValue::True)),
      Nil => Some(Expr::Literal(LiteralValue::Nil)),
      Num | Str => Some(Expr::Literal(current_token.literal)),
      LeftParen => {
        self.consume_current_token();

        let expr = match self.parse_expression() {
          Err(err) => return Err(err),
          Ok(expr) => expr,
        };

        if !self.current_token_matches(RightParen) {
          return Err(ParseErr::create(
            &self.current_token(),
            String::from("Expected ) after expression"),
          ));
        }

        Some(Expr::Grouping(expr))
      }
      _ => None,
    };

    if let Some(expr) = expr {
      self.consume_current_token();
      Ok(Box::new(expr))
    } else {
      Err(ParseErr::create(
        &self.current_token(),
        String::from("Expected expression."),
      ))
    }
  }

  fn consume_current_token(&mut self) -> Token {
    let current = self.current_token();

    if !self.is_at_end() {
      self.current += 1;
    }

    current
  }

  fn current_token_matches(&self, token_type: TokenType) -> bool {
    self.current_token().token_type == token_type
  }

  fn current_token_matches_one_of(&mut self, token_types: Vec<TokenType>) -> bool {
    let current_token = self.current_token();
    token_types
      .into_iter()
      .find(|token_type| current_token.token_type == *token_type)
      .is_some()
  }

  fn is_at_end(&self) -> bool {
    self.peek().token_type == TokenType::Eof
  }

  fn peek(&self) -> Token {
    // TODO - don't unwrap
    self.tokens.get(self.current + 1).unwrap().clone()
  }

  fn current_token(&self) -> Token {
    self.tokens.get(self.current).unwrap().clone()
  }
}
