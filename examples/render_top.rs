use kicad_parser::common::{GetBoundingBox, Layer};
use std::path::PathBuf;

use clap::Parser;

use svg::node::element::Path;
use svg::node::element::path::Data;
use svg::{Document, Node};

/// Program attempts to render the top layer to svg
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
  /// Pcbnew (.kicad_pcb) file to parse
  #[arg(short, long)]
  pcb_file: PathBuf,
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

  for footprint in pcb.footprints {
    let anchor = footprint.position.expect("Footprint without position?");

    for pad in footprint.pads.iter() {
      let pad_pos = anchor.transform_position(&pad.position);

      if pad.layers.contains(&Layer("F.Cu".to_string())) {
        let rel_start = pad_pos.transform_angle(pad.size);
        let pad_bounds = Data::new()
          .move_to((pad_pos.x - rel_start.x / 2., pad_pos.y - rel_start.y / 2.))
          .line_by(pad_pos.transform_angle((0., pad.size.1)).as_tuple())
          .line_by(pad_pos.transform_angle((pad.size.0, 0.)).as_tuple())
          .line_by(pad_pos.transform_angle((0., -pad.size.1)).as_tuple())
          .close();

        let path = Path::new()
          .set("fill", "none")
          .set("stroke", "black")
          .set("stroke-width", 0.1)
          .set("d", pad_bounds);

        doc.append(path);
      }
    }
  }
  svg::save(args.pcb_file.with_extension("svg"), &doc).unwrap();
}
