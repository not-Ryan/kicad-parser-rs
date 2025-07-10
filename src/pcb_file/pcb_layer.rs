use crate::{
  parser::ParserError,
  sexpr::{SExpr, SExprSymbol},
};

#[derive(Default, Debug, Clone)]
pub struct PcbLayer {
  /// The layer ORDINAL is an integer used to associate the layer stack ordering. This is mostly to ensure correct mapping when the number of layers is increased in the future.
  pub ordinal: u32,
  /// The NAME is the layer name defined for user interface display.
  pub name: String,
  /// The layer TYPE defines the type of layer and can be defined as jumper, mixed, power, signal, or user.
  pub layer_type: PcbLayerType,
  /// The optional USER_NAME attribute defines the custom user name.
  pub user_name: Option<String>,
}

impl TryFrom<SExpr> for Vec<PcbLayer> {
  type Error = ParserError;

  fn try_from(value: SExpr) -> Result<Self, Self::Error> {
    let mut list = value.as_list()?;
    let mut out = Self::new();
    crate::expect_eq!(list.next_symbol()?, "layers", "PcbLayer::try_from");

    while let Some(mut layer_list) = list.next_maybe_list()? {
      // ! Keep in mind the ordering is crucial here.
      out.push(PcbLayer {
        ordinal: layer_list.next_into()?,
        name: layer_list.next_into()?,
        layer_type: layer_list.next_into()?,
        user_name: layer_list.next_maybe_into()?,
      });
    }

    Ok(out)
  }
}

#[derive(Default, Debug, Clone)]
pub enum PcbLayerType {
  #[default]
  User,
  Jumper,
  Mixed,
  Power,
  Signal,
}

impl TryFrom<SExpr> for PcbLayerType {
  type Error = ParserError;

  fn try_from(value: SExpr) -> Result<Self, Self::Error> {
    let symbol: SExprSymbol = value.try_into()?;
    match symbol.0.as_str() {
      "user" => Ok(PcbLayerType::User),
      "jumper" => Ok(PcbLayerType::Jumper),
      "mixed" => Ok(PcbLayerType::Mixed),
      "power" => Ok(PcbLayerType::Power),
      "signal" => Ok(PcbLayerType::Signal),

      found => crate::error!("Valid PCB Layer", found),
    }
  }
}
