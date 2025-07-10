#[macro_export]
macro_rules! context {
  () => {
    format!("{}:{}", file!(), line!())
  };
}

#[macro_export]
macro_rules! error {
  ($expected: expr, $found: expr) => {
    return Err(
      ParserError::unexpected(format!("{:?}", $expected), format!("{:?}", $found))
        .add_context($crate::context!()),
    )
  };
  (SExpr, $expected: expr, $found: expr) => {
    return Err(
      ParserError::unexpected_sexpr(format!("{:?}", $expected), $found)
        .add_context($crate::context!()),
    )
  };
}

#[macro_export]
macro_rules! expect_eq {
  ($left: expr, $right: expr, $($error: tt)*) => {
    if $left != $right {
      $crate::error!($left, $right);
    }
  };
}

#[macro_export]
macro_rules! catch_all {
  ($name: expr) => {
    println!("Unaccounted sexpr in {}:{}: {:?}", file!(), line!(), $name)
  };
}
