use std::{fs, rc::Rc, time::Instant};

use ast::{Ast, Node, NodeRef};
use clap::Parser;
use node_processing::NodeProcessor;
use parser::parse_text;
use simple_logger::SimpleLogger;

#[derive(Parser)]
#[command(arg_required_else_help = true)]
struct Cli {
  file: Option<String>,
}

fn compare_rc_str_to_filters(needle: &Rc<str>, filters: &Vec<&str>) -> bool {
  let n: String = needle.to_string();
  for filter in filters {
    if &n == filter {
      return true;
    }
  }
  false
}

fn find_filters(ast: &Ast, filters: &Vec<&str>) -> Vec<NodeRef> {
  let mut result = vec![];
  for (index, node) in ast.nodes.borrow().iter().enumerate() {
    let n = node.borrow();
    match &n.node_data {
      Node::Entity(ent) => {
        if let Some(id) = &ent.ident {
          if compare_rc_str_to_filters(id, filters) {
            result.push(NodeRef(index as isize));
          }
        }
      }
      _ => continue,
    }
  }

  result
}

fn main() {
  SimpleLogger::new().init().unwrap();
  let cli = Cli::parse();
  if let Some(name) = cli.file.as_deref() {
    println!("File: {name}");
    let file_content = fs::read_to_string(name).expect("should be able to read file");

    let now = Instant::now();
    let mut total_nodes = 0;

    let ast = parse_text(&file_content).unwrap();
    let elapsed = now.elapsed();
    total_nodes += ast.nodes.borrow().len();
    println!("Done");
    println!("num nodes: {}", total_nodes);
    println!("time taken: {:.2?}", elapsed);

    let now = Instant::now();
    let clone = ast.clone();
    println!("num nodes: {}", clone.nodes.borrow().len());
    let elapsed = now.elapsed();
    println!("time taken for cloning: {:.2?}", elapsed);

    let now = Instant::now();
    let _json = serde_json::to_string(&ast).unwrap();
    let elapsed = now.elapsed();
    println!("time taken for json sericalizing: {:.2?}", elapsed);

    let filters = vec![
      "fromQuestionFilter_NP_LOB",
      "fromQuestionFilter_NP_PLAN",
      "fromQuestionFilter_NP_CONTRACT",
      "fromQuestionFilter_NP_GENDER",
      "fromQuestionFilter_NP_RACE",
      "fromQuestionFilter_NP_MEMST",
      "fromQuestionFilter_SA_LOB",
      "fromQuestionFilter_SA_PLAN",
      "fromQuestionFilter_SA_CONTRACT",
      "fromQuestionFilter_SA_GENDER",
      "fromQuestionFilter_SA_RACE",
      "fromQuestionFilter_SA_MEMST",
      "fromQuestionFilter_SA_OA1",
      "fromQuestionFilter_AC_LOB",
      "fromQuestionFilter_AC_PLAN",
      "fromQuestionFilter_AC_CONTRACT",
      "fromQuestionFilter_AC_GENDER",
      "fromQuestionFilter_AC_RACE",
      "fromQuestionFilter_AC_MEMST",
      "fromQuestionFilter_AC_OA1",
      "fromQuestionFilter_AC_MA1",
      "fromQuestionFilter_RXCombo_LOB",
      "fromQuestionFilter_RXCombo_PLAN",
      "fromQuestionFilter_RXCombo_CONTRACT",
      "fromQuestionFilter_RXCombo_GENDER",
      "fromQuestionFilter_RXCombo_RACE",
      "fromQuestionFilter_RXCombo_MEMST",
      "fromQuestionFilter_RXCombo_OA1",
      "fromQuestionFilter_RP_LOB",
      "fromQuestionFilter_RP_PLAN",
      "fromQuestionFilter_RP_CONTRACT",
      "fromQuestionFilter_RP_GENDER",
      "fromQuestionFilter_RP_RACE",
      "fromQuestionFilter_RP_MEMST",
      "fromQuestionFilter_RP_OA1",
      "fromQuestionFilter_RP_PA4",
      "fromQuestionFilter_GP_LOB",
      "fromQuestionFilter_GP_PLAN",
      "fromQuestionFilter_GP_CONTRACT",
      "fromQuestionFilter_GP_GENDER",
      "fromQuestionFilter_GP_RACE",
      "fromQuestionFilter_GP_MEMST",
      "fromQuestionFilter_GP_OA1",
      "fromQuestionFilter_CX_LOB",
      "fromQuestionFilter_CX_PLAN",
      "fromQuestionFilter_CX_CONTRACT",
      "fromQuestionFilter_CX_GENDER",
      "fromQuestionFilter_CX_RACE",
      "fromQuestionFilter_CX_MEMST",
      "fromQuestionFilter_CX_OA1",
    ];
    let now = Instant::now();
    let found = find_filters(&ast, &filters);

    let elapsed = now.elapsed();
    println!("time taken to find {} nodes: {:.2?}", found.len(), elapsed);

    let now = Instant::now();
    let np = NodeProcessor::new(ast);
    let _processed_ast = np.process().unwrap();

    let elapsed = now.elapsed();
    println!("time taken to process ast: {:.2?}", elapsed);
  }
}
