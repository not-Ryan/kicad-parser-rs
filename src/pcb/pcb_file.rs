use crate::{
  parser::{ParseableFromList, ParserError},
  sexpr::{SExpr, SExprList, SExprSymbol},
};

pub fn parse_pcb_file(input: &str) -> Result<PcbFile, ParserError> {
  let sexprs = crate::sexpr::parse_sexpr(input).map_err(ParserError::SExpressionError)?;
  let parser = crate::parser::Parser::new(sexprs);
  PcbFile::parse(parser)
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
}

impl ParseableFromList for PcbFile {
  fn parse(mut parser: crate::parser::Parser) -> Result<Self, ParserError> {
    let mut pcb_file = PcbFile::default();

    parser.next_symbol_is("kicad_pcb")?;

    while let Some(mut parser) = parser.next_parser_maybe()? {
      match parser.next_symbol()?.as_str() {
        "version" => {
          let version: f64 = parser.next_expect()?;
          pcb_file.version = (version as u64).to_string();
        }

        "generator" => pcb_file.generator = parser.next_expect()?,
        "generator_version" => pcb_file.generator_version = parser.next_expect()?,
        "paper" => pcb_file.paper = parser.next_expect()?,

        "general" => pcb_file.general = parser.next_parse()?,
        "layers" => pcb_file.layers = parser.next_parse_vec()?,
        "net" => {
          pcb_file.nets.push(PcbNet {
            ordinal: parser.next_expect_u32()?,
            name: parser.next_expect()?,
          });
        }

        name => parser.error_unexpected("named list", format!("unknown name: {name}")),
      }

      // parser.expect_end()?;
    }

    // let while

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

#[derive(Default, Debug, Clone)]
pub struct PcbFileGeneral {
  /// The thickness token attribute defines the overall board thickness.
  pub thickness: f64,
}

impl ParseableFromList for PcbFileGeneral {
  fn parse(mut parser: crate::parser::Parser) -> Result<Self, ParserError> {
    let mut general = PcbFileGeneral::default();

    while let Some(SExpr::List(list)) = parser.next_maybe() {
      let mut parser = list.into_parser();

      match parser.next_symbol()?.as_str() {
        "thickness" => general.thickness = parser.next_expect()?,
        name => parser.error_unexpected("named list", format!("unknown name: {name}")),
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

impl ParseableFromList for PcbLayer {
  fn parse(mut parser: crate::parser::Parser) -> Result<Self, ParserError> {
    let mut layer = PcbLayer::default();

    let id: f64 = parser.next_expect()?;
    layer.ordinal = id as u32;

    layer.name = parser.next_expect()?;
    layer.layer_type = {
      let layer_name: SExprSymbol = parser.next_expect()?;
      match layer_name.0.as_str() {
        "user" => PcbLayerType::User,
        "jumper" => PcbLayerType::Jumper,
        "mixed" => PcbLayerType::Mixed,
        "power" => PcbLayerType::Power,
        "signal" => PcbLayerType::Signal,
        _ => panic!("Unknown layer type: {}", layer_name.0),
      }
    };

    layer.user_name = parser.next_expect_maybe()?;

    Ok(layer)
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
