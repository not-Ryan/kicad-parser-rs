use crate::{
  impl_from_into,
  parser::{ParserError, TryFromSExpr},
  sexpr::SExprSymbol,
};

use super::SExpr;

#[derive(Clone, Debug, PartialEq)]
pub struct SExprList(pub Vec<SExpr>);
impl_from_into!(SExprList, SExpr::List);

impl SExprList {
  pub fn as_sexpr(self) -> SExpr {
    SExpr::List(self)
  }

  /// Converts the entire `SExprList` into another type that implements `TryFrom<SExpr>`.
  ///
  /// This method should be used instead of `next_into` when you want to convert the whole list,
  /// not just the next element, into a target type. For example, if you have an S-expression like
  /// `(net 0 0)`, you can use this method to convert the entire list into a `PCBNet` struct.
  ///
  /// ## Do not use when
  /// The list has an element child that should be converted into a different type.
  /// For example, if you have an S-expression like `((net 0 0) (net 1 1))`, you should use
  /// `next_into` to convert each element into a `PCBNet` struct.
  ///
  /// # Errors
  ///
  /// Returns a `ParserError` if the conversion fails.
  pub fn as_sexpr_into<T: TryFromSExpr>(self) -> Result<T, ParserError> {
    T::try_from_with_context(self.as_sexpr())
  }

  pub fn peek_name_maybe(&self) -> Option<&str> {
    if let Some(SExpr::Symbol(SExprSymbol(name))) = self.0.first() {
      Some(name)
    } else {
      None
    }
  }

  pub fn peek_maybe(&self) -> Option<&SExpr> {
    self.0.first()
  }

  pub fn peek(&self) -> Result<&SExpr, ParserError> {
    self.peek_maybe().ok_or_else(|| ParserError::UnexpectedEnd)
  }

  /// Returns a reference to the name of the first element in the `SExprList` if it is a symbol.
  /// This does not consume move the cursor of the list, allowing you to check the name without modifying the list.
  ///
  /// **Important**: The name symbol will still be the `next` element in the list after this call.
  ///
  /// This method checks the first element of the list. If the first element is a `Symbol`,
  /// it returns its string value. If the first element is not a symbol or the list is empty,
  /// it returns a `ParserError::UnexpectedSExpr`.
  ///
  /// # Errors
  ///
  /// Returns a `ParserError::UnexpectedSExpr` if the first element is not a symbol or the list is empty.
  ///
  /// # Example
  ///
  /// ```rust
  /// let list = SExprList::from(vec![SExpr::Symbol(SExprSymbol("foo".to_string()))]);
  /// assert_eq!(list.peek_name().unwrap(), "foo");
  /// ```
  pub fn peek_name(&self) -> Result<&str, ParserError> {
    self.peek_name_maybe().ok_or_else(|| {
      ParserError::unexpected_sexpr(
        "peek_name(Symbol)",
        format!(
          "{:?}",
          self
            .0
            .first()
            .cloned()
            .unwrap_or(SExpr::List(SExprList(vec![]))),
        ),
      )
    })
  }

  pub fn discard(&mut self, amount: usize) -> Result<&mut Self, ParserError> {
    if amount > self.0.len() {
      return Err(ParserError::UnexpectedEnd);
    }

    self.0.drain(0..amount);
    Ok(self)
  }

  pub fn next_maybe(&mut self) -> Option<SExpr> {
    self.0.pop()
  }

  pub fn next_maybe_into<T>(&mut self) -> Result<Option<T>, ParserError>
  where
    T: TryFromSExpr,
  {
    let Some(expr) = self.next_maybe() else {
      return Ok(None);
    };

    Ok(Some(expr.try_into()?))
  }

  pub fn next_maybe_list(&mut self) -> Result<Option<SExprList>, ParserError> {
    self.next_maybe_into()
  }

  pub fn next_any(&mut self) -> Result<SExpr, ParserError> {
    if let Some(expr) = self.next() {
      Ok(expr)
    } else {
      Err(ParserError::UnexpectedEnd)
    }
  }

  /// Retrieves the next child element from the `SExprList` and attempts to convert it into the specified type `T`.
  ///
  /// This method should be used when you want to process each child element of the list individually,
  /// converting each one into the target type. For example, if you have an S-expression like
  /// `((net 0 0) (net 1 1))`, you can use this method in a loop to convert each child list into a `PCBNet` struct.
  ///
  /// # Errors
  ///
  /// Returns a `ParserError` if there are no more elements in the list or if the conversion fails.
  ///
  /// # Example
  ///
  /// ```rust
  /// while let Ok(net) = sexpr_list.next_into::<PCBNet>() {
  ///     // process net
  /// }
  /// ```
  ///
  /// ## Do not use when
  /// You want to convert the entire list at once into a type. In that case, use [`as_sexpr_into`] instead.
  pub fn next_into<T>(&mut self) -> Result<T, ParserError>
  where
    T: TryFromSExpr,
  {
    T::try_from_with_context(self.next_any()?)
  }

  pub fn next_symbol(&mut self) -> Result<SExprSymbol, ParserError> {
    self.next_into()
  }

  pub fn next_list(&mut self) -> Result<SExprList, ParserError> {
    self.next_into()
  }

  pub fn expect_end(&self) -> Result<(), ParserError> {
    if self.0.is_empty() {
      Ok(())
    } else {
      Err(ParserError::UnexpectedLeftover {
        leftover: self.0.clone(),
      })
    }
  }
}

impl Iterator for SExprList {
  type Item = SExpr;

  fn next(&mut self) -> Option<Self::Item> {
    self.0.pop()
  }
}
