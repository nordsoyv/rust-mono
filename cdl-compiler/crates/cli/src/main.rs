use std::{
  env,
  fs::{self, File},
  io::{BufReader, BufWriter},
  path::{Path, PathBuf},
  time::Instant,
};

use ast::{Ast, Node, NodeRef};
use clap::Parser;
use lexer::LexedStr;
use node_processing::NodeProcessor;
use parser::parse_text;
use tempfile::TempDir;
use tracing::{info, Level};
use tracing_flame::FlameLayer;
use tracing_subscriber::{prelude::*, registry::Registry};

#[derive(Parser)]
#[command(arg_required_else_help = true)]
struct Cli {
  #[arg(short, long)]
  file: Option<String>,

  #[arg(short, long, default_value_t = false)]
  graph: bool,
}

fn compare_rc_str_to_filters(needle: &LexedStr, filters: &Vec<&str>) -> bool {
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
    
    match &node.node_data {
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

static PATH: &str = "flame.folded";

fn setup_global_subscriber(dir: &Path) -> impl Drop {
  let (flame_layer, _guard) = FlameLayer::with_file(dir.join(PATH)).unwrap();

  let subscriber = Registry::default().with(flame_layer);

  tracing::subscriber::set_global_default(subscriber).unwrap();

  _guard
}

fn make_flamegraph(tmpdir: &Path, out: &Path) {
  println!("outputting flamegraph to {}", out.display());
  let inf = File::open(tmpdir.join(PATH)).unwrap();
  let reader = BufReader::new(inf);

  let out = File::create(out).unwrap();
  let writer = BufWriter::new(out);

  let mut opts = inferno::flamegraph::Options::default();
  inferno::flamegraph::from_reader(&mut opts, reader, writer).unwrap();
}
struct DropDummy;
impl Drop for DropDummy {
  fn drop(&mut self) {}
}
fn main() {
  //tracing_subscriber::fmt::init();
  let cli = Cli::parse();

  let (guard, out, tmp_dir) = if cli.graph {
    let out = {
      let mut path = env::current_dir().unwrap();
      path.push("tracing-flame-inferno.svg");
      path
    };
    let tmp_dir = tempfile::Builder::new()
      .prefix("flamegraphs")
      .tempdir()
      .expect("failed to create temporary directory");
    (Some(setup_global_subscriber(tmp_dir.path())), out, tmp_dir)
  } else {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();
    (None, PathBuf::new(), TempDir::new().unwrap())
  };

  info!("Start compiling");
  if let Some(name) = cli.file.as_deref() {
    info!("File: {name}");
    let file_content = fs::read_to_string(name).expect("should be able to read file");

    let now = Instant::now();
    let mut total_nodes = 0;

    let ast = parse_text(&file_content).unwrap();
    let elapsed = now.elapsed();
    total_nodes += ast.nodes.borrow().len();
    info!("Done");
    info!("num nodes: {}", total_nodes);
    info!("time taken: {:.2?}", elapsed);

    let now = Instant::now();
    let clone = ast.clone();
    info!("num nodes: {}", clone.nodes.borrow().len());
    let elapsed = now.elapsed();
    info!("time taken for cloning: {:.2?}", elapsed);

    let now = Instant::now();
    let _json = serde_json::to_string(&ast).unwrap();
    let elapsed = now.elapsed();
    info!("time taken for json sericalizing: {:.2?}", elapsed);

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
    info!("time taken to find {} nodes: {:.2?}", found.len(), elapsed);

    let now = Instant::now();
    let np = NodeProcessor::new(ast);
    let _processed_ast = np.process().unwrap();

    let elapsed = now.elapsed();
    info!("time taken to process ast: {:.2?}", elapsed);
  }
  if cli.graph {
    drop(guard.unwrap());
    make_flamegraph(tmp_dir.path(), out.as_ref());
  }
}
