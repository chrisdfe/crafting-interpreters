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

use TokenType::*;
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
        Ok(stmt) => {
          if let Some(stmt) = stmt {
            stmts.push(stmt);
          }
        }
      }
    }

    stmts
  }

  fn parse_declaration(&mut self) -> Result<Option<Stmt>, ParseErr> {
    let result = if self.match_and_consume(Var).is_some() {
      self.parse_var_declaration()
    } else {
      self.parse_statement()
    };

    match result {
      Ok(stmt) => Ok(Some(stmt)),
      Err(err) => {
        // TODO - should I not print this here?
        println!("{}", err.full_text());
        self.synchronize();
        Ok(None)
      }
    }
  }

  fn parse_var_declaration(&mut self) -> ParseStmtResult {
    let name = if let Some(name) = self.match_and_consume(Identifier) {
      name
    } else {
      return self.create_parse_stmt_err(String::from("Expected variable name."));
    };

    let initializer = if self.match_and_consume(Equal).is_some() {
      let expr = self.parse_expression()?;

      Some(*expr)
    } else {
      None
    };

    if self.match_and_consume(Semicolon).is_none() {
      return self.create_parse_stmt_err(String::from("Expected ';' after variable declaration"));
    }

    Ok(Stmt::Var(name, initializer))
  }

  fn parse_statement(&mut self) -> ParseStmtResult {
    if self.match_and_consume(If).is_some() {
      self.parse_if_statement()
    } else if self.match_and_consume(Print).is_some() {
      self.parse_print_statement()
    } else if self.match_and_consume(LeftBrace).is_some() {
      let statements = self.parse_statements_in_block()?;
      Ok(Stmt::Block(statements))
    } else {
      //
      match self.parse_expression_statement() {
        Ok(stmt) => Ok(stmt),
        Err(err) => Err(err),
      }
    }
  }

  fn parse_if_statement(&mut self) -> ParseStmtResult {
    if self.match_and_consume(LeftParen).is_none() {
      return self.create_parse_stmt_err(String::from("Expected '(' after 'if'"));
    }

    let condition = self.parse_expression()?;

    if self.match_and_consume(RightParen).is_none() {
      return self.create_parse_stmt_err(String::from("Expected '(' after 'if'"));
    }

    let then_branch = match self.parse_statement() {
      Ok(stmt) => Box::new(stmt),
      Err(err) => return Err(err),
    };

    let else_branch = if self.match_and_consume(Else).is_some() {
      let statement = self.parse_statement()?;
      Some(Box::new(statement))
    } else {
      None
    };

    Ok(Stmt::If(*condition, then_branch, else_branch))
  }

  fn parse_print_statement(&mut self) -> ParseStmtResult {
    let value = self.parse_expression()?;

    if self.match_and_consume(Semicolon).is_none() {
      return self.create_parse_stmt_err(String::from("Expect ';' after value."));
    }

    Ok(Stmt::Print(*value))
  }

  fn parse_statements_in_block(&mut self) -> Result<Vec<Stmt>, ParseErr> {
    let mut statements: Vec<Stmt> = Vec::new();

    while !self.current_token_matches(RightBrace) && !self.is_at_end() {
      if let Some(statement) = self.parse_declaration()? {
        statements.push(statement);
      }
    }

    if self.match_and_consume(RightBrace).is_none() {
      Err(ParseErr::create(
        &self.current_token(),
        String::from("Expected '}' after block."),
      ))
    } else {
      Ok(statements)
    }
  }

  fn parse_expression_statement(&mut self) -> ParseStmtResult {
    let expr = self.parse_expression()?;

    if self.match_and_consume(Semicolon).is_none() {
      return self.create_parse_stmt_err(String::from("Expect ';' after value."));
    }

    Ok(Stmt::Expr(*expr))
  }

  fn parse_expression(&mut self) -> ParseExprResult {
    self.parse_assignment_expression()
  }

  fn parse_assignment_expression(&mut self) -> ParseExprResult {
    let expr = self.parse_equality_expression()?;

    if self.match_and_consume(Equal).is_some() {
      let equals = self.prev_token();

      let value = self.parse_assignment_expression()?;

      if let Expr::Variable(name) = expr.as_ref() {
        Ok(Box::new(Expr::Assign(name.clone(), value)))
      } else {
        self.create_parse_expr_err(&equals, String::from("Invalid asignment target."))
      }
    } else {
      Ok(expr)
    }
  }

  fn parse_equality_expression(&mut self) -> ParseExprResult {
    let mut expr = self.parse_comparison_expression()?;

    use TokenType::*;
    while let Some(operator) = self.match_one_of_and_consume(vec![BangEqual, EqualEqual]) {
      let right = self.parse_comparison_expression()?;

      expr = Box::new(Expr::Binary(expr, operator.clone(), right));
    }

    Ok(expr)
  }

  fn parse_comparison_expression(&mut self) -> ParseExprResult {
    let mut expr = self.parse_term_expression()?;

    use TokenType::*;
    while let Some(operator) =
      self.match_one_of_and_consume(vec![Greater, GreaterEqual, Less, LessEqual])
    {
      let right = self.parse_term_expression()?;

      expr = Box::new(Expr::Binary(expr, operator.clone(), right))
    }

    Ok(expr)
  }

  fn parse_term_expression(&mut self) -> ParseExprResult {
    let mut expr = self.parse_factor_expression()?;

    use TokenType::*;
    while let Some(operator) = self.match_one_of_and_consume(vec![Minus, Plus]) {
      let right = self.parse_factor_expression()?;

      expr = Box::new(Expr::Binary(expr, operator.clone(), right));
    }

    Ok(expr)
  }

  fn parse_factor_expression(&mut self) -> ParseExprResult {
    let mut expr = self.parse_unary_expression()?;

    use TokenType::*;
    while let Some(operator) = self.match_one_of_and_consume(vec![Slash, Star]) {
      let right = self.parse_unary_expression()?;

      expr = Box::new(Expr::Binary(expr, operator, right));
    }

    Ok(expr)
  }

  fn parse_unary_expression(&mut self) -> ParseExprResult {
    use TokenType::*;
    if let Some(operator) = self.match_one_of_and_consume(vec![Bang, Minus]) {
      let right = self.parse_unary_expression()?;

      Ok(Box::new(Expr::Unary(operator, right)))
    } else {
      self.parse_primary_expression()
    }
  }

  fn parse_primary_expression(&mut self) -> ParseExprResult {
    use TokenType::*;

    let token = match self.consume() {
      Some(token) => token,
      // TODO - not sure what the best way to handle this is
      None => {
        return self
          .create_parse_expr_err(&self.current_token(), String::from("Expected expression"))
      }
    };

    let expr = match &token.token_type {
      False => Some(Expr::Literal(LiteralValue::False)),
      True => Some(Expr::Literal(LiteralValue::True)),
      Nil => Some(Expr::Literal(LiteralValue::Nil)),
      Num | Str => Some(Expr::Literal(token.literal)),
      Identifier => Some(Expr::Variable(self.prev_token())),
      LeftParen => {
        let expr = self.parse_expression()?;

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
      self.create_parse_expr_err(&self.current_token(), String::from("Expected expression."))
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
      .any(|token_type| token.token_type == token_type);

    if matches {
      self.consume();

      Some(token)
    } else {
      None
    }
  }

  fn current_token_matches(&self, token_type: TokenType) -> bool {
    if self.is_at_end() {
      false
    } else {
      self.current_token().token_type == token_type
    }
  }

  fn is_at_end(&self) -> bool {
    self.peek().token_type == Eof
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
    Err(ParseErr::create(&self.current_token(), message))
  }

  fn create_parse_expr_err(&self, token: &Token, message: String) -> Result<Box<Expr>, ParseErr> {
    Err(ParseErr::create(token, message))
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
