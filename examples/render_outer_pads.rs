use kicad_parser::common::{BoundingBox, GetBoundingBox, Layer, PadType};
use std::path::PathBuf;

use clap::Parser;

use svg::node::element::path::Data;
use svg::node::element::{Circle, Group, Path, Rectangle};
use svg::{Document, Node};

/// Program attempts to render the top layer to svg
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
  /// Pcbnew (.kicad_pcb) file to parse
  #[arg(short, long)]
  pcb_file: PathBuf,

  /// Layer to render, e.g. "F.Cu" | "B.Cu"
  #[arg(short, long)]
  layer: String,
}

pub fn main() {
  use kicad_parser::pcb_file::parse_pcb_file;
  let args = Args::parse();

  let content = std::fs::read(args.pcb_file.clone()).unwrap();
  let content = std::str::from_utf8(&content).expect("File doesn't contain valid utf-8");
  let pcb = parse_pcb_file(content).unwrap();

  let bounding = pcb.bounding_box();

  let w = bounding.width();
  let h = bounding.height();
  println!("PCB Bounding Box: {w}x{h}");

  let mut doc = Document::new().set(
    "viewBox",
    (
      bounding.min_x as i32,
      bounding.min_y as i32,
      bounding.max_x as i32,
      bounding.max_y as i32,
    ),
  );

  let board_center_x = (bounding.max_x + bounding.min_x) / 2.;

  // NOTE: this is the bottom as viewed when canonical view is flipped
  let layer = args.layer.as_str().into();
  for footprint in pcb.footprints.iter() {
    let anchor = footprint
      .position
      .clone()
      .expect("Footprint without position?")
      .mirror_around_x(board_center_x);

    let mut group = Group::new();
    let mut bb = BoundingBox::default();

    for pad in footprint.pads.iter() {
      if pad.layers.contains(&layer) {
        println!("ref {:?} {:?}", pad.position, anchor);
        let pad_bb = pad
          .translate(&anchor, footprint.layer.is_back())
          .bounding_box();
        bb.envelop(&pad_bb);

        // Reference impl for rects (Top only)
        // *pad.position.angle.get_or_insert(0.) += anchor.angle.unwrap_or(0.);
        // let mut pad_pos = anchor.transform_position(&pad.position);
        // pad_pos.angle = pad_pos.angle.map(|v| -v);
        // let rel_start = pad_pos.transform_angle(pad.size);
        // let pad_bounds = Data::new()
        //   .move_to((pad_pos.x - rel_start.x / 2., pad_pos.y - rel_start.y / 2.))
        //   .line_by(pad_pos.transform_angle((0., pad.size.1)).as_tuple())
        //   .line_by(pad_pos.transform_angle((pad.size.0, 0.)).as_tuple())
        //   .line_by(pad_pos.transform_angle((0., -pad.size.1)).as_tuple())
        //   .close();

        // if pad.pad_type == PadType::Smd {
        //   bb.envelop(&pad_bb);
        // }

        // let path = Path::new()
        //   .set("fill", "none")
        //   .set(
        //     "stroke",
        //     format!("oklch(0.3 0.7 {:.1}deg)", pad_pos.angle.unwrap_or(0.)),
        //   )
        //   .set("stroke-width", 0.1)
        //   .set("d", pad_bounds);

        // group.append(path);

        group.append(
          Rectangle::new()
            .set("width", pad_bb.width())
            .set("height", pad_bb.height())
            .set("x", pad_bb.min_x)
            .set("y", pad_bb.min_y)
            .set(
              "stroke",
              format!(
                "oklch(0.3 0.7 {:.1}deg)",
                footprint
                  .position
                  .clone()
                  .and_then(|v| v.angle)
                  .unwrap_or(0.)
              ),
            )
            .set("style", "stroke-width: 0.1; fill: none"),
        );
      }
    }
    if bb != BoundingBox::default() {
      let center = bb.center();
      group.append(
        Circle::new()
          .set("cx", center.0)
          .set("cy", center.1)
          .set("r", 0.3),
      );
      group.append(
        Circle::new()
          .set("cx", anchor.x)
          .set("cy", anchor.y)
          .set("r", 0.3)
          .set("fill", "red"),
      );
    }

    if group.get_children().is_some_and(|v| !v.is_empty()) {
      doc.append(group);
    }
  }

  let out_path = args.pcb_file.with_extension("svg");
  svg::save(&out_path, &doc).unwrap();
  println!("See {:?} for result", out_path);
}
