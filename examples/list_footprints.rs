use kicad_parser::common::GetBoundingBox;
use std::path::PathBuf;

use clap::Parser;

/// Program lists components whose values are not "Fiducial"
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

  let content = std::fs::read(args.pcb_file).unwrap();
  let content = std::str::from_utf8(&content).expect("File doesn't contain valid utf-8");
  let pcb = parse_pcb_file(content).unwrap();
  let bounding = pcb.bounding_box();

  let w = bounding.width();
  let h = bounding.height();
  println!("PCB Bounding Box: {w}x{h}");

  for footprint in pcb.footprints {
    if footprint.properties.get("Value") == Some(&"Fiducial".to_string()) {
      continue;
    }

    let bounding = footprint.bounding_box();
    let position = footprint.position.unwrap();
    let name = footprint.properties.get("Value");
    println!("Footprint bounding: {bounding} {position:?} {name:?}");
  }
}
