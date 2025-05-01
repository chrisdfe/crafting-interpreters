use std::str::Chars;

enum TokenType {
  // Single-character tokens.
  LeftParen,
  RightParen,
  LeftBrace,
  RightBrace,
  Comma,
  Dot,
  Minus,
  Plus,
  Semicolon,
  Slash,
  Star,

  // One or two character tokens.
  Bang,
  BangEqual,
  Equal,
  EqualEqual,
  Greater,
  GreaterEqual,
  Less,
  LessEqual,

  // Literals.
  Identifier,
  Str, // string
  Number,

  // Keywords.
  And,
  Class,
  Else,
  False,
  Fun,
  For,
  If,
  Nil,
  Or,
  Print,
  Return,
  Super,
  This,
  True,
  Var,
  While,

  Eof,
}

pub struct Token {
  token_type: TokenType,
  lexeme: String,
  // final Object literal;
  line: usize,
}

struct ScanErr {
  message: String,
  column: usize,
  line: usize,
}

struct Scanner {
  source: String,
  tokens: Vec<Token>,
  errors: Vec<ScanErr>,
  start: usize,
  current: usize,
  line: usize,
}

impl Scanner {
  fn new(source: String) -> Scanner {
    Scanner {
      source,
      tokens: Vec::new(),
      errors: Vec::new(),
      start: 0,
      current: 0,
      line: 1,
    }
  }

  fn scan(&mut self) -> Result<Vec<Token>, String> {
    let mut result = Vec::new();

    while !self.is_at_end() {
      // we are at the beginning of the next lexeme
      self.start = self.current;
      match self.scan_token() {
        Err(message) => {
          // TODO - add error to errors
          self.errors.push(ScanErr {
            message,
            line: self.line,
            // TODO - I think this is wrong
            column: self.current,
          });
        }
        _ => (),
      }
    }

    result.push(Token {
      token_type: TokenType::Eof,
      lexeme: String::from(""),
      line: self.line,
    });

    Ok(result)
  }

  fn is_at_end(&self) -> bool {
    self.current >= self.source.len()
  }

  fn scan_token(&mut self) -> Result<(), String> {
    use TokenType::*;
    let c = self.advance();

    println!("c: {c}");
    match c {
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
        let token = if self.matches('=') { BangEqual } else { Bang };
        self.add_token(token);
      }
      '=' => {
        let token = if self.matches('=') { EqualEqual } else { Equal };
        self.add_token(token);
      }
      '<' => {
        let token = if self.matches('=') { LessEqual } else { Less };
        self.add_token(token);
      }
      '>' => {
        let token = if self.matches('=') {
          GreaterEqual
        } else {
          Greater
        };
        self.add_token(token);
      }
      _ => return Err(std::string::String::from("Unrecognized character: '{c}'")),
    }

    Ok(())
  }

  fn matches(&mut self, expected: char) -> bool {
    if self.is_at_end() {
      false
    } else if self.current_char() != expected {
      false
    } else {
      self.current += 1;
      true
    }
  }

  fn advance(&mut self) -> char {
    let c = self.current_char();
    self.current += 1;
    c
  }

  fn add_token(&mut self, token_type: TokenType) {
    // TODO - does this need to be current + 1?
    let text = String::from(&self.source[self.start..self.current]);
    self.tokens.push(Token {
      token_type,
      lexeme: text,
      line: self.line,
    });
  }

  fn char_at(&self, idx: usize) -> char {
    self.source.chars().nth(idx).unwrap()
  }

  fn current_char(&self) -> char {
    self.char_at(self.current)
  }
}

pub fn scan(source: String) -> Result<Vec<Token>, String> {
  Scanner::new(source).scan()
}
