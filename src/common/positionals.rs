use std::{
  fmt::Display,
  ops::{Add, AddAssign, Div},
};

use crate::{parser::ParserError, sexpr::SExpr};

/// Position identifier defining X/Y coordinates and optional rotation angle
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Position {
  /// X coordinate in millimeters
  pub x: f64,
  /// Y coordinate in millimeters  
  pub y: f64,
  /// Optional rotation angle in degrees
  pub angle: Option<f64>,
}

impl Position {
  pub fn mirror_around_x(&self, x: f64) -> Position {
    Self {
      x: self.x + (x - self.x) * 2.,
      y: self.y,
      angle: self.angle,
    }
  }

  pub fn transform_position(&self, sub_pos: &Position) -> Position {
    if let Some(angle) = self.angle {
      let angle = angle.to_radians();
      Position {
        x: self.x + (sub_pos.x * angle.cos() + sub_pos.y * angle.sin()),
        y: self.y + (-sub_pos.x * angle.sin() + sub_pos.y * angle.cos()),
        angle: sub_pos.angle,
      }
    } else {
      Position {
        x: self.x + sub_pos.x,
        y: self.y + sub_pos.y,
        angle: sub_pos.angle,
      }
    }
  }

  pub fn transform_angle(&self, point: impl Into<Point>) -> Point {
    let point = point.into();
    if let Some(angle) = self.angle {
      let angle = angle.to_radians();
      Point {
        x: point.x * angle.cos() + point.y * angle.sin(),
        y: point.x * angle.sin() - point.y * angle.cos(),
      }
    } else {
      point
    }
  }

  pub fn transform_point(&self, point: impl Into<Point>) -> Point {
    let point = point.into();
    if let Some(angle) = self.angle {
      let angle = angle.to_radians();
      Point {
        x: self.x + (point.x * angle.cos() + point.y * angle.sin()),
        y: self.y + (-point.x * angle.sin() + point.y * angle.cos()),
      }
    } else {
      Point {
        x: self.x + point.x,
        y: self.y + point.y,
      }
    }
  }
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

impl From<(f64, f64)> for Point {
  fn from(value: (f64, f64)) -> Self {
    Point {
      x: value.0,
      y: value.1,
    }
  }
}

/// Coordinate point for use in point lists
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Point {
  /// X coordinate in millimeters
  pub x: f64,
  /// Y coordinate in millimeters
  pub y: f64,
}

fn rotate_point(x: f64, y: f64, angle: f64) -> (f64, f64) {
  let angle = angle.to_radians();
  let cos_a = angle.cos();
  let sin_a = angle.sin();
  (x * cos_a + y * sin_a, x * sin_a - y * cos_a)
}

impl Point {
  pub fn as_tuple(&self) -> (f64, f64) {
    (self.x, self.y)
  }

  /// Returns a new `Point` rotated counter-clockwise around the origin by `angle` (radians).
  pub fn rotate(&self, angle: f64) -> Point {
    let (rx, ry) = rotate_point(self.x, self.y, angle);
    Point { x: rx, y: ry }
  }

  pub fn new(x: f64, y: f64) -> Self {
    Point { x, y }
  }
}

impl Add for Point {
  type Output = Point;

  fn add(self, rhs: Self) -> Self::Output {
    Point::new(self.x + rhs.x, self.y + rhs.y)
  }
}

impl AddAssign for Point {
  fn add_assign(&mut self, rhs: Self) {
    self.x += rhs.x;
    self.y += rhs.y;
  }
}

impl Div<f64> for Point {
  type Output = Point;

  fn div(self, rhs: f64) -> Self::Output {
    Point::new(self.x / rhs, self.y / rhs)
  }
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

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum PointItem {
  Point(Point),
  Arc(Arc),
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Arc {
  pub start: Point,
  pub mid: Point,
  pub end: Point,
}

impl Arc {
  /// Returns the axis-aligned bounding box of the arc's centerline (no stroke width).
  pub fn bounding_box_centerline(&self) -> BoundingBox {
    let (cx, cy, r) = circle_from_three_points(&self.start, &self.mid, &self.end);
    let start_angle = normalize_angle((self.start.y - cy).atan2(self.start.x - cx));
    let mid_angle = normalize_angle((self.mid.y - cy).atan2(self.mid.x - cx));
    let end_angle = normalize_angle((self.end.y - cy).atan2(self.end.x - cx));

    let (angle_start, angle_end) = get_arc_interval(start_angle, end_angle, mid_angle);

    let mut bbox = BoundingBox::default();
    bbox.add_point(&self.start);
    bbox.add_point(&self.end);

    // Check the four cardinal extremes of a circle
    let cardinals = [
      0.0,
      std::f64::consts::FRAC_PI_2,
      std::f64::consts::PI,
      3.0 * std::f64::consts::FRAC_PI_2,
    ];
    for &angle in &cardinals {
      if angle_in_interval(angle, angle_start, angle_end) {
        let px = cx + r * angle.cos();
        let py = cy + r * angle.sin();
        bbox.add_point(&Point { x: px, y: py });
      }
    }

    bbox
  }

  /// Generates `num_points` evenly spaced points along the arc.
  pub fn sample_points(&self, num_points: usize) -> Vec<Point> {
    let (cx, cy, r) = circle_from_three_points(&self.start, &self.mid, &self.end);
    let start_angle = normalize_angle((self.start.y - cy).atan2(self.start.x - cx));
    let mid_angle = normalize_angle((self.mid.y - cy).atan2(self.mid.x - cx));
    let end_angle = normalize_angle((self.end.y - cy).atan2(self.end.x - cx));
    let (angle_start, angle_end) = get_arc_interval(start_angle, end_angle, mid_angle);

    let n = num_points.max(2);
    (0..n)
      .map(|i| {
        let t = angle_start + (i as f64) * (angle_end - angle_start) / ((n - 1) as f64);
        Point {
          x: cx + r * t.cos(),
          y: cy + r * t.sin(),
        }
      })
      .collect()
  }
}

// ---------- Helper functions ----------

/// Normalize an angle (in radians) into the range [0, 2π).
fn normalize_angle(angle: f64) -> f64 {
  let two_pi = 2.0 * std::f64::consts::PI;
  let a = angle % two_pi;
  if a < 0.0 { a + two_pi } else { a }
}

/// Determine the continuous CCW interval [angle_start, angle_end] that represents the
/// smaller arc going from `start_angle` to `end_angle` through `mid_angle`.
///
/// All input angles are in [0, 2π). The returned `angle_end` may be larger than 2π if
/// the interval wraps around.
/// Given the start, end, and mid angles (all normalized to [0, 2π)),
/// returns the continuous CCW angular interval [a, b] that represents
/// the actual arc. The returned `b` may exceed 2π if the interval wraps.
/// Returns the continuous CCW angular interval [a, b] that represents the arc.
/// `b` may be > 2π if the interval wraps.
fn get_arc_interval(start_angle: f64, end_angle: f64, mid_angle: f64) -> (f64, f64) {
  let two_pi = 2.0 * std::f64::consts::PI;

  // Degenerate case: start and end coincide → treat as full circle
  if (start_angle - end_angle).abs() < 1e-12 {
    return (0.0, two_pi);
  }

  // CCW distance from start to end
  let ccw_dist = if end_angle >= start_angle {
    end_angle - start_angle
  } else {
    end_angle + two_pi - start_angle
  };

  // Shift mid so it lies in the same continuous range as [start, start+ccw_dist)
  let mid_shifted = if mid_angle >= start_angle {
    mid_angle
  } else {
    mid_angle + two_pi
  };

  // Does mid lie on the CCW path from start to end?
  let mid_on_ccw = mid_shifted < start_angle + ccw_dist;

  if mid_on_ccw {
    // Arc is the minor (or at least the CCW) arc from start to end
    (start_angle, start_angle + ccw_dist)
  } else {
    // Arc is the major (CW) arc → equivalent to CCW from end to start + 2π
    (end_angle, start_angle + two_pi)
  }
}

/// Returns true if `angle` (in [0, 2π)) lies within the interval `[angle_start, angle_end]`,
/// where `angle_start < angle_end` and `angle_end` may exceed 2π if the interval wraps.
fn angle_in_interval(angle: f64, angle_start: f64, angle_end: f64) -> bool {
  let two_pi = 2.0 * std::f64::consts::PI;
  if angle_end > two_pi {
    angle >= angle_start || angle <= angle_end - two_pi
  } else {
    angle >= angle_start && angle <= angle_end
  }
}

/// Computes the center `(cx, cy)` and radius `r` of the unique circle passing through
/// three non‑collinear points.
///
/// Uses the determinant formula for the circumcenter. Returns `(0.0, 0.0, 0.0)` if the
/// points are collinear (which should never happen for a valid arc).
fn circle_from_three_points(p1: &Point, p2: &Point, p3: &Point) -> (f64, f64, f64) {
  let d = 2.0 * (p1.x * (p2.y - p3.y) + p2.x * (p3.y - p1.y) + p3.x * (p1.y - p2.y));
  if d.abs() < 1e-12 {
    // Points are collinear – fall back gracefully (theoretical arc cannot exist)
    return (0.0, 0.0, 0.0);
  }

  let p1_sq = p1.x.powi(2) + p1.y.powi(2);
  let p2_sq = p2.x.powi(2) + p2.y.powi(2);
  let p3_sq = p3.x.powi(2) + p3.y.powi(2);

  let cx = (p1_sq * (p2.y - p3.y) + p2_sq * (p3.y - p1.y) + p3_sq * (p1.y - p2.y)) / d;
  let cy = (p1_sq * (p3.x - p2.x) + p2_sq * (p1.x - p3.x) + p3_sq * (p2.x - p1.x)) / d;
  let r = ((p1.x - cx).powi(2) + (p1.y - cy).powi(2)).sqrt();

  (cx, cy, r)
}

/// Coordinate point for use in point lists
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct PointList(pub Vec<PointItem>);

impl TryFrom<SExpr> for PointList {
  type Error = ParserError;

  fn try_from(value: SExpr) -> Result<Self, Self::Error> {
    let mut list = value.as_list()?;
    crate::expect_eq!(list.next_symbol()?, "pts", "Point::try_from");

    let mut out = PointList::default();
    while let Some(mut pt) = list.next_maybe_list()? {
      match pt.peek_name()? {
        "xy" => {
          let pt: Point = pt.as_sexpr_into()?;
          out.0.push(PointItem::Point(pt));
        }
        "arc" => {
          pt.next_symbol()?; // arc
          let mut list = pt.next_list()?;
          let mut arc = Arc::default();
          while let Some(list) = list.next_maybe() {
            match list {
              SExpr::List(attr) => match attr.peek_name()? {
                "start" => arc.start = attr.as_sexpr_into()?,
                "mid" => arc.mid = attr.as_sexpr_into()?,
                "end" => arc.end = attr.as_sexpr_into()?,

                name => crate::catch_all!(name),
              },
              name => crate::catch_all!(name),
            }
          }
          out.0.push(PointItem::Arc(arc));
        }
        name => crate::catch_all!(name),
      }
    }

    Ok(out)
  }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
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
  // NOTE: this does not work for board flipping as that is done the anchor rotation
  pub fn mirror_x_around(&self, center_x: f64) -> Self {
    let min_dist = center_x - self.max_x;
    let max_dist = center_x - self.min_x;
    BoundingBox {
      min_x: center_x + max_dist.min(min_dist),
      min_y: self.min_y,
      max_x: center_x + min_dist.max(max_dist),
      max_y: self.max_y,
    }
  }

  pub fn translate(&self, pos: Position) -> Self {
    // TODO: this should probably return an angle bounding box
    let corners = [
      Point::new(self.min_x, self.min_y),
      Point::new(self.min_x, self.max_y),
      Point::new(self.max_x, self.min_y),
      Point::new(self.max_x, self.max_y),
    ];

    let mut bbox = BoundingBox::default();
    let corners = corners.map(|v| pos.transform_point(v));
    for p in corners.iter() {
      bbox.add_point(p);
    }

    bbox
  }

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

  pub fn center(&self) -> (f64, f64) {
    (
      (self.min_x + self.max_x) / 2.,
      (self.min_y + self.max_y) / 2.,
    )
  }

  pub fn add_point(&mut self, point: &Point) {
    if point.x < self.min_x {
      self.min_x = point.x;
    }
    if point.y < self.min_y {
      self.min_y = point.y;
    }
    if point.x > self.max_x {
      self.max_x = point.x;
    }
    if point.y > self.max_y {
      self.max_y = point.y;
    }
  }

  pub fn from_points(points: &[Point]) -> Self {
    let mut result = BoundingBox::default();
    for p in points {
      result.add_point(p);
    }
    result
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
