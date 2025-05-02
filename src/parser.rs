use crate::{
  expressions::{Expr, LiteralValue},
  tokens::{Token, TokenType},
};

struct Parser {
  tokens: Vec<Token>,
  current: usize,
}

impl Parser {
  pub fn new(tokens: Vec<Token>) -> Self {
    Self { tokens, current: 0 }
  }

  fn parse_expression(&mut self) -> Box<Expr> {
    self.parse_equality()
  }

  fn parse_equality(&mut self) -> Box<Expr> {
    let mut expr = self.parse_comparison();

    use TokenType::*;
    while self.advance_for_token_types(vec![BangEqual, EqualEqual]) {
      let operator = self.current_token();
      let right = self.parse_comparison();
      expr = Box::new(Expr::Binary(expr, operator.clone(), right));
    }

    expr
  }

  fn parse_comparison(&self) -> Box<Expr> {
    //
    let mut expr = self.parse_term();

    // TODO

    expr
  }

  fn parse_term(&self) -> Box<Expr> {
    // placeholder
    Box::new(Expr::Literal(LiteralValue::Nil))
  }

  fn advance(&mut self) -> &Token {
    if !self.is_at_end() {
      self.current += 1;
    }

    self.current_token()
  }

  fn advance_for_token_types(&mut self, token_types: Vec<TokenType>) -> bool {
    for token_type in token_types.iter() {
      if self.next_token_matches(&token_type) {
        self.advance();
        return true;
      }
    }

    false
  }

  fn next_token_matches(&self, token_type: &TokenType) -> bool {
    if self.is_at_end() {
      return false;
    }

    self.peek_token(1).token_type == *token_type
  }

  fn is_at_end(&self) -> bool {
    self.peek_token(1).token_type == TokenType::Eof
  }

  fn peek_token(&self, distance: usize) -> &Token {
    // TODO - don't unwrap
    self.tokens.get(self.current + distance).unwrap()
  }

  fn current_token(&self) -> &Token {
    self.tokens.get(self.current).unwrap()
  }
}
