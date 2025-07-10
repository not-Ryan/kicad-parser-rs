use nom::{
  AsChar, Err, IResult, Parser,
  branch::alt,
  bytes::complete::{is_not, tag, take_while, take_while1},
  character::complete::{char, one_of},
  combinator::{cut, map},
  error::{ContextError, ParseError, context},
  multi::separated_list0,
  number::complete::double,
  sequence::{delimited, preceded, separated_pair, terminated},
};

use nom_language::error::{VerboseError, convert_error};
use std::str;

use crate::sexpr::SExprList;

use super::SExpr;

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
) -> IResult<&'a str, Vec<SExpr>, E> {
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

fn named_list<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
  i: &'a str,
) -> IResult<&'a str, (&'a str, Vec<SExpr>), E> {
  context(
    "list",
    preceded(
      char('('),
      terminated(separated_pair(symbol, sp, list), preceded(sp, char(')'))),
    ),
  )
  .parse(i)
}

/// here, we apply the space parser before trying to parse a value
fn sexpr<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
  i: &'a str,
) -> IResult<&'a str, SExpr, E> {
  preceded(
    sp,
    alt((
      map(list, |items| SExpr::List(super::SExprList(items))),
      map(quoted_string, |s| {
        SExpr::Value(super::SExprValue(s.to_string()))
      }),
      map(hexadecimal, SExpr::Hex),
      map(double, SExpr::Float),
      map(symbol, |s| SExpr::Symbol(super::SExprSymbol(s.to_string()))),
    )),
  )
  .parse(i)
}

pub fn parse_sexpr(input: &str) -> Result<SExprList, String> {
  match sexpr::<VerboseError<&str>>(input) {
    Ok((rest, ..)) if !rest.trim().is_empty() => Err(format!("Unparsed input: '{rest:?}'")),

    Ok((.., SExpr::List(list))) => Ok(list),
    Ok((.., expr)) => Err(format!("Root must be list, found: '{expr:?}'")),

    Err(Err::Error(e)) | Err(Err::Failure(e)) => Err(convert_error(input, e)),
    Err(Err::Incomplete(_)) => Err("Incomplete input".to_string()),
  }
}
