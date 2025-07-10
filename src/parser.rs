use backtrace::Backtrace;

use crate::sexpr::SExpr;

#[derive(Debug)]
pub struct ParserError {
  pub kind: ParserErrorKind,
  pub expected: String,
  pub found: String,
  pub in_context: Vec<String>,
  pub backtrace: Backtrace,
}

#[derive(Debug, PartialEq)]
pub enum ParserErrorKind {
  General,
  SExpressionError,
  Leftover,
  UnexpectedEnd,
  InvalidSExpr,
  Unexpected,
}

impl ParserError {
  pub fn unexpected(expected: impl Into<String>, found: impl Into<String>) -> Self {
    ParserError {
      kind: ParserErrorKind::Unexpected,
      expected: expected.into(),
      found: found.into(),
      in_context: vec![],
      backtrace: Backtrace::new(),
    }
  }

  pub fn unexpected_sexpr(expected: impl Into<String>, found: impl Into<SExpr>) -> Self {
    ParserError {
      kind: ParserErrorKind::Unexpected,
      expected: expected.into(),
      found: format!("{:?}", found.into()),
      in_context: vec![],
      backtrace: Backtrace::new(),
    }
  }

  pub fn add_context(mut self, context: impl Into<String>) -> Self {
    self.in_context.push(context.into());
    self
  }
}
