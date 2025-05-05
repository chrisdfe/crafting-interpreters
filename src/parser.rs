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
  pub fn new(token: &Token, message: String) -> Self {
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
    Parser::new(tokens).parse_program()
  }

  fn parse_program(&mut self) -> Vec<Stmt> {
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
    let token = self.consume()?;

    match token.token_type {
      For => self.parse_for_statement(),
      If => self.parse_if_statement(),
      Print => self.parse_print_statement(),
      While => self.parse_while_statement(),
      LeftBrace => {
        let statements = self.parse_statements_in_block()?;
        Ok(Stmt::Block(statements))
      }
      _ => {
        self.unconsume();
        self.parse_expression_statement()
      }
    }
  }

  fn parse_for_statement(&mut self) -> ParseStmtResult {
    self.match_and_consume_or_err(LeftParen, String::from("Expect '(' after for"))?;

    let initializer = match self.consume()?.token_type {
      Semicolon => None,
      Var => {
        let expr = self.parse_var_declaration()?;
        Some(Box::new(expr))
      }
      _ => {
        let expr = self.parse_expression_statement()?;
        Some(Box::new(expr))
      }
    };

    let condition = if self.current_token().token_type != Semicolon {
      self.parse_expression()?
    } else {
      Box::new(Expr::Literal(LiteralValue::True))
    };
    self.match_and_consume_or_err(Semicolon, String::from("Expected ';' after loop condition"))?;

    let increment = if self.current_token().token_type != RightParen {
      Some(self.parse_expression()?)
    } else {
      None
    };
    self.match_and_consume_or_err(RightParen, String::from("Expected ')' after 'for' clause"))?;

    let mut body = self.parse_statement()?;

    if let Some(increment) = increment {
      body = Stmt::Block(vec![body, Stmt::Expr(*increment)])
    }

    body = Stmt::While(condition, Box::new(body));

    if let Some(initializer) = initializer {
      body = Stmt::Block(vec![*initializer, body])
    }

    Ok(body)
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

  fn parse_while_statement(&mut self) -> ParseStmtResult {
    self.match_and_consume_or_err(LeftParen, String::from("Expected '(' after 'while'"))?;
    let condition = self.parse_expression()?;
    self.match_and_consume_or_err(RightParen, String::from("Expected ')' after condition"))?;

    let body = self.parse_statement()?;

    Ok(Stmt::While(condition, Box::new(body)))
  }

  fn parse_statements_in_block(&mut self) -> Result<Vec<Stmt>, ParseErr> {
    let mut statements: Vec<Stmt> = Vec::new();

    while !self.current_token_matches(RightBrace) && !self.is_at_end() {
      if let Some(statement) = self.parse_declaration()? {
        statements.push(statement);
      }
    }

    if self.match_and_consume(RightBrace).is_none() {
      Err(ParseErr::new(
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
    let expr = self.parse_or_expression()?;

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

  fn parse_or_expression(&mut self) -> ParseExprResult {
    //
    let mut expr = self.parse_and_expression()?;

    while self.match_and_consume(Or).is_some() {
      let operator = self.prev_token();
      let right = self.parse_and_expression()?;
      expr = Box::new(Expr::Logical(expr, operator, right));
    }

    Ok(expr)
  }

  fn parse_and_expression(&mut self) -> ParseExprResult {
    //
    let mut expr = self.parse_equality_expression()?;

    while self.match_and_consume(And).is_some() {
      let operator = self.prev_token();
      let right = self.parse_equality_expression()?;
      expr = Box::new(Expr::Logical(expr, operator, right))
    }

    Ok(expr)
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
      self.parse_call_expression()
    }
  }

  fn parse_call_expression(&mut self) -> ParseExprResult {
    let mut expr = self.parse_primary_expression()?;

    loop {
      if self.match_and_consume(LeftParen).is_some() {
        expr = Box::new(self.finish_call(*expr)?);
      } else {
        break;
      }
    }

    Ok(expr)
  }

  fn finish_call(&mut self, callee: Expr) -> Result<Expr, ParseErr> {
    let mut arguments = Vec::new();
    if self.current_token().token_type != RightParen {
      'parse_args: loop {
        arguments.push(*self.parse_expression()?);
        if self.match_and_consume(Comma).is_none() {
          break 'parse_args;
        }
      }
    }

    if arguments.len() > 255 {
      return Err(ParseErr::new(
        &self.current_token(),
        String::from("Can't have more than 255 arguments"),
      ));
    }

    let paren =
      self.match_and_consume_or_err(RightParen, String::from("Expected ')' after arguments"))?;

    Ok(Expr::Call(Box::new(callee), paren, arguments))
  }

  fn parse_primary_expression(&mut self) -> ParseExprResult {
    let token = self.consume()?;

    let expr = match &token.token_type {
      False => Some(Expr::Literal(LiteralValue::False)),
      True => Some(Expr::Literal(LiteralValue::True)),
      Nil => Some(Expr::Literal(LiteralValue::Nil)),
      Num | Str => Some(Expr::Literal(token.literal)),
      Identifier => Some(Expr::Variable(self.prev_token())),
      LeftParen => {
        let expr = self.parse_expression()?;

        if self.match_and_consume(RightParen).is_none() {
          return Err(ParseErr::new(
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

  fn consume(&mut self) -> Result<Token, ParseErr> {
    let current = self.current_token();

    if !self.is_at_end() {
      self.current += 1;
      Ok(current)
    } else {
      Err(ParseErr::new(
        &current,
        format!("Unexpected end of input at token '{}'", &current.lexeme),
      ))
    }
  }

  fn unconsume(&mut self) -> Token {
    if self.current > 0 {
      self.current -= 1;
    }

    self.current_token()
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

  fn match_and_consume_or_err(
    &mut self,
    token_type: TokenType,
    message: String,
  ) -> Result<Token, ParseErr> {
    let token = self.current_token();
    if token.token_type == token_type {
      self.consume();
      Ok(token)
    } else {
      Err(ParseErr::new(&self.current_token(), message))
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
    Err(ParseErr::new(&self.current_token(), message))
  }

  fn create_parse_expr_err(&self, token: &Token, message: String) -> Result<Box<Expr>, ParseErr> {
    Err(ParseErr::new(token, message))
  }

  fn synchronize(&mut self) {
    let _ = self.consume();

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
