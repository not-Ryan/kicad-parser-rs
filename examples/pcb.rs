use std::fs;

const CONTENT: &str = include_str!("./board.kicad_pcb");

pub fn main() {
  use kicad_parser::pcb::parse_pcb_file;
  parse_pcb_file(CONTENT).unwrap();
  assert!(
    parse_pcb_file(CONTENT).is_ok(),
    "Failed to parse s-expression from board.kicad_pcb"
  );

  fs::write(
    "./pcb_file.txt",
    format!("{:?}", parse_pcb_file(CONTENT).unwrap()),
  )
  .unwrap();
}
