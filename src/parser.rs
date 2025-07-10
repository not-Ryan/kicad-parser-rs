use crate::sexpr::{SExpr, SExprList, SExprSymbol};

#[derive(Debug, PartialEq)]
pub enum ParserError {
  SExpressionError(String),
  General(String),

  UnexpectedEnd,
  UnexpectedLeftover {
    leftover: Vec<SExpr>,
  },
  UnexpectedToken {
    message: String,
    token: Option<String>,
    expr: Option<SExpr>,
  },
  UnexpectedSExpr {
    expected: String,
    found: SExpr,
  },
  UnknownListName {
    name: String,
  },
  Unexpected {
    expected: String,
    found: String,
  },
}

impl ParserError {
  pub fn expected(expected: impl Into<String>, found: impl Into<String>) -> Self {
    Self::Unexpected {
      expected: expected.into(),
      found: found.into(),
    }
  }
}

pub struct Parser {
  inner: std::iter::Peekable<std::vec::IntoIter<SExpr>>,
  recovarable_errors: Vec<ParserError>,
}

impl Parser {
  pub fn new(sexprs: SExprList) -> Self {
    Self {
      inner: sexprs.0.into_iter().peekable(),
      recovarable_errors: Vec::new(),
    }
  }

  pub fn peek(&mut self) -> Option<&SExpr> {
    self.inner.peek()
  }

  pub fn peek_symbol_str(&mut self) -> Result<String, ParserError> {
    match self.peek() {
      Some(SExpr::Symbol(SExprSymbol(name))) => Ok(name.clone()),
      None => Err(ParserError::UnexpectedEnd),
      Some(expr) => Err(ParserError::UnexpectedSExpr {
        expected: "Symbol".to_string(),
        found: expr.clone(),
      }),
    }
  }

  pub fn next_maybe(&mut self) -> Option<SExpr> {
    self.inner.next()
  }

  pub fn next_any(&mut self) -> Result<SExpr, ParserError> {
    self.inner.next().ok_or(ParserError::UnexpectedEnd)
  }

  pub fn next_expect<T: Expectable>(&mut self) -> Result<T, ParserError> {
    let next = self.next_any()?;
    T::expect(next)
  }

  pub fn next_expect_u32(&mut self) -> Result<u32, ParserError> {
    match self.next_any()? {
      SExpr::Float(d) => Ok(d as u32),
      SExpr::Hex(d) => Ok(d as u32),
      expr => Err(ParserError::UnexpectedSExpr {
        expected: "hex or float".to_string(),
        found: expr,
      }),
    }
  }

  pub fn next_expect_maybe<T: Expectable>(&mut self) -> Result<Option<T>, ParserError> {
    if let Some(next) = self.next_maybe() {
      T::expect(next).map(|me| Some(me))
    } else {
      Ok(None)
    }
  }

  pub fn next_parser_maybe(&mut self) -> Result<Option<Self>, ParserError> {
    let list = self.next_expect_maybe::<SExprList>()?;
    Ok(list.map(|l| l.into_parser()))
  }

  pub fn next_parser(&mut self) -> Result<Self, ParserError> {
    let list = self.next_expect::<SExprList>()?;
    Ok(list.into_parser())
  }

  pub fn next_parse_maybe<T: ParseableFromList>(&mut self) -> Result<Option<T>, ParserError> {
    let Some(list) = self.next_expect_maybe::<SExprList>()? else {
      return Ok(None);
    };

    Ok(Some(T::parse(list.into_parser())?))
  }

  pub fn next_parse_vec<T: ParseableFromList>(&mut self) -> Result<Vec<T>, ParserError> {
    let mut result = Vec::new();
    while let Some(value) = self.next_parse_maybe::<T>()? {
      result.push(value);
    }
    Ok(result)
  }

  ///
  /// Expects the next expression to be a list and attempts to parse it into the specified type.
  /// The type must implement the `ParseableFromList` trait.
  /// If the next expression is not a list or cannot be parsed into the specified type,
  /// it returns an error.
  ///  
  pub fn next_parse<T: ParseableFromList>(&mut self) -> Result<T, ParserError> {
    let list: SExprList = self.next_expect()?;
    T::parse(Parser::new(list))
  }

  pub fn next_symbol(&mut self) -> Result<String, ParserError> {
    let next: SExprSymbol = self.next_expect()?;
    Ok(next.0)
  }

  pub fn next_symbol_is(&mut self, name: impl Into<String>) -> Result<String, ParserError> {
    let next: SExprSymbol = self.next_expect()?;
    let name = name.into();
    if next.0 != name {
      Err(ParserError::UnexpectedSExpr {
        expected: format!("symbol '{name}'",),
        found: SExpr::Symbol(next),
      })
    } else {
      Ok(next.0)
    }
  }

  pub fn error(&mut self, error: ParserError) {
    self.recovarable_errors.push(error);
  }

  pub fn error_unexpected(&mut self, expected: impl Into<String>, found: impl Into<String>) {
    self.recovarable_errors.push(ParserError::Unexpected {
      expected: expected.into(),
      found: found.into(),
    });
  }
  pub fn error_unknown(&mut self, name: impl Into<String>) {
    self
      .recovarable_errors
      .push(ParserError::UnknownListName { name: name.into() });
  }

  pub fn expect_end(self) -> Result<(), ParserError> {
    if self.inner.len() > 0 {
      Err(ParserError::UnexpectedLeftover {
        leftover: self.inner.collect(),
      })
    } else {
      Ok(())
    }
  }
}

/// Consumes the parser
pub trait ParseableFromList: Sized {
  fn parse(parser: Parser) -> Result<Self, ParserError>;
}

pub trait Expectable: Sized {
  fn expect(parser: SExpr) -> Result<Self, ParserError>;
}
