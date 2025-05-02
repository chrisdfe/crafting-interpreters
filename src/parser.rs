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
    while self.consume_next_token_types(vec![BangEqual, EqualEqual]) {
      let operator = self.current_token();
      let right = self.parse_comparison();
      expr = Box::new(Expr::Binary(expr, operator.clone(), right));
    }

    expr
  }

  fn parse_comparison(&mut self) -> Box<Expr> {
    //
    let mut expr = self.parse_term();

    // TODO
    use TokenType::*;
    while self.consume_next_token_types(vec![Greater, GreaterEqual, Less, LessEqual]) {
      let operator = self.current_token();
      let right = self.parse_term();
      expr = Box::new(Expr::Binary(expr, operator.clone(), right))
    }

    expr
  }

  fn parse_term(&mut self) -> Box<Expr> {
    // placeholder
    let mut expr = self.parse_factor();

    use TokenType::*;
    while self.consume_next_token_types(vec![Minus, Plus]) {
      let operator = self.current_token();
      let right = self.parse_factor();
      expr = Box::new(Expr::Binary(expr, operator.clone(), right));
    }

    expr
  }

  fn parse_factor(&mut self) -> Box<Expr> {
    let mut expr = self.parse_unary();

    use TokenType::*;
    while self.consume_next_token_types(vec![Slash, Star]) {
      let operator = self.current_token();
      let right = self.parse_unary();
      expr = Box::new(Expr::Binary(expr, operator, right));
    }

    expr
  }

  fn parse_unary(&mut self) -> Box<Expr> {
    use TokenType::*;
    if self.consume_next_token_types(vec![Bang, Minus]) {
      let operator = self.current_token();
      let right = self.parse_unary();
      return Box::new(Expr::Unary(operator, right));
    }

    self.parse_primary()
  }

  // Left off right above here
  // https://craftinginterpreters.com/representing-code.html#metaprogramming-the-trees
  fn parse_primary(&mut self) -> Box<Expr> {
    use TokenType::*;

    let expr = Expr::Literal(LiteralValue::Nil);

    // let expr = if self.consume_next_token_type(False) {
    //   Expr::Literal(LiteralValue::False)
    // } else if self.consume_next_token_type(True) {
    //   Expr::Literal(LiteralValue::True)
    // } else if self.consume_next_token_type(Nil) {
    //   Expr::Literal(LiteralValue::Nil)
    // } if self.consume_next_token_types(vec![Num, Str]) {
    //   // TODO
    // }

    // TODO
    Box::new(expr)
  }

  fn consume_next(&mut self) -> Token {
    if !self.is_at_end() {
      self.current += 1;
    }

    self.current_token()
  }

  fn consume_next_token_types(&mut self, token_types: Vec<TokenType>) -> bool {
    for token_type in token_types.iter() {
      if self.next_token_matches(&token_type) {
        self.consume_next();
        return true;
      }
    }

    false
  }

  fn consume_next_token_type(&mut self, token_type: TokenType) -> bool {
    if self.next_token_matches(&token_type) {
      self.consume_next();
      return true;
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

  fn peek_token(&self, distance: usize) -> Token {
    // TODO - don't unwrap
    self.tokens.get(self.current + distance).unwrap().clone()
  }

  fn current_token(&self) -> Token {
    self.tokens.get(self.current).unwrap().clone()
  }
}
