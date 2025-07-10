use kicad_parser::common::GetBoundingBox;

const CONTENT: &str = include_str!("./board.kicad_pcb");

pub fn main() {
  use kicad_parser::pcb_file::parse_pcb_file;
  let pcb = parse_pcb_file(CONTENT).unwrap();
  let bounding = pcb.bounding_box();

  let w = bounding.width();
  let h = bounding.height();
  println!("PCB Bounding Box: {w}x{h}");

  for footprint in pcb.footprints {
    if footprint.properties.get("Value") != Some(&"Fiducial".to_string()) {
      continue;
    }

    let bounding = footprint.bounding_box();
    println!("Footprint bounding: {bounding}");
  }
}
