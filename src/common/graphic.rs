use crate::{
  common::{BoundingBox, GetBoundingBox, Layer, Point, PointList, Position, Uuid},
  parser::ParserError,
  sexpr::SExpr,
};

/// Stroke definition for drawing outlines
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Stroke {
  /// Line width
  pub width: f64,
  /// Line style type
  pub line_type: StrokeType,
  /// Optional color (R, G, B, A)
  pub color: Option<RgbaColor>,
}

impl TryFrom<SExpr> for Stroke {
  type Error = ParserError;
  fn try_from(value: SExpr) -> Result<Self, Self::Error> {
    let mut list = value.as_list()?;
    let mut stroke = Self::default();
    crate::expect_eq!(list.next_symbol()?, "stroke", "Stroke::try_from");

    while let Some(mut list) = list.next_maybe_list()? {
      match list.peek_name()? {
        "width" => stroke.width = list.discard(1)?.next_into()?,
        "type" => stroke.line_type = list.as_sexpr_into()?,
        "color" => stroke.color = Some(list.as_sexpr_into()?),

        _ => {
          // Maybe log unknown attribute?
        }
      }
    }

    Ok(stroke)
  }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct RgbaColor(u8, u8, u8, u8);

impl TryFrom<SExpr> for RgbaColor {
  type Error = ParserError;
  fn try_from(value: SExpr) -> Result<Self, Self::Error> {
    let mut list = value.as_list()?;
    crate::expect_eq!(list.next_symbol()?, "color", "Color::try_from");

    let r: u8 = list.next_into()?;
    let g: u8 = list.next_into()?;
    let b: u8 = list.next_into()?;
    let a: u8 = list.next_into()?;
    list.expect_end()?;

    Ok(Self(r, g, b, a))
  }
}

/// Valid stroke line styles
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum StrokeType {
  #[default]
  Default,
  Solid,
  Dash,
  DashDot,
  DashDotDot,
  Dot,
}

impl TryFrom<SExpr> for StrokeType {
  type Error = ParserError;
  fn try_from(value: SExpr) -> Result<Self, Self::Error> {
    let mut list = value.as_list()?;
    crate::expect_eq!(list.next_symbol()?, "type", "StrokeType::try_from");

    Ok(match list.next_symbol()?.as_str() {
      "dash" => Self::Dash,
      "dash_dot" => Self::DashDot,
      "dash_dot_dot" => Self::DashDotDot,
      "dot" => Self::Dot,
      "default" => Self::Default,
      "solid" => Self::Solid,
      s => crate::error!("Valid stroke type", s),
    })
  }
}

/// Footprint graphic items
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum Graphic {
  Text(FootprintText),
  TextBox(FootprintTextBox),
  Line(FootprintLine),
  Rectangle(FootprintRectangle),
  Circle(FootprintCircle),
  Arc(FootprintArc),
  Polygon(FootprintPolygon),
  Curve(FootprintCurve),
}

impl Graphic {
  pub fn layer(&self) -> &Layer {
    match self {
      Self::Text(value) => &value.layer,
      Self::TextBox(value) => &value.layer,
      Self::Line(value) => &value.layer,
      Self::Rectangle(value) => &value.layer,
      Self::Circle(value) => &value.layer,
      Self::Arc(value) => &value.layer,
      Self::Polygon(value) => &value.layer,
      Self::Curve(value) => &value.layer,
    }
  }
}

impl GetBoundingBox for Graphic {
  fn bounding_box(&self) -> BoundingBox {
    match self {
      Graphic::Text(value) => value.bounding_box(),
      Graphic::TextBox(value) => value.bounding_box(),
      Graphic::Line(value) => value.bounding_box(),
      Graphic::Rectangle(value) => value.bounding_box(),
      Graphic::Circle(value) => value.bounding_box(),
      Graphic::Arc(value) => value.bounding_box(),
      Graphic::Polygon(value) => value.bounding_box(),
      Graphic::Curve(value) => value.bounding_box(),
    }
  }
}

impl TryFrom<SExpr> for Graphic {
  type Error = ParserError;
  fn try_from(value: SExpr) -> Result<Self, Self::Error> {
    let list = value.as_list()?;
    let full_name = list.peek_name()?;
    let Some(name) = full_name.split("_").nth(1) else {
      crate::error!("Valid footprint graphic type", full_name);
    };

    match name {
      "text" => Ok(Graphic::Text(list.as_sexpr_into()?)),
      "text_box" => Ok(Graphic::TextBox(list.as_sexpr_into()?)),
      "line" => Ok(Graphic::Line(list.as_sexpr_into()?)),
      "rect" => Ok(Graphic::Rectangle(list.as_sexpr_into()?)),
      "circle" => Ok(Graphic::Circle(list.as_sexpr_into()?)),
      "arc" => Ok(Graphic::Arc(list.as_sexpr_into()?)),
      "poly" => Ok(Graphic::Polygon(list.as_sexpr_into()?)),
      "curve" => Ok(Graphic::Curve(list.as_sexpr_into()?)),

      name => crate::error!("Valid footprint graphic type", name),
    }
  }
}

/// A macro that checks if the next symbol ends with a specific suffix.
/// This is because graphics are marked by type `fp_<type>` for footprint graphics,
///
macro_rules! symbol_ends_with {
  ($list: expr, $expected: literal) => {
    let found = $list.next_symbol()?.0;
    if !found.ends_with($expected) {
      return Err(ParserError::unexpected($expected, found));
    }
  };
}

/// Footprint text
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
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
  /// TODO: Implement TextEffects
  // pub effects: TextEffects,
  /// Unique identifier
  pub uuid: Uuid,
}

impl TryFrom<SExpr> for FootprintText {
  type Error = ParserError;
  fn try_from(value: SExpr) -> Result<Self, Self::Error> {
    let mut list = value.as_list()?;
    symbol_ends_with!(list, "_text");

    let mut textbox = FootprintText::default();
    while let Some(next) = list.next_maybe() {
      match next {
        SExpr::Value(value) => textbox.text = value.0,

        SExpr::Symbol(symbol) if symbol == "hide" => textbox.hide = true,
        SExpr::Symbol(symbol) if symbol == "unlocked" => textbox.hide = true,
        SExpr::Symbol(symbol) if symbol == "reference" => {
          textbox.text_type = FootprintTextType::Reference
        }
        SExpr::Symbol(symbol) if symbol == "value" => textbox.text_type = FootprintTextType::Value,
        SExpr::Symbol(symbol) if symbol == "user" => textbox.text_type = FootprintTextType::User,

        SExpr::List(attr) => match attr.peek_name()? {
          "at" => textbox.position = attr.as_sexpr_into()?,
          "layer" => textbox.layer = attr.as_sexpr_into()?,
          "uuid" => textbox.uuid = attr.as_sexpr_into()?,
          // "effects" => ???
          other => crate::catch_all!(other),
        },

        other => crate::catch_all!(other),
      }
    }

    Ok(textbox)
  }
}

impl GetBoundingBox for FootprintText {
  fn bounding_box(&self) -> BoundingBox {
    let x = self.position.x;
    let y = self.position.y;

    // TODO: Calculate width and height based on text content
    let width = 10.0; // Placeholder for text width
    let height = 5.0; // Placeholder for text height

    BoundingBox {
      min_x: x,
      min_y: y - height / 2.0,
      max_x: x + width,
      max_y: y + height / 2.0,
    }
  }
}

/// Footprint text types
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum FootprintTextType {
  #[default]
  Reference,
  Value,
  User,
}

/// Footprint text box (from version 7)
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
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
  pub points: PointList,
  /// Rotation angle
  pub angle: Option<f64>,
  /// Layer
  pub layer: Layer,
  /// Unique identifier
  pub uuid: Uuid,
  /// Text effects
  // pub effects: TextEffects,
  /// Border stroke
  pub stroke: Option<Stroke>,
}

impl TryFrom<SExpr> for FootprintTextBox {
  type Error = ParserError;
  fn try_from(value: SExpr) -> Result<Self, Self::Error> {
    let mut list = value.as_list()?;
    symbol_ends_with!(list, "_text_box");

    let mut text = Self::default();
    while let Some(next) = list.next_maybe() {
      match next {
        SExpr::Value(value) => text.text = value.0,
        SExpr::Symbol(symbol) if symbol == "locked" => text.locked = true,

        SExpr::List(mut attr) => match attr.peek_name()? {
          "start" => text.start = Some(attr.as_sexpr_into()?),
          "end" => text.end = Some(attr.as_sexpr_into()?),
          "uuid" => text.uuid = attr.as_sexpr_into()?,
          "layer" => text.layer = attr.as_sexpr_into()?,
          "angle" => text.angle = attr.discard(1)?.next_maybe_into()?,
          "stroke" => text.stroke = Some(attr.as_sexpr_into()?),
          "pts" => text.points = attr.as_sexpr_into()?,

          // "effects" => ???
          other => crate::catch_all!(other),
        },

        other => crate::catch_all!(other),
      }
    }

    Ok(text)
  }
}

impl GetBoundingBox for FootprintTextBox {
  fn bounding_box(&self) -> BoundingBox {
    let min_x = self.start.as_ref().map(|f| f.x).unwrap_or_default();
    let min_y = self.start.as_ref().map(|f| f.y).unwrap_or_default();
    let max_x = self.end.as_ref().map(|f| f.x).unwrap_or(min_x);
    let max_y = self.end.as_ref().map(|f| f.y).unwrap_or(min_y);

    BoundingBox {
      min_x,
      min_y,
      max_x,
      max_y,
    }
  }
}

/// Footprint line
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
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
  /// The width token defines the line width.
  pub width: f32,
}

impl TryFrom<SExpr> for FootprintLine {
  type Error = ParserError;
  fn try_from(value: SExpr) -> Result<Self, Self::Error> {
    let mut list = value.as_list()?;
    symbol_ends_with!(list, "_line");

    let mut line = Self::default();
    while let Some(attr) = list.next_maybe() {
      match attr {
        SExpr::Symbol(s) if s == "locked" => line.locked = false,

        SExpr::List(mut attr) => match attr.peek_name()? {
          "start" => line.start = attr.as_sexpr_into()?,
          "end" => line.end = attr.as_sexpr_into()?,

          "layer" => line.layer = attr.as_sexpr_into()?,
          "stroke" => line.stroke = attr.as_sexpr_into()?,
          "uuid" => line.uuid = attr.as_sexpr_into()?,

          "width" => line.width = attr.discard(1)?.next_into()?,

          name => crate::catch_all!(name),
        },
        name => crate::catch_all!(name),
      }
    }

    Ok(line)
  }
}

impl GetBoundingBox for FootprintLine {
  fn bounding_box(&self) -> BoundingBox {
    BoundingBox {
      min_x: self.start.x,
      min_y: self.start.y,
      max_x: self.end.x,
      max_y: self.end.y,
    }
  }
}

/// Footprint rectangle
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct FootprintRectangle {
  /// Upper left corner
  pub start: Point,
  /// Lower right corner
  pub end: Point,
  // The width token defines the line width of the rectangle. (prior to version 7)
  pub width: f32,
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

impl TryFrom<SExpr> for FootprintRectangle {
  type Error = ParserError;
  fn try_from(value: SExpr) -> Result<Self, Self::Error> {
    let mut list = value.as_list()?;
    symbol_ends_with!(list, "_rect");

    let mut rect = Self::default();
    while let Some(list) = list.next_maybe() {
      match list {
        SExpr::Symbol(s) if s == "locked" => rect.locked = false,

        SExpr::List(mut attr) => match attr.peek_name()? {
          "start" => rect.start = attr.as_sexpr_into()?,
          "end" => rect.end = attr.as_sexpr_into()?,

          "layer" => rect.layer = attr.as_sexpr_into()?,
          "stroke" => rect.stroke = attr.as_sexpr_into()?,
          "uuid" => rect.uuid = attr.as_sexpr_into()?,
          "fill" => rect.fill = attr.discard(1)?.next_symbol()? == "yes",
          "width" => rect.width = attr.discard(1)?.next_into()?,

          name => crate::catch_all!(name),
        },
        name => crate::catch_all!(name),
      }
    }

    Ok(rect)
  }
}

impl GetBoundingBox for FootprintRectangle {
  fn bounding_box(&self) -> BoundingBox {
    BoundingBox {
      min_x: self.start.x,
      min_y: self.start.y,
      max_x: self.end.x,
      max_y: self.end.y,
    }
  }
}

/// Footprint circle
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct FootprintCircle {
  /// Center point
  pub center: Point,
  /// End of radius
  pub end: Point,
  /// Layer
  pub layer: Layer,
  /// Stroke definition
  pub stroke: Stroke,
  // The width token defines the line width of the circle. (prior to version 7)
  pub width: f32,
  /// Fill flag
  pub fill: bool,
  /// Locked flag
  pub locked: bool,
  /// Unique identifier
  pub uuid: Uuid,
}

impl GetBoundingBox for FootprintCircle {
  fn bounding_box(&self) -> BoundingBox {
    let radius =
      ((self.end.x - self.center.x).powi(2) + (self.end.y - self.center.y).powi(2)).sqrt();

    BoundingBox {
      min_x: self.center.x - radius,
      min_y: self.center.y - radius,
      max_x: self.center.x + radius,
      max_y: self.center.y + radius,
    }
  }
}

impl TryFrom<SExpr> for FootprintCircle {
  type Error = ParserError;
  fn try_from(value: SExpr) -> Result<Self, Self::Error> {
    let mut list = value.as_list()?;
    symbol_ends_with!(list, "_circle");

    let mut circle = Self::default();
    while let Some(list) = list.next_maybe() {
      match list {
        SExpr::Symbol(s) if s == "locked" => circle.locked = false,

        SExpr::List(mut attr) => match attr.peek_name()? {
          "center" => circle.center = attr.as_sexpr_into()?,
          "end" => circle.end = attr.as_sexpr_into()?,

          "layer" => circle.layer = attr.as_sexpr_into()?,
          "stroke" => circle.stroke = attr.as_sexpr_into()?,
          "uuid" => circle.uuid = attr.as_sexpr_into()?,
          "fill" => circle.fill = attr.discard(1)?.next_symbol()? == "yes",
          "width" => circle.width = attr.discard(1)?.next_into()?,

          name => crate::catch_all!(name),
        },
        name => crate::catch_all!(name),
      }
    }

    Ok(circle)
  }
}

/// Footprint arc
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct FootprintArc {
  /// Start position
  pub start: Point,
  /// Midpoint along arc
  pub mid: Point,
  /// End position
  pub end: Point,
  /// Layer
  pub layer: Layer,
  /// Width of the arc (prior to version 7)
  pub width: f32,
  /// Stroke definition
  pub stroke: Stroke,
  /// Locked flag
  pub locked: bool,
  /// Unique identifier
  pub uuid: Uuid,
}

impl TryFrom<SExpr> for FootprintArc {
  type Error = ParserError;
  fn try_from(value: SExpr) -> Result<Self, Self::Error> {
    let mut list = value.as_list()?;
    symbol_ends_with!(list, "_arc");

    let mut arc = Self::default();
    while let Some(list) = list.next_maybe() {
      match list {
        SExpr::Symbol(s) if s == "locked" => arc.locked = false,

        SExpr::List(mut attr) => match attr.peek_name()? {
          "start" => arc.start = attr.as_sexpr_into()?,
          "mid" => arc.mid = attr.as_sexpr_into()?,
          "end" => arc.end = attr.as_sexpr_into()?,

          "layer" => arc.layer = attr.as_sexpr_into()?,
          "stroke" => arc.stroke = attr.as_sexpr_into()?,
          "uuid" => arc.uuid = attr.as_sexpr_into()?,
          "width" => arc.width = attr.discard(1)?.next_into()?,

          name => crate::catch_all!(name),
        },
        name => crate::catch_all!(name),
      }
    }

    Ok(arc)
  }
}

impl GetBoundingBox for FootprintArc {
  fn bounding_box(&self) -> BoundingBox {
    let min_x = self.start.x.min(self.end.x).min(self.mid.x);
    let min_y = self.start.y.min(self.end.y).min(self.mid.y);
    let max_x = self.start.x.max(self.end.x).max(self.mid.x);
    let max_y = self.start.y.max(self.end.y).max(self.mid.y);

    BoundingBox {
      min_x,
      min_y,
      max_x,
      max_y,
    }
  }
}

/// Footprint polygon
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct FootprintPolygon {
  /// Polygon outline points
  pub points: PointList,
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
  /// Width of the polygon stroke (prior to version 7)
  pub width: f32,
}

impl TryFrom<SExpr> for FootprintPolygon {
  type Error = ParserError;
  fn try_from(value: SExpr) -> Result<Self, Self::Error> {
    let mut list = value.as_list()?;
    symbol_ends_with!(list, "_poly");

    let mut poly = Self::default();
    while let Some(list) = list.next_maybe() {
      match list {
        SExpr::Symbol(s) if s == "locked" => poly.locked = false,

        SExpr::List(mut attr) => match attr.peek_name()? {
          "pts" => poly.points = attr.as_sexpr_into()?,

          "layer" => poly.layer = attr.as_sexpr_into()?,
          "stroke" => poly.stroke = attr.as_sexpr_into()?,
          "uuid" => poly.uuid = attr.as_sexpr_into()?,
          "width" => poly.width = attr.discard(1)?.next_into()?,
          "fill" => poly.fill = attr.discard(1)?.next_symbol()? == "yes",

          name => crate::catch_all!(name),
        },
        name => crate::catch_all!(name),
      }
    }

    Ok(poly)
  }
}

impl GetBoundingBox for FootprintPolygon {
  fn bounding_box(&self) -> BoundingBox {
    let mut min_x = f64::INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut max_y = f64::NEG_INFINITY;

    for point in &self.points.0 {
      match point {
        super::PointItem::Point(point) => {
          min_x = min_x.min(point.x);
          min_y = min_y.min(point.y);
          max_x = max_x.max(point.x);
          max_y = max_y.max(point.y);
        }
        super::PointItem::Arc(arc) => todo!(),
      }
    }

    BoundingBox {
      min_x,
      min_y,
      max_x,
      max_y,
    }
  }
}

/// Footprint curve (Cubic Bezier)
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct FootprintCurve {
  /// Four control points of the Bezier curve
  pub points: PointList,
  /// Layer
  pub layer: Layer,
  /// Stroke definition
  pub stroke: Stroke,
  /// Locked flag
  pub locked: bool,
  /// Width of the polygon stroke (prior to version 7)
  pub width: f32,
  /// Unique identifier
  pub uuid: Uuid,
}

impl TryFrom<SExpr> for FootprintCurve {
  type Error = ParserError;
  fn try_from(value: SExpr) -> Result<Self, Self::Error> {
    let mut list = value.as_list()?;
    symbol_ends_with!(list, "_curve");

    let mut curve = Self::default();
    while let Some(list) = list.next_maybe() {
      match list {
        SExpr::Symbol(s) if s == "locked" => curve.locked = false,

        SExpr::List(mut attr) => match attr.peek_name()? {
          "pts" => curve.points = attr.as_sexpr_into()?,

          "layer" => curve.layer = attr.as_sexpr_into()?,
          "stroke" => curve.stroke = attr.as_sexpr_into()?,
          "uuid" => curve.uuid = attr.as_sexpr_into()?,
          "width" => curve.width = attr.discard(1)?.next_into()?,

          name => crate::catch_all!(name),
        },
        name => crate::catch_all!(name),
      }
    }

    Ok(curve)
  }
}

impl GetBoundingBox for FootprintCurve {
  fn bounding_box(&self) -> BoundingBox {
    let mut min_x = f64::INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut max_y = f64::NEG_INFINITY;

    for point in &self.points.0 {
      match point {
        super::PointItem::Point(point) => {
          min_x = min_x.min(point.x);
          min_y = min_y.min(point.y);
          max_x = max_x.max(point.x);
          max_y = max_y.max(point.y);
        }
        super::PointItem::Arc(arc) => todo!(),
      }
    }

    BoundingBox {
      min_x,
      min_y,
      max_x,
      max_y,
    }
  }
}
