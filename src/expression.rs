use super::tokens::Token;

// https://craftinginterpreters.com/representing-code.html#metaprogramming-the-trees
pub struct Expr<'a> {
  left: &'a Expr<'a>,
  operator: Token,
  right: &'a Expr<'a>,
}
