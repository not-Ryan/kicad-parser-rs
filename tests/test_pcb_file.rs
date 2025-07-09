const CONTENT: &str = include_str!("./board.kicad_pcb");

#[test]
pub fn sexpr() {
  use kicad_parser::sexpr::parse_sexpr;
  assert!(
    parse_sexpr(CONTENT).is_ok(),
    "Failed to parse s-expression from board.kicad_pcb"
  );
}
