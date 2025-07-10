use crate::{parser::ParserError, sexpr::SExpr};

#[derive(Default, Debug, Clone)]
pub struct PcbNet {
  pub ordinal: u32,
  pub name: String,
}

impl TryFrom<SExpr> for PcbNet {
  type Error = ParserError;

  fn try_from(value: SExpr) -> Result<Self, Self::Error> {
    let mut list = value.as_list()?;
    let mut net = PcbNet::default();

    crate::expect_eq!(list.next_symbol()?, "net", "PcbNet::try_from");
    net.ordinal = list.next_into()?;
    net.name = list.next_into()?;

    Ok(net)
  }
}
