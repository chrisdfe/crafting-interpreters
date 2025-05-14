use crate::literals::LiteralValue;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
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
  Str,
  Num,

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
  Return,
  Super,
  This,
  True,
  Var,
  While,

  Eof,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
  pub token_type: TokenType,
  pub lexeme: String,
  pub literal: LiteralValue,
  pub line: usize,
  pub column: usize,
}
