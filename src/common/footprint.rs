use std::collections::HashMap;

use crate::{
  common::{GetBoundingBox, Graphic, Point, Position},
  parser::ParserError,
  sexpr::{SExpr, SExprValue},
};

/// Text effects for controlling text display
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct TextEffects {
  /// Font settings
  pub font: Font,
  /// Text justification
  pub justify: Option<Justify>,
  /// Whether text is mirrored (PCB/Footprint only)
  pub mirror: bool,
  /// Whether text is hidden
  pub hide: bool,
}

/// Font definition
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Font {
  /// Font family name or "KiCad Font"
  pub face: Option<String>,
  /// Font height and width
  pub size: (f64, f64),
  /// Line thickness
  pub thickness: f64,
  /// Bold flag
  pub bold: bool,
  /// Italic flag
  pub italic: bool,
  /// Line spacing ratio
  pub line_spacing: Option<f64>,
}

/// Text justification options
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Justify {
  /// Horizontal justification
  pub horizontal: Option<HorizontalJustify>,
  /// Vertical justification
  pub vertical: Option<VerticalJustify>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum HorizontalJustify {
  Left,
  Right,
  Center,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum VerticalJustify {
  Top,
  Bottom,
  Center,
}

/// Universally unique identifier
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Uuid(pub String);

impl TryFrom<SExpr> for Uuid {
  type Error = ParserError;

  fn try_from(value: SExpr) -> Result<Self, Self::Error> {
    let mut list = value.as_list()?;
    crate::expect_eq!(list.next_symbol()?, "uuid", "Uuid::try_from");

    let uuid_str: String = list.next_into()?;
    if uuid_str.is_empty() {
      return Err(ParserError::unexpected("Non-empty UUID", uuid_str));
    }

    list.expect_end()?;
    Ok(Uuid(uuid_str))
  }
}

/// Canonical layer names
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Layer(String);

impl TryFrom<SExpr> for Layer {
  type Error = ParserError;

  fn try_from(value: SExpr) -> Result<Self, Self::Error> {
    let mut list = value.as_list()?;
    crate::expect_eq!(list.next_symbol()?, "layer", "Layer::try_from");

    let value: SExprValue = list.next_into()?;
    list.expect_end()?;

    Ok(Self(value.to_string()))
  }
}

impl TryFrom<SExpr> for Vec<Layer> {
  type Error = ParserError;

  fn try_from(value: SExpr) -> Result<Self, Self::Error> {
    let mut list = value.as_list()?;
    crate::expect_eq!(list.next_symbol()?, "layers", "Layer::try_from");

    let mut out = Self::new();
    while let Some(value) = list.next_maybe_into::<SExprValue>()? {
      out.push(Layer(value.to_string()));
    }

    Ok(out)
  }
}

impl PartialEq<str> for Layer {
  fn eq(&self, other: &str) -> bool {
    self.0 == other
  }
}

/// Zone connection types
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum ZoneConnect {
  /// Pad not connected to zone
  None = 0,
  /// Pad connected to zone using thermal relief
  Thermal = 1,
  /// Pad connected to zone using solid fill
  Solid = 2,
}

/// Footprint attributes
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct FootprintAttributes {
  /// Footprint type (SMD or through-hole)
  pub footprint_type: FootprintType,
  /// Board-only flag (no schematic symbol reference)
  pub board_only: bool,
  /// Exclude from position files
  pub exclude_from_pos_files: bool,
  /// Exclude from bill of materials
  pub exclude_from_bom: bool,
  /// Do not populate this footprint in the BOM
  pub do_not_populate: bool,
}

impl TryFrom<SExpr> for FootprintAttributes {
  type Error = ParserError;

  fn try_from(value: SExpr) -> Result<Self, Self::Error> {
    let mut list = value.as_list()?;
    crate::expect_eq!(list.next_symbol()?, "attr", "FootprintAttributes::try_from");

    let mut attributes = Self::default();
    while let Some(next) = list.next_maybe_symbol()? {
      match next.as_str() {
        "smd" => attributes.footprint_type = FootprintType::Smd,
        "through_hole" => attributes.footprint_type = FootprintType::ThroughHole,
        "board_only" => attributes.board_only = true,
        "exclude_from_pos_files" => attributes.exclude_from_pos_files = true,
        "exclude_from_bom" => attributes.exclude_from_bom = true,
        "dnp" | "do_not_populate" => attributes.do_not_populate = true,
        name => crate::catch_all!(name),
      }
    }

    Ok(attributes)
  }
}

/// Footprint type classification
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum FootprintType {
  #[default]
  Smd,
  ThroughHole,
}

/// 3D model definition
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Model3D {
  /// Path to 3D model file
  pub file: String,
  /// 3D position offset
  pub position: (f64, f64, f64),
  /// Scale factors for each axis
  pub scale: (f64, f64, f64),
  /// Rotation for each axis
  pub rotation: (f64, f64, f64),
}

/// Main footprint definition
/// Prior to version 6, this was called `module`
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Footprint {
  /// Library link (for board footprints)
  pub library_link: Option<String>,
  /// Locked flag - cannot be edited
  pub locked: bool,
  /// Placed flag - indicates footprint placement status
  pub placed: bool,
  /// Layer the footprint is placed on
  pub layer: Layer,
  /// Last edit time
  pub tedit: Option<String>,
  /// Unique identifier (board footprints only)
  pub uuid: Option<Uuid>,
  /// Position and rotation
  pub position: Option<Position>,
  /// Search tags
  pub tags: Option<String>,
  /// Description
  pub description: Option<String>,
  /// Properties
  pub properties: HashMap<String, String>,
  /// Hierarchical path (board footprints only)
  pub path: Option<String>,
  /// Autoplace cost for 90° rotation
  pub autoplace_cost90: Option<i32>,
  /// Autoplace cost for 180° rotation
  pub autoplace_cost180: Option<i32>,
  /// Solder mask margin override
  pub solder_mask_margin: Option<f64>,
  /// Solder paste margin override
  pub solder_paste_margin: Option<f64>,
  /// Solder paste ratio override
  pub solder_paste_ratio: Option<f64>,
  /// Clearance override
  pub clearance: Option<f64>,
  /// Zone connection override
  pub zone_connect: Option<ZoneConnect>,
  /// Thermal relief width override
  pub thermal_width: Option<f64>,
  /// Thermal relief gap override
  pub thermal_gap: Option<f64>,
  /// Footprint attributes
  pub attributes: Option<FootprintAttributes>,
  /// Private layers
  pub private_layers: Vec<Layer>,
  /// Net-tie pad groups
  pub net_tie_pad_groups: Vec<Vec<String>>,
  /// Graphic items
  pub graphics: Vec<Graphic>,
  /// Pads
  pub pads: Vec<Pad>,
  /// Keep-out zones
  pub zones: Vec<Zone>,
  /// Grouped objects
  pub groups: Vec<Group>,
  /// 3D models
  pub models: Vec<Model3D>,
}

impl TryFrom<SExpr> for Footprint {
  type Error = ParserError;

  fn try_from(value: SExpr) -> Result<Self, Self::Error> {
    let mut list = value.as_list()?;
    crate::expect_eq!(list.next_symbol()?, "footprint", "Footprint::try_from");

    let mut footprint = Footprint::default();

    while let Some(next) = list.next_maybe() {
      match next {
        SExpr::Value(value) => footprint.library_link = Some(value.0),
        SExpr::Symbol(symbol) if symbol == "locked" => footprint.locked = true,
        SExpr::Symbol(symbol) if symbol == "placed" => footprint.placed = true,

        SExpr::List(mut list) => match list.peek_name()? {
          "uuid" => footprint.uuid = Some(list.as_sexpr_into()?),
          "layer" => footprint.layer = list.as_sexpr_into()?,
          // TEDIT?
          "descr" | "description" => footprint.description = Some(list.discard(1)?.next_into()?),
          "at" => footprint.position = Some(list.as_sexpr_into()?),
          "tags" => footprint.tags = Some(list.discard(1)?.next_into()?),
          "path" => footprint.path = Some(list.discard(1)?.next_into()?),

          // TODO: Where do these go?
          // "sheetname" => footprint.name = Some(list.discard(1)?.next_into()?),
          // "sheetfile" => footprint.sheetfile = Some(list.discard(1)?.next_into()?),
          "attr" => footprint.attributes = Some(list.as_sexpr_into()?),
          "pad" => footprint.pads.push(list.as_sexpr_into()?),

          "property" => {
            list.discard(1)?; // Discard the "property" keyword
            let key = match list.next_any()? {
              SExpr::Value(value) => value.0,
              SExpr::Symbol(symbol) => symbol.0,
              got => return Err(ParserError::unexpected_sexpr("Value or Symbol", got)),
            };

            let value: String = list.next_into()?;
            footprint.properties.insert(key, value);
          }

          name if name.starts_with("fp_") => footprint.graphics.push(list.as_sexpr_into()?),

          name => crate::catch_all!(name),
        },
        name => crate::catch_all!(name),
      }
    }

    Ok(footprint)
  }
}

impl GetBoundingBox for Footprint {
  fn bounding_box(&self) -> crate::common::BoundingBox {
    let mut bounding = crate::common::BoundingBox::default();
    for graphic in &self.graphics {
      if graphic.layer() == "F.Fab" {
        continue;
      }

      bounding.envelop(&graphic.bounding_box());
    }

    let position = self.position.as_ref();
    let x = position.map_or(0.0, |p| p.x);
    let y = position.map_or(0.0, |p| p.y);
    bounding.move_by(x, y);
    bounding
  }
}

/// Footprint pad
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Pad {
  /// Pad number
  pub number: String,
  /// Pad type
  pub pad_type: PadType,
  /// Pad shape
  pub shape: PadShape,
  /// Position and rotation
  pub position: Position,
  /// Locked flag
  pub locked: bool,
  /// Pad size (width, height)
  pub size: (f64, f64),
  /// Drill definition
  pub drill: Option<Drill>,
  /// Layers the pad resides on
  pub layers: Vec<Layer>,
  /// Special properties
  pub properties: Vec<PadProperty>,
  /// Remove unused layers flag
  pub remove_unused_layers: bool,
  /// Keep end layers flag
  pub keep_end_layers: bool,
  /// Round rectangle ratio (0-1)
  pub roundrect_rratio: Option<f64>,
  /// Chamfer ratio (0-1)
  pub chamfer_ratio: Option<f64>,
  /// Chamfered corners
  pub chamfer: Vec<PadCorner>,
  /// Net connection
  pub net: Option<(i32, String)>,
  /// Unique identifier
  pub uuid: Uuid,
  /// Pin function name
  pub pin_function: Option<String>,
  /// Pin type
  pub pin_type: Option<String>,
  /// Die length
  pub die_length: Option<f64>,
  /// Solder mask margin override
  pub solder_mask_margin: Option<f64>,
  /// Solder paste margin override
  pub solder_paste_margin: Option<f64>,
  /// Solder paste margin ratio override
  pub solder_paste_margin_ratio: Option<f64>,
  /// Clearance override
  pub clearance: Option<f64>,
  /// Zone connection override
  pub zone_connection: Option<ZoneConnect>,
  /// Thermal width override
  pub thermal_width: Option<f64>,
  /// Thermal gap override
  pub thermal_gap: Option<f64>,
  /// Custom pad options
  pub custom_options: Option<CustomPadOptions>,
  /// Custom pad primitives
  pub custom_primitives: Option<CustomPadPrimitives>,
}

impl TryFrom<SExpr> for Pad {
  type Error = ParserError;
  fn try_from(value: SExpr) -> Result<Self, Self::Error> {
    let mut list = value.as_list()?;
    let mut pad = Self::default();

    crate::expect_eq!(list.next_symbol()?, "pad", "FootprintLine::try_from");

    while let Some(list) = list.next_maybe() {
      match list {
        SExpr::Value(value) => pad.number = value.0,
        SExpr::Symbol(s) if s == "locked" => pad.locked = false,

        SExpr::Symbol(s) if s == "smd" => pad.pad_type = PadType::Smd,
        SExpr::Symbol(s) if s == "connect" => pad.pad_type = PadType::Connect,
        SExpr::Symbol(s) if s == "thru_hole" => pad.pad_type = PadType::ThroughHole,
        SExpr::Symbol(s) if s == "np_thru_hole" => pad.pad_type = PadType::NonPlatedThroughHole,

        SExpr::Symbol(s) if s == "oval" => pad.shape = PadShape::Oval,
        SExpr::Symbol(s) if s == "circle" => pad.shape = PadShape::Circle,
        SExpr::Symbol(s) if s == "custom" => pad.shape = PadShape::Custom,
        SExpr::Symbol(s) if s == "rect" => pad.shape = PadShape::Rectangle,
        SExpr::Symbol(s) if s == "trapezoid" => pad.shape = PadShape::Trapezoid,
        SExpr::Symbol(s) if s == "roundrect" => pad.shape = PadShape::RoundedRectangle,

        SExpr::List(mut attr) => match attr.peek_name()? {
          "size" => {
            attr.discard(1)?; // Discard the "size" keyword
            let x: f64 = attr.next_into()?;
            let y: f64 = attr.next_into()?;
            pad.size = (x, y)
          }

          "at" => pad.position = attr.as_sexpr_into()?,
          "uuid" => pad.uuid = attr.as_sexpr_into()?,
          "layers" => pad.layers = attr.as_sexpr_into()?,
          "net" => {
            attr.discard(1)?; // Discard the "net" keyword
            let net_id: i32 = attr.next_into()?;
            let net_name: String = attr.next_into()?;
            pad.net = Some((net_id, net_name));
          }
          "pintype" => {
            attr.discard(1)?; // Discard the "pintype" keyword
            pad.pin_type = Some(attr.next_into()?);
          }
          name => crate::catch_all!(name),
        },
        name => crate::catch_all!(name),
      }
    }

    Ok(pad)
  }
}

/// Pad types
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum PadType {
  #[default]
  ThroughHole,
  Smd,
  Connect,
  NonPlatedThroughHole,
}

/// Pad shapes
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum PadShape {
  Circle,
  #[default]
  Rectangle,
  Oval,
  Trapezoid,
  RoundedRectangle,
  Custom,
}

/// Pad properties
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum PadProperty {
  #[default]
  Heatsink,
  Castellated,
}

/// Pad corners for chamfering
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum PadCorner {
  #[default]
  TopLeft,
  TopRight,
  BottomLeft,
  BottomRight,
}

/// Drill definition
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Drill {
  /// Oval drill flag
  pub oval: bool,
  /// Drill diameter
  pub diameter: f64,
  /// Width for oval drills
  pub width: Option<f64>,
  /// Drill offset from pad center
  pub offset: Option<Point>,
}

/// Custom pad options
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct CustomPadOptions {
  /// Clearance type
  pub clearance: CustomPadClearance,
  /// Anchor pad shape
  pub anchor: PadShape,
}

/// Custom pad clearance types
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum CustomPadClearance {
  Outline,
  ConvexHull,
}

/// Custom pad primitives
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct CustomPadPrimitives {
  /// Graphical items defining the pad shape
  pub graphics: Vec<PadGraphic>,
  /// Line width for graphics
  pub width: f64,
  /// Fill flag
  pub fill: bool,
}

/// Graphics items for custom pads
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum PadGraphic {
  Line {
    start: Point,
    end: Point,
  },
  Rectangle {
    start: Point,
    end: Point,
  },
  Circle {
    center: Point,
    end: Point,
  },
  Arc {
    start: Point,
    mid: Point,
    end: Point,
  },
  Polygon {
    points: Vec<Point>,
  },
}

/// Zone definition (placeholder)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Zone {
  // TODO: Implement zone structure
}

/// Group definition (placeholder)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Group {
  // TODO: Implement group structure
}
