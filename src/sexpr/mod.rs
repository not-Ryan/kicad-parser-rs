use crate::parser::{Expectable, Parser, ParserError};

mod parse_sexpr;
pub use parse_sexpr::parse_sexpr;

#[derive(Debug, PartialEq, Clone)]
pub enum SExpr {
  List(SExprList),
  Symbol(SExprSymbol),
  Value(SExprValue),
  Float(f64),
  Hex(u64),
}

macro_rules! impl_expectable {
  ($name:ident, $expected: path) => {
    impl Expectable for $name {
      fn expect(expr: SExpr) -> Result<Self, ParserError> {
        match expr {
          $expected(me) => Ok(me),
          expr => Err(ParserError::UnexpectedSExpr {
            expected: stringify!($expected).to_string(),
            found: expr,
          }),
        }
      }
    }
  };
}

#[derive(Clone, Debug, PartialEq)]
pub struct SExprList(pub Vec<SExpr>);
impl_expectable!(SExprList, SExpr::List);

impl SExprList {
  pub fn name(&self) -> Option<&str> {
    if let Some(SExpr::Symbol(SExprSymbol(name))) = self.0.first() {
      Some(name)
    } else {
      None
    }
  }

  pub fn into_parser(self) -> Parser {
    Parser::new(self)
  }
}

#[derive(Debug, PartialEq)]
pub struct SExprSymbol(pub String);
impl_expectable!(SExprSymbol, SExpr::Symbol);

#[derive(Debug, PartialEq)]
pub struct SExprValue(pub String);
impl_expectable!(SExprValue, SExpr::Value);

impl Expectable for String {
  fn expect(expr: SExpr) -> Result<Self, ParserError> {
    match expr {
      SExpr::Value(me) => Ok(me.0),
      expr => Err(ParserError::UnexpectedSExpr {
        expected: stringify!(SExpr::Value).to_string(),
        found: expr,
      }),
    }
  }
}

impl_expectable!(u64, SExpr::Hex);
impl_expectable!(f64, SExpr::Float);
