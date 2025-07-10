#[macro_export]
macro_rules! expect_eq {
  ($left: expr, $right: expr, $($error: tt)*) => {
    if $left != $right {
      return Err(ParserError::unexpected(
        format!("{:?}", $right),
        format!("{:?}", $left),
      ));
    }
  };
}
