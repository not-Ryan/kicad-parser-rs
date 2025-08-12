#[derive(Default, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct PcbProperty {
  pub key: String,
  pub value: String,
}
