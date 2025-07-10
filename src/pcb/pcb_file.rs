use crate::{
  expect_eq,
  parser::ParserError,
  sexpr::{SExpr, SExprSymbol},
};

pub fn parse_pcb_file(input: &str) -> Result<PcbFile, ParserError> {
  let sexprs = crate::sexpr::parse_sexpr(input).map_err(|error| ParserError {
    found: error,
    kind: crate::parser::ParserErrorKind::SExpressionError,
    expected: "valid KiCad PCB file".to_string(),
    in_context: vec![crate::context!()],
    backtrace: backtrace::Backtrace::new(),
  })?;

  sexprs.as_sexpr_into()
}

#[derive(Default, Debug, Clone)]
pub struct PcbFile {
  pub version: String,
  pub generator: String,
  pub generator_version: String,
  pub paper: String,

  pub general: PcbFileGeneral,
  pub layers: Vec<PcbLayer>,
  pub properties: Vec<PcbProperty>,
  pub nets: Vec<PcbNet>,

  pub footprints: Vec<crate::common::Footprint>,
}

impl TryFrom<SExpr> for PcbFile {
  type Error = ParserError;

  fn try_from(value: SExpr) -> Result<Self, Self::Error> {
    let mut list = value.as_list()?;

    let mut pcb_file = PcbFile::default();
    expect_eq!(list.next_symbol()?, "kicad_pcb", "PcbFile::try_from");

    while let Some(mut list) = list.next_maybe_list()? {
      match list.peek_name()? {
        "version" => {
          let version: f64 = list.discard(1)?.next_into()?;
          pcb_file.version = (version as u64).to_string();
        }

        "generator" => pcb_file.generator = list.discard(1)?.next_into()?,
        "generator_version" => pcb_file.generator_version = list.discard(1)?.next_into()?,
        "paper" => pcb_file.paper = list.discard(1)?.next_into()?,

        "general" => pcb_file.general = list.as_sexpr_into()?,
        "layers" => pcb_file.layers = list.as_sexpr_into()?,
        "net" => pcb_file.nets.push(list.as_sexpr_into()?),
        "footprint" => pcb_file.footprints.push(list.as_sexpr_into()?),

        _other => {
          // TODO: Maybe log?
          // list.error_unexpected("named list", format!("unknown name: {name}")),
        }
      }
    }

    Ok(pcb_file)
  }
}

#[derive(Default, Debug, Clone)]
pub struct PcbProperty {
  pub key: String,
  pub value: String,
}

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

    expect_eq!(list.next_symbol()?, "net", "PcbNet::try_from");
    net.ordinal = list.next_into()?;
    net.name = list.next_into()?;

    Ok(net)
  }
}

#[derive(Default, Debug, Clone)]
pub struct PcbFileGeneral {
  /// The thickness token attribute defines the overall board thickness.
  pub thickness: f64,
}

impl TryFrom<SExpr> for PcbFileGeneral {
  type Error = ParserError;

  fn try_from(value: SExpr) -> Result<Self, Self::Error> {
    let mut list = value.as_list()?;
    let mut general = PcbFileGeneral::default();

    expect_eq!(list.next_symbol()?, "general", "PcbFileGeneral::try_from");

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
    expect_eq!(list.next_symbol()?, "layers", "PcbLayer::try_from");

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

// TODO: Implement `layer_stackup` using https://dev-docs.kicad.org/en/file-formats/sexpr-pcb/index.html#_stack_up_layer_settings
// The layer stack up definitions is a list of layer settings for each layer required to manufacture a board including the dielectric material between the actual layers defined in the board editor.
// layer_stackup: Vec<PcbLayerStackupSetting>,

#[derive(Default, Debug, Clone)]
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
pub enum EdgeConnectorSetting {
  #[default]
  Bevelled,
  Yes,
}
