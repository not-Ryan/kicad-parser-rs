use std::fmt::Display;

use crate::parser::ParserError;
pub use sexpr_list::SExprList;

mod parse_sexpr;
mod sexpr_list;
pub use parse_sexpr::parse_sexpr;

#[derive(Debug, Clone)]
pub enum SExpr {
  List(SExprList),
  Symbol(SExprSymbol),
  Value(SExprValue),
  Float(f64),
  Hex(i64),
}

impl SExpr {
  pub fn as_list(self) -> Result<SExprList, ParserError> {
    self.try_into()
  }
}

#[macro_export]
macro_rules! impl_from_into {
  ($name:ident, $expected: path) => {
    impl TryFrom<SExpr> for $name {
      type Error = ParserError;

      fn try_from(expr: SExpr) -> Result<Self, ParserError> {
        match expr {
          $expected(me) => Ok(me),
          expr => Err(ParserError::unexpected_sexpr(stringify!($expected), expr)),
        }
      }
    }

    impl TryFrom<$name> for SExpr {
      type Error = ParserError;

      fn try_from(expr: $name) -> Result<SExpr, ParserError> {
        Ok($expected(expr))
      }
    }
  };
}

#[derive(Clone, Debug, PartialEq)]
pub struct SExprSymbol(pub String);
impl_from_into!(SExprSymbol, SExpr::Symbol);

impl PartialEq<&str> for SExprSymbol {
  fn eq(&self, other: &&str) -> bool {
    other == &self.0
  }
}

impl SExprSymbol {
  pub fn as_str(&self) -> &str {
    &self.0
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SExprValue(pub String);
impl_from_into!(SExprValue, SExpr::Value);

impl TryFrom<SExpr> for String {
  type Error = ParserError;

  fn try_from(expr: SExpr) -> Result<Self, ParserError> {
    match expr {
      SExpr::Value(me) => Ok(me.0),
      expr => Err(ParserError::unexpected_sexpr(
        stringify!(SExpr::Value),
        expr,
      )),
    }
  }
}

impl SExprValue {
  pub fn as_str(&self) -> &str {
    &self.0
  }
}

impl Display for SExprValue {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl TryFrom<SExpr> for f64 {
  type Error = ParserError;

  fn try_from(expr: SExpr) -> Result<Self, ParserError> {
    match expr {
      SExpr::Float(d) => Ok(d),
      SExpr::Hex(d) => Ok(d as f64),
      expr => crate::error!(SExpr, "Value or Hex", expr),
    }
  }
}

impl TryFrom<SExpr> for f32 {
  type Error = ParserError;

  fn try_from(expr: SExpr) -> Result<Self, ParserError> {
    match expr {
      SExpr::Float(d) => Ok(d as f32),
      SExpr::Hex(d) => Ok(d as f32),
      expr => crate::error!(SExpr, "Value or Hex", expr),
    }
  }
}

impl TryFrom<SExpr> for u32 {
  type Error = ParserError;

  fn try_from(expr: SExpr) -> Result<Self, ParserError> {
    match expr {
      SExpr::Float(d) => Ok(d as u32),
      SExpr::Hex(d) => Ok(d as u32),
      expr => crate::error!(SExpr, "Value or Hex", expr),
    }
  }
}

impl TryFrom<SExpr> for u8 {
  type Error = ParserError;

  fn try_from(expr: SExpr) -> Result<Self, ParserError> {
    match expr {
      SExpr::Float(d) => Ok(d as u8),
      SExpr::Hex(d) => Ok(d as u8),
      expr => crate::error!(SExpr, "Value or Hex", expr),
    }
  }
}

impl TryFrom<SExpr> for i32 {
  type Error = ParserError;

  fn try_from(expr: SExpr) -> Result<Self, ParserError> {
    match expr {
      SExpr::Float(d) => Ok(d as i32),
      SExpr::Hex(d) => Ok(d as i32),
      expr => crate::error!(SExpr, "Value or Hex", expr),
    }
  }
}
