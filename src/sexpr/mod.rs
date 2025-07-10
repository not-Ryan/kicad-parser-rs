use crate::parser::{ParserError, TryFromSExpr};
pub use sexpr_list::SExprList;

mod parse_sexpr;
mod sexpr_list;
pub use parse_sexpr::parse_sexpr;

#[derive(Debug, PartialEq, Clone)]
pub enum SExpr {
  List(SExprList),
  Symbol(SExprSymbol),
  Value(SExprValue),
  Float(f64),
  Hex(u64),
}

impl SExpr {
  pub fn as_list(self) -> Result<SExprList, ParserError> {
    SExprList::try_from_with_context(self)
  }
}

#[macro_export]
macro_rules! impl_from_into {
  ($name:ident, $expected: path) => {
    impl TryFromSExpr for $name {
      const CONTEXT: &'static str = stringify!($name);

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

impl TryFromSExpr for String {
  const CONTEXT: &'static str = "sexpr::String";

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

impl TryFromSExpr for f32 {
  const CONTEXT: &'static str = "sexpr::f32";

  fn try_from(expr: SExpr) -> Result<Self, ParserError> {
    match expr {
      SExpr::Float(d) => Ok(d as f32),
      SExpr::Hex(d) => Ok(d as f32),

      expr => Err(ParserError::unexpected_sexpr(
        stringify!(SExpr::Value or SExpr::Hex),
        expr,
      )),
    }
  }
}

impl TryFromSExpr for u32 {
  const CONTEXT: &'static str = "sexpr::u32";

  fn try_from(expr: SExpr) -> Result<Self, ParserError> {
    match expr {
      SExpr::Float(d) => Ok(d as u32),
      SExpr::Hex(d) => Ok(d as u32),

      expr => Err(ParserError::unexpected_sexpr(
        stringify!(SExpr::Value or SExpr::Hex),
        expr,
      )),
    }
  }
}
