mod pcb_layer;
pub use pcb_layer::*;

mod pcb_net;
pub use pcb_net::*;

mod pcb_property;
pub use pcb_property::*;

mod pcb_file_general;
pub use pcb_file_general::*;

mod pcb_setup;
pub use pcb_setup::*;

mod pcb_stack_settings;
pub use pcb_stack_settings::*;

use crate::{common::GetBoundingBox, parser::ParserError, sexpr::SExpr};

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
  pub graphics: Vec<crate::common::Graphic>,
}

impl TryFrom<SExpr> for PcbFile {
  type Error = ParserError;

  fn try_from(value: SExpr) -> Result<Self, Self::Error> {
    let mut list = value.as_list()?;

    let mut pcb_file = PcbFile::default();
    crate::expect_eq!(list.next_symbol()?, "kicad_pcb", "PcbFile::try_from");

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

        name if name.starts_with("gr_") => pcb_file.graphics.push(list.as_sexpr_into()?),

        _other => {
          // TODO: Maybe log?
          // list.error_unexpected("named list", format!("unknown name: {name}")),
        }
      }
    }

    Ok(pcb_file)
  }
}

impl GetBoundingBox for PcbFile {
  fn bounding_box(&self) -> crate::common::BoundingBox {
    let mut bounding = crate::common::BoundingBox::default();
    for graphics in &self.graphics {
      if graphics.layer() != "Edge.Cuts" {
        continue;
      }

      bounding.envelop(&graphics.bounding_box());
    }

    bounding
  }
}
