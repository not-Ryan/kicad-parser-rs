#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parse_number() {
    assert_eq!(parse_sexpr("42"), Ok(Sexpr::Number(42.0)));
    assert_eq!(parse_sexpr("  3.14 "), Ok(Sexpr::Number(3.14)));
  }

  #[test]
  fn test_parse_string() {
    assert_eq!(
      parse_sexpr("\"hello\""),
      Ok(Sexpr::String("hello".to_string()))
    );
    assert_eq!(
      parse_sexpr("\"abc123\""),
      Ok(Sexpr::String("abc123".to_string()))
    );
  }

  #[test]
  fn test_parse_empty_list() {
    assert_eq!(parse_sexpr("()"), Ok(Sexpr::List(vec![])));
  }

  #[test]
  fn test_parse_simple_list() {
    assert_eq!(
      parse_sexpr("(42 foo)"),
      Ok(Sexpr::List(vec![
        Sexpr::Number(42.0),
        Sexpr::Symbol("foo".to_string())
      ]))
    );
  }

  #[test]
  fn test_parse_nested_list() {
    assert_eq!(
      parse_sexpr("(1 (2 3) \"bar\")"),
      Ok(Sexpr::List(vec![
        Sexpr::Number(1.0),
        Sexpr::List(vec![Sexpr::Number(2.0), Sexpr::Number(3.0)]),
        Sexpr::String("bar".to_string())
      ]))
    );
  }

  #[test]
  fn test_parse_list_with_spaces() {
    assert_eq!(
      parse_sexpr(" ( 1   2   3 ) "),
      Ok(Sexpr::List(vec![
        Sexpr::Number(1.0),
        Sexpr::Number(2.0),
        Sexpr::Number(3.0)
      ]))
    );
  }

  #[test]
  fn test_parse_long_int() {
    assert_eq!(
      parse_sexpr(
        "(pcbplotparams
          (layerselection 0x00000000_00000000_55555555_5755f5ff)
          (layerselection 0x00000000_00000000_55555555_5755f5ff)
        )"
      ),
      Ok(Sexpr::NamedList(
        "pcbplotparams".to_string(),
        vec![
          Sexpr::Symbol("layerselection".to_string()),
          Sexpr::Hex(0x00000000_00000000_55555555_5755f5ff)
        ],
      ))
    );
  }

  #[test]
  fn test_parse_invalid_input() {
    assert!(parse_sexpr("(1 2").is_err());
    assert!(parse_sexpr("\"unterminated").is_err());
    assert!(parse_sexpr(")").is_err());
  }
}
