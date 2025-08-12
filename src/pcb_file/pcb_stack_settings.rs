// TODO: Implement `layer_stackup` using https://dev-docs.kicad.org/en/file-formats/sexpr-pcb/index.html#_stack_up_layer_settings
// The layer stack up definitions is a list of layer settings for each layer required to manufacture a board including the dielectric material between the actual layers defined in the board editor.
// layer_stackup: Vec<PcbLayerStackupSetting>,
#[derive(Default, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct PcbStackUpSettings {
  /// The optional copper_finish token is a string that defines the copper finish used to manufacture the board.
  pub copper_finish: Option<String>,
  /// The optional dielectric_contraints token define if the board should meet all dielectric requirements.
  pub dielectric_constraints: Option<bool>,
  /// The optional edge_connector token defines if the board has an edge connector and if the edge connector is bevelled.
  pub edge_connector: Option<EdgeConnectorSetting>,
  /// The optional castellated_pads token defines if the board edges contain castellated pads.
  pub castellated_pads: Option<bool>,
  /// The optional edge_plating token defines if the board edges should be plated.
  pub edge_plating: Option<bool>,
}

#[derive(Default, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum EdgeConnectorSetting {
  #[default]
  Bevelled,
  Yes,
}
