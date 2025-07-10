use crate::sexpr::SExpr;

#[derive(Debug, PartialEq)]
pub struct ParserError {
  pub kind: ParserErrorKind,
  pub expected: String,
  pub found: String,
  pub in_context: Vec<&'static str>,
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
    }
  }

  pub fn unexpected_sexpr(expected: impl Into<String>, found: impl Into<SExpr>) -> Self {
    ParserError {
      kind: ParserErrorKind::Unexpected,
      expected: expected.into(),
      found: format!("{:?}", found.into()),
      in_context: vec![],
    }
  }

  pub fn add_context(mut self, context: &'static str) -> Self {
    self.in_context.push(context);
    self
  }
}

pub trait TryFromSExpr: Sized {
  const CONTEXT: &'static str;

  fn try_from(sexpr: SExpr) -> Result<Self, ParserError>;

  fn try_from_with_context(sexpr: SExpr) -> Result<Self, ParserError> {
    match Self::try_from(sexpr) {
      Ok(value) => Ok(value),
      Err(mut err) => {
        err.in_context.push(Self::CONTEXT);
        Err(err)
      }
    }
  }
}
