//! # KiCad Footprint Parser
//!
//! This module provides Rust structs for parsing and representing KiCad footprints
//! and their associated graphics, pads, and other elements.
//!
//! ## Key Structures
//!
//! - [`Footprint`]: The main footprint container
//! - [`Pad`]: Individual footprint pads
//! - [`FootprintGraphic`]: Graphics elements (text, lines, shapes, etc.)
//! - [`Position`]: X/Y coordinates with optional rotation
//! - [`Layer`]: KiCad canonical layer names
//!
//! ## Example Usage
//!
//! ```rust
//! use kicad_parser::common::*;
//!
//! // Create a simple SMD footprint
//! let mut footprint = Footprint::new(Layer::FSilkS);
//! footprint.description = Some("Simple SMD footprint".to_string());
//!
//! // Add an SMD pad
//! let pad = Pad::new_smd(
//!     "1".to_string(),
//!     PadShape::Rectangle,
//!     Position::new(0.0, 0.0),
//!     (1.0, 0.5),  // 1mm x 0.5mm
//!     vec![Layer::FCu, Layer::FPaste, Layer::FMask]
//! );
//! footprint.add_pad(pad);
//! ```
//!
//! ## S-Expression Format Compliance
//!
//! These structures are designed to match the KiCad s-expression file format
//! as documented in the KiCad file format specification. All coordinate values
//! are in millimeters, and angles are in degrees.

use crate::{expect_eq, parser::ParserError, sexpr::SExpr};

/// Position identifier defining X/Y coordinates and optional rotation angle
#[derive(Debug, Clone, PartialEq)]
pub struct Position {
  /// X coordinate in millimeters
  pub x: f64,
  /// Y coordinate in millimeters  
  pub y: f64,
  /// Optional rotation angle in degrees
  pub angle: Option<f64>,
}

impl TryFrom<SExpr> for Position {
  type Error = ParserError;
  fn try_from(value: SExpr) -> Result<Self, Self::Error> {
    let mut list = value.as_list()?;
    expect_eq!(list.next_symbol()?, "at", "Position::try_from");

    let x: f64 = list.next_into()?;
    let y: f64 = list.next_into()?;
    let angle: Option<f64> = list.next_maybe_into()?;
    list.expect_end()?;

    Ok(Position { x, y, angle })
  }
}

/// Coordinate point for use in point lists
#[derive(Debug, Clone, PartialEq)]
pub struct Point {
  /// X coordinate in millimeters
  pub x: f64,
  /// Y coordinate in millimeters
  pub y: f64,
}

/// Stroke definition for drawing outlines
#[derive(Debug, Clone, PartialEq)]
pub struct Stroke {
  /// Line width
  pub width: f64,
  /// Line style type
  pub line_type: StrokeType,
  /// Optional color (R, G, B, A)
  pub color: Option<(u8, u8, u8, u8)>,
}

/// Valid stroke line styles
#[derive(Debug, Clone, PartialEq)]
pub enum StrokeType {
  Dash,
  DashDot,
  DashDotDot,
  Dot,
  Default,
  Solid,
}

/// Text effects for controlling text display
#[derive(Debug, Clone, PartialEq)]
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
pub struct Justify {
  /// Horizontal justification
  pub horizontal: Option<HorizontalJustify>,
  /// Vertical justification
  pub vertical: Option<VerticalJustify>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum HorizontalJustify {
  Left,
  Right,
  Center,
}

#[derive(Debug, Clone, PartialEq)]
pub enum VerticalJustify {
  Top,
  Bottom,
  Center,
}

/// Universally unique identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Uuid(pub String);

impl TryFrom<SExpr> for Uuid {
  type Error = ParserError;

  fn try_from(value: SExpr) -> Result<Self, Self::Error> {
    let mut list = value.as_list()?;
    expect_eq!(list.next_symbol()?, "uuid", "Uuid::try_from");

    let uuid_str: String = list.next_into()?;
    if uuid_str.is_empty() {
      return Err(ParserError::unexpected("Non-empty UUID", uuid_str));
    }

    list.expect_end()?;
    Ok(Uuid(uuid_str))
  }
}

/// Property key-value pair
#[derive(Debug, Clone, PartialEq)]
pub struct Property {
  /// Property key
  pub key: String,
  /// Property value
  pub value: String,
}

/// Canonical layer names
#[derive(Default, Debug, Clone, PartialEq)]
pub enum Layer {
  // Copper layers
  #[default]
  FCu,
  BCu,
  In1Cu,
  In2Cu,
  In3Cu,
  In4Cu,
  In5Cu,
  In6Cu,
  In7Cu,
  In8Cu,
  In9Cu,
  In10Cu,
  In11Cu,
  In12Cu,
  In13Cu,
  In14Cu,
  In15Cu,
  In16Cu,
  In17Cu,
  In18Cu,
  In19Cu,
  In20Cu,
  In21Cu,
  In22Cu,
  In23Cu,
  In24Cu,
  In25Cu,
  In26Cu,
  In27Cu,
  In28Cu,
  In29Cu,
  In30Cu,

  // Technical layers
  BAdhes,
  FAdhes,
  BPaste,
  FPaste,
  BSilkS,
  FSilkS,
  BMask,
  FMask,

  // User layers
  DwgsUser,
  CmtsUser,
  Eco1User,
  Eco2User,

  // Special layers
  EdgeCuts,
  FCrtYd,
  BCrtYd,
  FFab,
  BFab,

  // User definable layers
  User1,
  User2,
  User3,
  User4,
  User5,
  User6,
  User7,
  User8,
  User9,
}

impl TryFrom<SExpr> for Layer {
  type Error = ParserError;

  fn try_from(value: SExpr) -> Result<Self, Self::Error> {
    let mut list = value.as_list()?;
    expect_eq!(list.next_symbol()?, "layer", "Layer::try_from");

    let layer = match list.next_symbol()?.as_str() {
      "F.Cu" => Layer::FCu,
      "B.Cu" => Layer::BCu,
      "In1.Cu" => Layer::In1Cu,
      "In2.Cu" => Layer::In2Cu,
      "In3.Cu" => Layer::In3Cu,
      "In4.Cu" => Layer::In4Cu,
      "In5.Cu" => Layer::In5Cu,
      "In6.Cu" => Layer::In6Cu,
      "In7.Cu" => Layer::In7Cu,
      "In8.Cu" => Layer::In8Cu,
      "In9.Cu" => Layer::In9Cu,
      "In10.Cu" => Layer::In10Cu,
      "In11.Cu" => Layer::In11Cu,
      "In12.Cu" => Layer::In12Cu,
      "In13.Cu" => Layer::In13Cu,
      "In14.Cu" => Layer::In14Cu,
      "In15.Cu" => Layer::In15Cu,
      "In16.Cu" => Layer::In16Cu,
      "In17.Cu" => Layer::In17Cu,
      "In18.Cu" => Layer::In18Cu,
      "In19.Cu" => Layer::In19Cu,
      "In20.Cu" => Layer::In20Cu,
      "In21.Cu" => Layer::In21Cu,
      "In22.Cu" => Layer::In22Cu,
      "In23.Cu" => Layer::In23Cu,
      "In24.Cu" => Layer::In24Cu,
      "In25.Cu" => Layer::In25Cu,
      "In26.Cu" => Layer::In26Cu,
      "In27.Cu" => Layer::In27Cu,
      "In28.Cu" => Layer::In28Cu,
      "In29.Cu" => Layer::In29Cu,
      "In30.Cu" => Layer::In30Cu,
      "B.Adhes" => Layer::BAdhes,
      "F.Adhes" => Layer::FAdhes,
      "B.Paste" => Layer::BPaste,
      "F.Paste" => Layer::FPaste,
      "B.SilkS" => Layer::BSilkS,
      "F.SilkS" => Layer::FSilkS,
      "B.Mask" => Layer::BMask,
      "F.Mask" => Layer::FMask,
      "Dwgs.User" => Layer::DwgsUser,
      "Cmts.User" => Layer::CmtsUser,
      "Eco1.User" => Layer::Eco1User,
      "Eco2.User" => Layer::Eco2User,
      "Edge.Cuts" => Layer::EdgeCuts,
      "F.CrtYd" => Layer::FCrtYd,
      "B.CrtYd" => Layer::BCrtYd,
      "F.Fab" => Layer::FFab,
      "B.Fab" => Layer::BFab,
      "User.1" => Layer::User1,
      "User.2" => Layer::User2,
      "User.3" => Layer::User3,
      "User.4" => Layer::User4,
      "User.5" => Layer::User5,
      "User.6" => Layer::User6,
      "User.7" => Layer::User7,
      "User.8" => Layer::User8,
      "User.9" => Layer::User9,

      s => return Err(ParserError::unexpected("Valid layer", s)),
    };

    list.expect_end()?;
    Ok(layer)
  }
}

/// Zone connection types
#[derive(Debug, Clone, PartialEq)]
pub enum ZoneConnect {
  /// Pad not connected to zone
  None = 0,
  /// Pad connected to zone using thermal relief
  Thermal = 1,
  /// Pad connected to zone using solid fill
  Solid = 2,
}

/// Footprint attributes
#[derive(Debug, Clone, PartialEq)]
pub struct FootprintAttributes {
  /// Footprint type (SMD or through-hole)
  pub footprint_type: Option<FootprintType>,
  /// Board-only flag (no schematic symbol reference)
  pub board_only: bool,
  /// Exclude from position files
  pub exclude_from_pos_files: bool,
  /// Exclude from bill of materials
  pub exclude_from_bom: bool,
}

/// Footprint type classification
#[derive(Debug, Clone, PartialEq)]
pub enum FootprintType {
  Smd,
  ThroughHole,
}

/// 3D model definition
#[derive(Debug, Clone, PartialEq)]
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
  pub properties: Vec<Property>,
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
  pub graphics: Vec<FootprintGraphic>,
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

    let mut footprint = Footprint::default();

    while let Some(next) = list.next_maybe() {
      match next {
        SExpr::Value(value) => footprint.library_link = Some(value.0),
        SExpr::Symbol(symbol) if symbol == "locked" => footprint.locked = true,
        SExpr::Symbol(symbol) if symbol == "placed" => footprint.placed = true,

        SExpr::List(mut list) => match list.peek_name()? {
          "uuid" => footprint.uuid = Some(list.as_sexpr_into()?),
          "layer" => footprint.layer = list.as_sexpr_into()?,

          "at" => footprint.position = Some(list.as_sexpr_into()?),
          "path" => footprint.path = Some(list.discard(1)?.next_into()?),

          // TODO: Where do these go?
          // "sheetname" => footprint.name = Some(list.discard(1)?.next_into()?),
          // "sheetfile" => footprint.sheetfile = Some(list.discard(1)?.next_into()?),
          // "attr" => footprint.attr = attr(list.discard(1)?.next_into()?),
          name => {
            // TODO: Unknown list name. Maybe log?
          }
        },

        _ => {}
      }
    }

    Ok(footprint)
  }
}

/// Footprint graphic items
#[derive(Debug, Clone, PartialEq)]
pub enum FootprintGraphic {
  Text(FootprintText),
  TextBox(FootprintTextBox),
  Line(FootprintLine),
  Rectangle(FootprintRectangle),
  Circle(FootprintCircle),
  Arc(FootprintArc),
  Polygon(FootprintPolygon),
  Curve(FootprintCurve),
}

/// Footprint text
#[derive(Debug, Clone, PartialEq)]
pub struct FootprintText {
  /// Text type
  pub text_type: FootprintTextType,
  /// Text content
  pub text: String,
  /// Position and angle
  pub position: Position,
  /// Unlocked orientation flag
  pub unlocked: bool,
  /// Layer
  pub layer: Layer,
  /// Hidden flag
  pub hide: bool,
  /// Text effects
  pub effects: TextEffects,
  /// Unique identifier
  pub uuid: Uuid,
}

/// Footprint text types
#[derive(Debug, Clone, PartialEq)]
pub enum FootprintTextType {
  Reference,
  Value,
  User,
}

/// Footprint text box (from version 7)
#[derive(Debug, Clone, PartialEq)]
pub struct FootprintTextBox {
  /// Locked flag
  pub locked: bool,
  /// Text content
  pub text: String,
  /// Start position (cardinal orientation)
  pub start: Option<Point>,
  /// End position (cardinal orientation)
  pub end: Option<Point>,
  /// Four corner points (non-cardinal orientation)
  pub points: Option<Vec<Point>>,
  /// Rotation angle
  pub angle: Option<f64>,
  /// Layer
  pub layer: Layer,
  /// Unique identifier
  pub uuid: Uuid,
  /// Text effects
  pub effects: TextEffects,
  /// Border stroke
  pub stroke: Option<Stroke>,
}

/// Footprint line
#[derive(Debug, Clone, PartialEq)]
pub struct FootprintLine {
  /// Start point
  pub start: Point,
  /// End point
  pub end: Point,
  /// Layer
  pub layer: Layer,
  /// Stroke definition
  pub stroke: Stroke,
  /// Locked flag
  pub locked: bool,
  /// Unique identifier
  pub uuid: Uuid,
}

/// Footprint rectangle
#[derive(Debug, Clone, PartialEq)]
pub struct FootprintRectangle {
  /// Upper left corner
  pub start: Point,
  /// Lower right corner
  pub end: Point,
  /// Layer
  pub layer: Layer,
  /// Stroke definition
  pub stroke: Stroke,
  /// Fill flag
  pub fill: bool,
  /// Locked flag
  pub locked: bool,
  /// Unique identifier
  pub uuid: Uuid,
}

/// Footprint circle
#[derive(Debug, Clone, PartialEq)]
pub struct FootprintCircle {
  /// Center point
  pub center: Point,
  /// End of radius
  pub end: Point,
  /// Layer
  pub layer: Layer,
  /// Stroke definition
  pub stroke: Stroke,
  /// Fill flag
  pub fill: bool,
  /// Locked flag
  pub locked: bool,
  /// Unique identifier
  pub uuid: Uuid,
}

/// Footprint arc
#[derive(Debug, Clone, PartialEq)]
pub struct FootprintArc {
  /// Start position
  pub start: Point,
  /// Midpoint along arc
  pub mid: Point,
  /// End position
  pub end: Point,
  /// Layer
  pub layer: Layer,
  /// Stroke definition
  pub stroke: Stroke,
  /// Locked flag
  pub locked: bool,
  /// Unique identifier
  pub uuid: Uuid,
}

/// Footprint polygon
#[derive(Debug, Clone, PartialEq)]
pub struct FootprintPolygon {
  /// Polygon outline points
  pub points: Vec<Point>,
  /// Layer
  pub layer: Layer,
  /// Stroke definition
  pub stroke: Stroke,
  /// Fill flag
  pub fill: bool,
  /// Locked flag
  pub locked: bool,
  /// Unique identifier
  pub uuid: Uuid,
}

/// Footprint curve (Cubic Bezier)
#[derive(Debug, Clone, PartialEq)]
pub struct FootprintCurve {
  /// Four control points of the Bezier curve
  pub points: [Point; 4],
  /// Layer
  pub layer: Layer,
  /// Stroke definition
  pub stroke: Stroke,
  /// Locked flag
  pub locked: bool,
  /// Unique identifier
  pub uuid: Uuid,
}

/// Footprint pad
#[derive(Debug, Clone, PartialEq)]
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

/// Pad types
#[derive(Debug, Clone, PartialEq)]
pub enum PadType {
  ThroughHole,
  Smd,
  Connect,
  NonPlatedThroughHole,
}

/// Pad shapes
#[derive(Debug, Clone, PartialEq)]
pub enum PadShape {
  Circle,
  Rectangle,
  Oval,
  Trapezoid,
  RoundedRectangle,
  Custom,
}

/// Pad properties
#[derive(Debug, Clone, PartialEq)]
pub enum PadProperty {
  Heatsink,
  Castellated,
}

/// Pad corners for chamfering
#[derive(Debug, Clone, PartialEq)]
pub enum PadCorner {
  TopLeft,
  TopRight,
  BottomLeft,
  BottomRight,
}

/// Drill definition
#[derive(Debug, Clone, PartialEq)]
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
pub struct CustomPadOptions {
  /// Clearance type
  pub clearance: CustomPadClearance,
  /// Anchor pad shape
  pub anchor: PadShape,
}

/// Custom pad clearance types
#[derive(Debug, Clone, PartialEq)]
pub enum CustomPadClearance {
  Outline,
  ConvexHull,
}

/// Custom pad primitives
#[derive(Debug, Clone, PartialEq)]
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
pub struct Zone {
  // TODO: Implement zone structure
}

/// Group definition (placeholder)
#[derive(Debug, Clone, PartialEq)]
pub struct Group {
  // TODO: Implement group structure
}
impl Position {
  /// Create a new position with X, Y coordinates
  pub fn new(x: f64, y: f64) -> Self {
    Self { x, y, angle: None }
  }

  /// Create a new position with X, Y coordinates and rotation angle
  pub fn with_angle(x: f64, y: f64, angle: f64) -> Self {
    Self {
      x,
      y,
      angle: Some(angle),
    }
  }
}

impl Point {
  /// Create a new point with X, Y coordinates
  pub fn new(x: f64, y: f64) -> Self {
    Self { x, y }
  }
}

impl Stroke {
  /// Create a new stroke with default solid style
  pub fn new(width: f64) -> Self {
    Self {
      width,
      line_type: StrokeType::Solid,
      color: None,
    }
  }

  /// Create a stroke with specified type
  pub fn with_type(width: f64, line_type: StrokeType) -> Self {
    Self {
      width,
      line_type,
      color: None,
    }
  }
}

impl Default for Font {
  fn default() -> Self {
    Self {
      face: None,
      size: (1.0, 1.0),
      thickness: 0.15,
      bold: false,
      italic: false,
      line_spacing: None,
    }
  }
}

impl Default for TextEffects {
  fn default() -> Self {
    Self {
      font: Font::default(),
      justify: None,
      mirror: false,
      hide: false,
    }
  }
}

impl Default for Uuid {
  fn default() -> Self {
    Self::new()
  }
}

impl Uuid {
  /// Create a new UUID (placeholder implementation)
  pub fn new() -> Self {
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .unwrap()
      .as_nanos();
    Self(format!("{timestamp:x}"))
  }
}

impl Pad {
  /// Create a new SMD pad
  pub fn new_smd(
    number: String,
    shape: PadShape,
    position: Position,
    size: (f64, f64),
    layers: Vec<Layer>,
  ) -> Self {
    Self {
      number,
      pad_type: PadType::Smd,
      shape,
      position,
      locked: false,
      size,
      drill: None,
      layers,
      properties: Vec::new(),
      remove_unused_layers: false,
      keep_end_layers: false,
      roundrect_rratio: None,
      chamfer_ratio: None,
      chamfer: Vec::new(),
      net: None,
      uuid: Uuid::new(),
      pin_function: None,
      pin_type: None,
      die_length: None,
      solder_mask_margin: None,
      solder_paste_margin: None,
      solder_paste_margin_ratio: None,
      clearance: None,
      zone_connection: None,
      thermal_width: None,
      thermal_gap: None,
      custom_options: None,
      custom_primitives: None,
    }
  }

  /// Create a new through-hole pad
  pub fn new_through_hole(
    number: String,
    shape: PadShape,
    position: Position,
    size: (f64, f64),
    drill: Drill,
    layers: Vec<Layer>,
  ) -> Self {
    Self {
      number,
      pad_type: PadType::ThroughHole,
      shape,
      position,
      locked: false,
      size,
      drill: Some(drill),
      layers,
      properties: Vec::new(),
      remove_unused_layers: false,
      keep_end_layers: false,
      roundrect_rratio: None,
      chamfer_ratio: None,
      chamfer: Vec::new(),
      net: None,
      uuid: Uuid::new(),
      pin_function: None,
      pin_type: None,
      die_length: None,
      solder_mask_margin: None,
      solder_paste_margin: None,
      solder_paste_margin_ratio: None,
      clearance: None,
      zone_connection: None,
      thermal_width: None,
      thermal_gap: None,
      custom_options: None,
      custom_primitives: None,
    }
  }
}

impl Drill {
  /// Create a round drill
  pub fn round(diameter: f64) -> Self {
    Self {
      oval: false,
      diameter,
      width: None,
      offset: None,
    }
  }

  /// Create an oval drill
  pub fn oval(diameter: f64, width: f64) -> Self {
    Self {
      oval: true,
      diameter,
      width: Some(width),
      offset: None,
    }
  }
}

impl std::fmt::Display for Layer {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let name = match self {
      Layer::FCu => "F.Cu",
      Layer::BCu => "B.Cu",
      Layer::In1Cu => "In1.Cu",
      Layer::In2Cu => "In2.Cu",
      Layer::In3Cu => "In3.Cu",
      Layer::In4Cu => "In4.Cu",
      Layer::In5Cu => "In5.Cu",
      Layer::In6Cu => "In6.Cu",
      Layer::In7Cu => "In7.Cu",
      Layer::In8Cu => "In8.Cu",
      Layer::In9Cu => "In9.Cu",
      Layer::In10Cu => "In10.Cu",
      Layer::In11Cu => "In11.Cu",
      Layer::In12Cu => "In12.Cu",
      Layer::In13Cu => "In13.Cu",
      Layer::In14Cu => "In14.Cu",
      Layer::In15Cu => "In15.Cu",
      Layer::In16Cu => "In16.Cu",
      Layer::In17Cu => "In17.Cu",
      Layer::In18Cu => "In18.Cu",
      Layer::In19Cu => "In19.Cu",
      Layer::In20Cu => "In20.Cu",
      Layer::In21Cu => "In21.Cu",
      Layer::In22Cu => "In22.Cu",
      Layer::In23Cu => "In23.Cu",
      Layer::In24Cu => "In24.Cu",
      Layer::In25Cu => "In25.Cu",
      Layer::In26Cu => "In26.Cu",
      Layer::In27Cu => "In27.Cu",
      Layer::In28Cu => "In28.Cu",
      Layer::In29Cu => "In29.Cu",
      Layer::In30Cu => "In30.Cu",
      Layer::BAdhes => "B.Adhes",
      Layer::FAdhes => "F.Adhes",
      Layer::BPaste => "B.Paste",
      Layer::FPaste => "F.Paste",
      Layer::BSilkS => "B.SilkS",
      Layer::FSilkS => "F.SilkS",
      Layer::BMask => "B.Mask",
      Layer::FMask => "F.Mask",
      Layer::DwgsUser => "Dwgs.User",
      Layer::CmtsUser => "Cmts.User",
      Layer::Eco1User => "Eco1.User",
      Layer::Eco2User => "Eco2.User",
      Layer::EdgeCuts => "Edge.Cuts",
      Layer::FCrtYd => "F.CrtYd",
      Layer::BCrtYd => "B.CrtYd",
      Layer::FFab => "F.Fab",
      Layer::BFab => "B.Fab",
      Layer::User1 => "User.1",
      Layer::User2 => "User.2",
      Layer::User3 => "User.3",
      Layer::User4 => "User.4",
      Layer::User5 => "User.5",
      Layer::User6 => "User.6",
      Layer::User7 => "User.7",
      Layer::User8 => "User.8",
      Layer::User9 => "User.9",
    };
    write!(f, "{name}")
  }
}
