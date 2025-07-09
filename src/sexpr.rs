use nom::{
  AsChar, Err, IResult, Parser,
  bits::complete::take,
  branch::alt,
  bytes::complete::{escaped, is_not, tag, take_while, take_while1},
  character::{
    anychar,
    complete::{alpha1, alphanumeric0, alphanumeric1, char, one_of},
  },
  combinator::{cut, map, map_res},
  error::{ContextError, ParseError, context},
  multi::separated_list0,
  number::complete::double,
  sequence::{delimited, preceded, terminated},
};
use nom_language::error::{VerboseError, VerboseErrorKind, convert_error};
// use nom::error::{VerboseError, convert_error};
use std::str;

#[derive(Debug, PartialEq)]
pub enum Sexpr {
  List(Vec<Sexpr>),
  String(String),
  Symbol(String),
  Number(f64),
  Hex(u64),
}

// Parses spaces
fn sp<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
  let chars = " \t\r\n";
  take_while(move |c| chars.contains(c))(i)
}

fn symbol<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
  i: &'a str,
) -> IResult<&'a str, &'a str, E> {
  take_while1(move |c: char| {
    c.is_alphanumeric() || c == '_' || c == '-' || c == '?' || c == '!' || c == '.'
  })(i)
}

fn quoted_string<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
  i: &'a str,
) -> IResult<&'a str, &'a str, E> {
  context(
    "string",
    alt((
      delimited(char('"'), is_not("\""), char('"')),
      // Or an empty string
      tag("\"\""),
    )),
  )
  .parse(i)
}

fn hexadecimal<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
  i: &'a str,
) -> IResult<&'a str, u64, E> {
  context(
    "hex",
    map(
      preceded(
        tag("0x"),
        take_while1(|s: char| s.is_hex_digit() || s == '_'),
      ),
      |raw: &str| {
        let stripped = raw.replace('_', "");
        u64::from_str_radix(&stripped, 16).unwrap()
      },
    ),
  )
  .parse(i)
}

#[test]
fn test_hexadecimal() {
  assert_eq!(
    hexadecimal::<VerboseError<&str>>("0x1234"),
    Ok(("", 0x1234))
  );
  assert_eq!(
    hexadecimal::<VerboseError<&str>>("0x00000000_00000000_55555555_5755f5ff"),
    Ok(("", 0x00000000_00000000_55555555_5755f5ff))
  );
  assert!(hexadecimal::<VerboseError<&str>>("1234").is_err());
}

fn list<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
  i: &'a str,
) -> IResult<&'a str, Vec<Sexpr>, E> {
  context(
    "list",
    preceded(
      char('('),
      cut(terminated(
        separated_list0(one_of(" \n\t"), sexpr),
        preceded(sp, char(')')),
      )),
    ),
  )
  .parse(i)
}

/// here, we apply the space parser before trying to parse a value
fn sexpr<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
  i: &'a str,
) -> IResult<&'a str, Sexpr, E> {
  preceded(
    sp,
    alt((
      map(list, Sexpr::List),
      map(quoted_string, |s| Sexpr::String(s.to_string())),
      map(hexadecimal, Sexpr::Hex),
      map(double, Sexpr::Number),
      map(symbol, |s| Sexpr::Symbol(s.to_string())),
    )),
  )
  .parse(i)
}

pub fn parse_sexpr(input: &str) -> Result<Sexpr, String> {
  match sexpr::<VerboseError<&str>>(input) {
    Ok((rest, expr)) if rest.trim().is_empty() => Ok(expr),
    Ok((rest, _)) => Err(format!("Unparsed input: '{rest:?}'")),
    Err(Err::Error(e)) | Err(Err::Failure(e)) => Err(convert_error(input, e)),
    Err(Err::Incomplete(_)) => Err("Incomplete input".to_string()),
  }
}

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
        )"
      ),
      Ok(Sexpr::List(vec![
        Sexpr::Symbol("pcbplotparams".to_string()),
        Sexpr::List(vec![
          Sexpr::Symbol("layerselection".to_string()),
          Sexpr::Hex(0x00000000_00000000_55555555_5755f5ff)
        ]),
      ]))
    );
  }

  #[test]
  fn test_parse_invalid_input() {
    assert!(parse_sexpr("(1 2").is_err());
    assert!(parse_sexpr("\"unterminated").is_err());
    assert!(parse_sexpr(")").is_err());
  }
}
