use crate::{parser::ParserError, sexpr::SExpr};

#[derive(Default, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct PcbFileGeneral {
  /// The thickness token attribute defines the overall board thickness.
  pub thickness: f64,
}

impl TryFrom<SExpr> for PcbFileGeneral {
  type Error = ParserError;

  fn try_from(value: SExpr) -> Result<Self, Self::Error> {
    let mut list = value.as_list()?;
    let mut general = PcbFileGeneral::default();

    crate::expect_eq!(list.next_symbol()?, "general", "PcbFileGeneral::try_from");

    while let Some(mut list) = list.next_maybe_list()? {
      match list.next_symbol()?.as_str() {
        "thickness" => general.thickness = list.next_into()?,
        _name => {
          // TODO: Maybe log?
          // list.error_unexpected("named list", format!("unknown name: {name}")),
        }
      }
    }

    Ok(general)
  }
}
