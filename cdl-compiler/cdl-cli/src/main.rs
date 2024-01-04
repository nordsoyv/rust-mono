use cdl_parser::parse_text;

fn main() {
  let file = include_str!("../../test_script/test.cdl");
  println!("Starting compile");

  let mut total_nodes = 0;
  for i in 0..100 {
    let ast = parse_text(file).unwrap();
    total_nodes += ast.nodes.len();
  }
  println!("Done");
  println!("num nodes: {}", total_nodes);
}
