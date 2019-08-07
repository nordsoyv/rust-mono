use crate::parser::{Node, Parser, parser_to_ast, Ast};
use crate::parser::ast_nodes::{NodeRef, Operator};
use crate::lexer::Lexer;

pub fn print_ast(ast: &Ast) -> String {
  let start = ast.script_entity;
  let script = &ast.nodes[start];
  match script {
    Node::Entity(n) => {
      let children: Vec<String> = (&n.children).into_iter().map(|c| print_node(ast, *c, 0)).collect();
      return format!("{}", children.join("\n"));
    }
    _ => { return format!("did not get entity as start node"); }
  }
}

fn print_node(ast: &Ast, node_ref: NodeRef, indent: u32) -> String {
  let n = &ast.nodes[node_ref];
  let padding = pad(indent);

  match n {
    Node::Title(node) => {
      return format!("title \"{}\"\n", node.title);
    }
    Node::Color(node) => {
      return format!("#{}", node.value);
    }
    Node::Number(node) => {
      return format!("{}", node.value);
    }
    Node::Entity(node) => {
      let mut header = String::new();
      header.push_str(&padding);
      header.push_str(&node.terms.join(" "));
      if node.refs.len()>0 {
        let ref_str :Vec<String> = (&node.refs).into_iter().map(|r| format!("@{}",r)).collect();
        header.push_str(" ");
        header.push_str(&ref_str.join(" "));
      }
      if node.identifier.len() > 0 {
        header.push_str(" #");
        header.push_str(&node.identifier);
      }
      let children: Vec<String> = (&node.children).into_iter().map(|c| print_node(ast, *c, indent + 2)).collect();
      return format!("{} {{\n{}\n{}}}\n", header, children.join("\n"), padding);
    }
    Node::Property(node) => {
      return format!("{}{}: {}", padding, node.name, print_node(ast, node.rhs, 0));
    }
    Node::String(node) => {
      return format!("\"{}\"", node.value);
    }
    Node::FunctionCall(node) => {
      let mut args = "".to_string();
      if let Some(arg_refs) = node.args {
        args.push_str(&print_node(ast, arg_refs, indent));
      }
      return format!("{}({})", node.name, args);
    }
    Node::List(node) => {
      let children: Vec<String> = (&node.items).into_iter().map(|c| print_node(ast, *c, indent + 2)).collect();
      return format!("{}", children.join(", "));
    }
    Node::Identifier(node) => {
      return format!("{}", node.value);
    }
    Node::Operator(node) => {
      return format!("{} {} {}", print_node(ast, node.left, indent), op_to_str(&node.op), print_node(ast, node.right, indent));
    }
    Node::UnaryOp(node) => {
      return format!("{}{}", op_to_str(&node.op), print_node(ast, node.right, indent));
    }
    Node::Reference(node) => {
      return format!("@{}", node.value);
    }
    Node::TableDecl(node) => {
      return format!("{}table {} = {}", padding, node.name, node.path);
    }
    Node::VPath(node) => {
      return format!("{}:{}", node.source, node.question);
    }
  }
}

fn op_to_str(op: &Operator) -> &str {
  match op {
    Operator::And => "AND",
    Operator::Del => "/",
    Operator::Equal => "=",
    Operator::LessThan => "<",
    Operator::LessThanOrEqual => "<=",
    Operator::Minus => "-",
    Operator::MoreThan => ">",
    Operator::MoreThanOrEqual => ">=",
    Operator::Mul => "*",
    Operator::Or => "OR",
    Operator::Plus => "+",
  }
}

fn pad(indent: u32) -> String {
  let mut s: String = "".to_string();
  for _ in 0..indent {
    s.push(' ');
  }
  return s;
}

#[test]
fn print_test() {
  let script = "title \"my title\"

config hub {
  hub: 12
  table accounts = crmdata.ArtuAccountHierarchy
  color: #ff00ff
  string: \"hello world\"
  func: foo()
  func2: foo(ident, ident2)
  currentPeriodB2b: survey:interview_start
}

page {
  widget kpi @default @other #widgetId {
    label: \"Label\"
    func: 1 + 2
    func: 1 AND 2
    func: -1
    ref: @widget.kpi
  }

}
".to_string();
  let l = Lexer::new();
  let tokens = l.lex(script.clone()).unwrap();
  let mut p = Parser::new();
  p.parse(tokens).unwrap();
  let ast = parser_to_ast(p);
  let res = print_ast(&ast);
  assert_eq!(res, script);
}