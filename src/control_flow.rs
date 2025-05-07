use crate::literals::LiteralValue;

#[derive(Debug)]
pub enum BeaControlFlow {
  Continue,
  Return(LiteralValue),
}
