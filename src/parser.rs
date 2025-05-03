use crate::{
  expressions::Expr,
  literals::LiteralValue,
  statements::Stmt,
  tokens::{Token, TokenType},
};

pub type ParseStmtResult = Result<Stmt, ParseErr>;
pub type ParseExprResult = Result<Box<Expr>, ParseErr>;

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
      "parse error: {} At token '{}' on line {} column {}",
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

  pub fn parse(tokens: Vec<Token>) -> Vec<Stmt> {
    Parser::new(tokens).begin_parse()
  }

  fn begin_parse(&mut self) -> Vec<Stmt> {
    let mut stmts = Vec::new();

    while !self.is_at_end() {
      match self.parse_declaration() {
        Err(err) => {
          println!("{}", err.full_text())
        }
        Ok(stmt) => match stmt {
          Some(stmt) => {
            stmts.push(stmt);
          }
          None => (),
        },
      }
    }

    stmts
  }

  fn parse_declaration(&mut self) -> Result<Option<Stmt>, ParseErr> {
    let result = if let Some(_) = self.match_and_consume(TokenType::Var) {
      self.parse_var_declaration()
    } else {
      self.parse_statement()
    };

    match result {
      Ok(stmt) => Ok(Some(stmt)),
      Err(err) => {
        // TODO - should I not do this?
        println!("{}", err.full_text());
        self.synchronize();
        Ok(None)
      }
    }
  }

  fn parse_var_declaration(&mut self) -> ParseStmtResult {
    let name = if let Some(name) = self.match_and_consume(TokenType::Identifier) {
      name
    } else {
      return self.create_parse_stmt_err(String::from("Expected variable name."));
    };

    let initializer = if self.match_and_consume(TokenType::Equal).is_some() {
      let expr = match self.parse_expression() {
        Err(err) => return Err(err),
        Ok(expr) => expr,
      };

      Some(*expr)
    } else {
      None
    };

    if self.match_and_consume(TokenType::Semicolon).is_none() {
      return self.create_parse_stmt_err(String::from("Expected ';' after variable declaration"));
    }

    Ok(Stmt::Var(name, initializer))
  }

  fn parse_statement(&mut self) -> ParseStmtResult {
    if let Some(_) = self.match_and_consume(TokenType::Print) {
      self.parse_print_statement()
    } else {
      //
      match self.parse_expression_statement() {
        Ok(stmt) => Ok(stmt),
        Err(err) => Err(err),
      }
    }
  }

  fn parse_print_statement(&mut self) -> ParseStmtResult {
    let value = match self.parse_expression() {
      Err(err) => return Err(err),
      Ok(value) => value,
    };

    if self.match_and_consume(TokenType::Semicolon).is_none() {
      return self.create_parse_stmt_err(String::from("Expect ';' after value."));
    }

    Ok(Stmt::Print(*value))
  }

  fn parse_expression_statement(&mut self) -> ParseStmtResult {
    let expr = match self.parse_expression() {
      Err(err) => return Err(err),
      Ok(expr) => expr,
    };

    if self.match_and_consume(TokenType::Semicolon).is_none() {
      return self.create_parse_stmt_err(String::from("Expect ';' after value."));
    }

    Ok(Stmt::Expr(*expr))
  }

  fn parse_expression(&mut self) -> ParseExprResult {
    self.parse_equality()
  }

  fn parse_equality(&mut self) -> ParseExprResult {
    let mut expr = match self.parse_comparison() {
      Err(err) => return Err(err),
      Ok(expr) => expr,
    };

    use TokenType::*;
    while let Some(operator) = self.match_one_of_and_consume(vec![BangEqual, EqualEqual]) {
      let right = match self.parse_comparison() {
        Err(err) => return Err(err),
        Ok(expr) => expr,
      };

      expr = Box::new(Expr::Binary(expr, operator.clone(), right));
    }

    Ok(expr)
  }

  fn parse_comparison(&mut self) -> ParseExprResult {
    let mut expr = match self.parse_term() {
      Err(err) => return Err(err),
      Ok(expr) => expr,
    };

    use TokenType::*;
    while let Some(operator) =
      self.match_one_of_and_consume(vec![Greater, GreaterEqual, Less, LessEqual])
    {
      let right = match self.parse_term() {
        Err(err) => return Err(err),
        Ok(expr) => expr,
      };

      expr = Box::new(Expr::Binary(expr, operator.clone(), right))
    }

    Ok(expr)
  }

  fn parse_term(&mut self) -> ParseExprResult {
    let mut expr = match self.parse_factor() {
      Err(err) => return Err(err),
      Ok(expr) => expr,
    };

    use TokenType::*;
    while let Some(operator) = self.match_one_of_and_consume(vec![Minus, Plus]) {
      let right = match self.parse_factor() {
        Err(err) => return Err(err),
        Ok(expr) => expr,
      };

      expr = Box::new(Expr::Binary(expr, operator.clone(), right));
    }

    Ok(expr)
  }

  fn parse_factor(&mut self) -> ParseExprResult {
    let mut expr = match self.parse_unary() {
      Err(err) => return Err(err),
      Ok(expr) => expr,
    };

    use TokenType::*;
    while let Some(operator) = self.match_one_of_and_consume(vec![Slash, Star]) {
      let right = match self.parse_unary() {
        Err(err) => return Err(err),
        Ok(expr) => expr,
      };

      expr = Box::new(Expr::Binary(expr, operator, right));
    }

    Ok(expr)
  }

  fn parse_unary(&mut self) -> ParseExprResult {
    use TokenType::*;
    if let Some(operator) = self.match_one_of_and_consume(vec![Bang, Minus]) {
      let right = match self.parse_unary() {
        Err(err) => return Err(err),
        Ok(expr) => expr,
      };

      Ok(Box::new(Expr::Unary(operator, right)))
    } else {
      self.parse_primary()
    }
  }

  fn parse_primary(&mut self) -> ParseExprResult {
    use TokenType::*;

    let token = match self.consume() {
      Some(token) => token,
      // TODO - not sure what the best way to handle this is
      None => return self.create_parse_expr_err(String::from("Expected expression")),
    };

    let expr = match &token.token_type {
      False => Some(Expr::Literal(LiteralValue::False)),
      True => Some(Expr::Literal(LiteralValue::True)),
      Nil => Some(Expr::Literal(LiteralValue::Nil)),
      Num | Str => Some(Expr::Literal(token.literal)),
      Identifier => Some(Expr::Variable(self.prev_token())),
      LeftParen => {
        let expr = match self.parse_expression() {
          Err(err) => return Err(err),
          Ok(expr) => expr,
        };

        if self.match_and_consume(RightParen).is_none() {
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
      Ok(Box::new(expr))
    } else {
      self.create_parse_expr_err(String::from("Expected expression."))
    }
  }

  fn consume(&mut self) -> Option<Token> {
    let current = self.current_token();

    if !self.is_at_end() {
      self.current += 1;
      Some(current)
    } else {
      None
    }
  }

  fn match_and_consume(&mut self, token_type: TokenType) -> Option<Token> {
    let token = self.current_token();
    if token.token_type == token_type {
      self.consume();
      Some(token)
    } else {
      None
    }
  }

  fn match_one_of_and_consume(&mut self, token_types: Vec<TokenType>) -> Option<Token> {
    let token = self.current_token();
    let matches = token_types
      .into_iter()
      .find(|token_type| token.token_type == *token_type)
      .is_some();

    if matches {
      self.consume();

      Some(token)
    } else {
      None
    }
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

  fn prev_token(&self) -> Token {
    // Note - will panic if self.current == 0
    self.tokens.get(self.current - 1).unwrap().clone()
  }

  fn create_parse_stmt_err(&self, message: String) -> Result<Stmt, ParseErr> {
    Err(ParseErr::create(
      &self.current_token(),
      String::from(message),
    ))
  }

  fn create_parse_expr_err(&self, message: String) -> Result<Box<Expr>, ParseErr> {
    Err(ParseErr::create(
      &self.current_token(),
      String::from(message),
    ))
  }

  fn synchronize(&mut self) {
    self.consume();

    use TokenType::*;
    while !self.is_at_end() {
      if self.prev_token().token_type == TokenType::Semicolon {
        return;
      }

      match self.peek().token_type {
        Class | Fun | Var | For | If | While | Print | Return => (),
        _ => {
          self.consume();
        }
      };
    }
  }
}
