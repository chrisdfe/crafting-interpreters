use lazy_static::lazy_static;
use std::collections::HashMap;

use crate::{
  literals::LiteralValue,
  tokens::{Token, TokenType},
};

lazy_static! {
  static ref KEYWORDS: HashMap<&'static str, TokenType> = HashMap::from([
    ("and", TokenType::And), //
    ("class", TokenType::And), //
    ("else", TokenType::Else), //
    ("false", TokenType::False), //
    ("for", TokenType::For), //
    ("fun", TokenType::Fun), //
    ("if", TokenType::If), //
    ("nil", TokenType::Nil), //
    ("or", TokenType::Or), //
    ("return", TokenType::Return), //
    ("super", TokenType::Super), //
    ("this", TokenType::This), //
    ("true", TokenType::True), //
    ("var", TokenType::Var), //
    ("while", TokenType::While), //
  ]);
}

pub struct ScanErr {
  pub message: String,
  pub column: usize,
  pub line: usize,
}

pub struct Scanner {
  pub tokens: Vec<Token>,
  pub errors: Vec<ScanErr>,
  source: String,
  start: usize,
  current: usize,
  line: usize,
  column: usize,
}

impl Scanner {
  pub fn new(source: String) -> Scanner {
    Scanner {
      source,
      tokens: Vec::new(),
      errors: Vec::new(),
      start: 0,
      current: 0,
      line: 1,
      column: 1,
    }
  }

  pub fn scan(source: String) -> Vec<Token> {
    let mut scanner = Scanner::new(source);
    scanner.start();
    scanner.tokens
  }

  pub fn start(&mut self) {
    if self.source.is_empty() {
      return;
    }

    self.scan_token();

    while !self.is_at_end() {
      // we are at the beginning of the next lexeme
      self.advance_char();
      self.start = self.current;

      self.scan_token();
    }

    self.tokens.push(Token {
      token_type: TokenType::Eof,
      lexeme: String::from(""),
      literal: LiteralValue::Nil,
      line: self.line,
      column: self.column,
    });
  }

  fn scan_token(&mut self) {
    use TokenType::*;

    match self.current_char() {
      '(' => self.add_token(LeftParen),
      ')' => self.add_token(RightParen),
      '{' => self.add_token(LeftBrace),
      '}' => self.add_token(RightBrace),
      ',' => self.add_token(Comma),
      '.' => self.add_token(Dot),
      '-' => self.add_token(Minus),
      '+' => self.add_token(Plus),
      ';' => self.add_token(Semicolon),
      '*' => self.add_token(Star),
      '!' => {
        let token = if self.match_next_char('=') {
          BangEqual
        } else {
          Bang
        };
        self.add_token(token);
      }
      '=' => {
        let token = if self.match_next_char('=') {
          EqualEqual
        } else {
          Equal
        };
        self.add_token(token);
      }
      '<' => {
        let token = if self.match_next_char('=') {
          LessEqual
        } else {
          Less
        };
        self.add_token(token);
      }
      '>' => {
        let token = if self.match_next_char('=') {
          GreaterEqual
        } else {
          Greater
        };
        self.add_token(token);
      }
      '/' => {
        if self.match_next_char('/') {
          // A comment goes until the end of the line
          while self.peek(1) != '\n' && !self.is_at_end() {
            self.advance_char();
          }
        } else {
          self.add_token(Slash);
        }
      }
      '"' => self.string(),
      '0'..='9' => self.number(),
      // Ignore whitespace
      ' ' | '\r' | '\t' => {}
      '\n' => {
        self.line += 1;
        self.column = 1;
      }
      c => {
        if is_alpha(c) {
          self.identifier();
        } else {
          self.add_error(format!("Unrecognized character: '{c}'"));
        }
      }
    }
  }

  fn identifier(&mut self) {
    while is_alphanumeric(self.peek(1)) {
      self.advance_char();
    }

    let text = self.get_current_substr();
    let token_type = match KEYWORDS.get(&text.as_str()) {
      Some(token_type) => token_type.clone(),
      None => TokenType::Identifier,
    };

    self.add_token(token_type);
  }

  // TODO - better name (this is what it's called in the book)
  fn number(&mut self) {
    //
    while is_digit(self.peek(1)) {
      self.advance_char();
    }

    // Look for a fractional part.
    if self.peek(1) == '.' && is_digit(self.peek(2)) {
      // consume the "."
      self.advance_char();

      while is_digit(self.peek(1)) {
        self.advance_char();
      }
    }

    let current_substr = self.get_current_substr();

    // TODO - don't unwrap
    let value = current_substr.as_str().parse::<f32>().unwrap();
    self.add_literal_token(TokenType::Num, LiteralValue::Num(value));
  }

  // TODO - better name (this is what it's called in the book)
  fn string(&mut self) {
    while self.peek(1) != '"' && !self.is_at_end() {
      if self.peek(1) == '\n' {
        self.line += 1;
      }

      self.advance_char();
    }

    if self.is_at_end() {
      self.add_error(String::from("Unterminated string"));
      return;
    }

    // The closing ".
    self.advance_char();

    // +1 and -1 to remove quotes
    let string_value = get_char_substr(&self.source, self.start + 1, self.current - 1);
    let value = TokenType::Str;
    self.add_literal_token(value, LiteralValue::Str(string_value));
  }

  fn add_error(&mut self, message: String) {
    self.errors.push(ScanErr {
      message,
      line: self.line,
      column: self.column,
    });
  }

  fn is_at_end(&self) -> bool {
    self.current >= self.source.len() - 1
  }

  fn match_next_char(&mut self, expected: char) -> bool {
    if self.is_at_end() {
      return false;
    }

    if self.peek(1) == expected {
      self.current += 1;
      true
    } else {
      false
    }
  }

  // Deviating from the book slightly, but I don't like how in the book this method
  // 1) is just named 'advance'
  // 2) increments like this (in java) "source[current++]", so the character returned is actually for the previous character to self.current
  fn advance_char(&mut self) -> char {
    self.current += 1;
    self.column += 1;
    self.current_char()
  }

  // The book's version of peek doesn't take this distance parameter because it wants to make a point of how
  // the interpreter only looks ahead at most 2 characters. I think this is clea(r,n)er
  fn peek(&self, distance: usize) -> char {
    if self.current + distance >= self.source.len() {
      '\0'
    } else {
      self.char_at(self.current + distance)
    }
  }

  fn add_token(&mut self, token_type: TokenType) {
    let text = get_char_substr(&self.source, self.start, self.current);

    self.tokens.push(Token {
      token_type,
      literal: LiteralValue::Nil,
      lexeme: text,
      line: self.line,
      column: self.column,
    });
  }

  fn add_literal_token(&mut self, token_type: TokenType, literal: LiteralValue) {
    let text = get_char_substr(&self.source, self.start, self.current);
    self.tokens.push(Token {
      token_type,
      literal,
      lexeme: text,
      line: self.line,
      column: self.column,
    });
  }

  fn char_at(&self, idx: usize) -> char {
    self.source.chars().nth(idx).unwrap()
  }

  fn current_char(&self) -> char {
    self.char_at(self.current)
  }

  fn get_current_substr(&self) -> String {
    get_char_substr(&self.source, self.start, self.current)
  }
}

fn is_alpha(c: char) -> bool {
  c.is_ascii_lowercase() || // lowercase
  c.is_ascii_uppercase() || // uppercase
  c == '_'
}

fn is_digit(c: char) -> bool {
  c.is_ascii_digit()
}

fn is_alphanumeric(c: char) -> bool {
  is_alpha(c) || is_digit(c)
}

fn get_char_substr(s: &str, start: usize, end: usize) -> String {
  s[start..=end].to_string()
}
