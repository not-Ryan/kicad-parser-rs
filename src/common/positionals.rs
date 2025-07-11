use std::fmt::Display;

use crate::{parser::ParserError, sexpr::SExpr};

/// Position identifier defining X/Y coordinates and optional rotation angle
#[derive(Default, Debug, Clone, PartialEq)]
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
    crate::expect_eq!(list.next_symbol()?, "at", "Position::try_from");

    let x: f64 = list.next_into()?;
    let y: f64 = list.next_into()?;
    let angle: Option<f64> = list.next_maybe_into()?;
    list.expect_end()?;

    Ok(Position { x, y, angle })
  }
}

/// Coordinate point for use in point lists
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Point {
  /// X coordinate in millimeters
  pub x: f64,
  /// Y coordinate in millimeters
  pub y: f64,
}

impl TryFrom<SExpr> for Point {
  type Error = ParserError;

  fn try_from(value: SExpr) -> Result<Self, Self::Error> {
    let mut list = value.as_list()?;
    let _name = list.next_symbol()?;

    let x: f64 = list.next_into()?;
    let y: f64 = list.next_into()?;
    list.expect_end()?;

    Ok(Point { x, y })
  }
}
/// Coordinate point for use in point lists
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PointList(pub Vec<Point>);

impl TryFrom<SExpr> for PointList {
  type Error = ParserError;

  fn try_from(value: SExpr) -> Result<Self, Self::Error> {
    let mut list = value.as_list()?;
    crate::expect_eq!(list.next_symbol()?, "pts", "Point::try_from");

    let mut out = PointList::default();
    while let Some(pt) = list.next_maybe_list()? {
      crate::expect_eq!(pt.peek_name()?, "xy", "PointList::try_from");
      let pt: Point = pt.as_sexpr_into()?;
      out.0.push(pt);
    }

    Ok(out)
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BoundingBox {
  pub min_x: f64,
  pub min_y: f64,
  pub max_x: f64,
  pub max_y: f64,
}

pub trait GetBoundingBox {
  fn bounding_box(&self) -> BoundingBox;
}

impl Default for BoundingBox {
  fn default() -> Self {
    BoundingBox {
      min_x: f64::MAX,
      min_y: f64::MAX,
      max_x: f64::MIN,
      max_y: f64::MIN,
    }
  }
}

impl BoundingBox {
  pub fn x(&self) -> f64 {
    self.min_x
  }
  pub fn y(&self) -> f64 {
    self.min_y
  }
  pub fn width(&self) -> f64 {
    self.max_x - self.min_x
  }

  pub fn height(&self) -> f64 {
    self.max_y - self.min_y
  }

  pub fn envelop(&mut self, other: &Self) {
    self.min_x = self.min_x.min(other.min_x);
    self.min_y = self.min_y.min(other.min_y);
    self.max_x = self.max_x.max(other.max_x);
    self.max_y = self.max_y.max(other.max_y);
  }

  pub fn move_by(&mut self, dx: f64, dy: f64) {
    self.min_x += dx;
    self.min_y += dy;
    self.max_x += dx;
    self.max_y += dy;
  }
}

impl Display for BoundingBox {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let x = self.x();
    let y = self.y();
    let w = self.width();
    let h = self.height();
    write!(f, "BoundingBox at {x}:{y} {w}mm {h}mm")
  }
}
