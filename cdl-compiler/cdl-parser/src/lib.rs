mod ast_nodes;
mod parser;
mod types;

use anyhow::Result;
use cdl_lexer::lex;
use parser::{Node, Parser};
use std::cell::RefCell;
use types::NodeRef;

pub fn parse_text(text: &str) -> Result<Ast> {
  let tokens = lex(text)?;
  let mut parser = Parser {
    curr_token: RefCell::new(0),
    nodes: RefCell::new(Vec::new()),
    tokens: tokens,
  };
  let root_ref = parser.parse()?;

  Ok(Ast {
    nodes: parser.nodes.take(),
    script_entity: root_ref,
  })
}

#[derive(Debug)]
pub struct Ast {
  pub nodes: Vec<Node>,
  pub script_entity: NodeRef,
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn can_parse_title() {
    let ast = parse_text("title \"dashboard title\"\n");
    assert!(ast.is_ok());
  }
  #[test]
  fn can_parse_entity() {
    let ast = parse_text(
      r"maintype {

    }   
    ",
    );
    assert!(ast.is_ok());
    let ast = ast.unwrap();
    if let Node::Entity(node) = &ast.nodes[1] {
      assert_eq!("maintype", node.terms[0].to_string());
      assert_eq!(0, node.children.len());
    }
  }

  #[test]
  fn can_parse_property_identifier() {
    let ast = parse_text(
      r#"maintype {
        prop: identifier
    }   
    "#,
    );
    assert!(ast.is_ok());
    let ast = ast.unwrap();
    if let Node::Identifier(node) = &ast.nodes[3] {
      assert_eq!("identifier", node.identifier.to_string());
    }
  }

  #[test]
  fn can_parse_property_string() {
    let ast = parse_text(
      r#"maintype {
        prop: "string"
    }   
    "#,
    );
    assert!(ast.is_ok());
    let ast = ast.unwrap();
    if let Node::String(node) = &ast.nodes[3] {
      assert_eq!("\"string\"", node.text.to_string());
    }
  }

  #[test]
  fn can_parse_property_number() {
    let ast = parse_text(
      r#"maintype {
        prop: 1234
    }   
    "#,
    );
    assert!(ast.is_ok());
    let ast = ast.unwrap();
    if let Node::Number(node) = &ast.nodes[3] {
      assert_eq!("1234", node.value.to_string());
    }
  }
  #[test]
  fn can_parse_property_color() {
    let ast = parse_text(
      r#"maintype {
        prop: #00aabb
    }   
    "#,
    );
    assert!(ast.is_ok());
    let ast = ast.unwrap();
    if let Node::Color(node) = &ast.nodes[3] {
      assert_eq!("00aabb", node.color.to_string());
    }
  }

  #[test]
  fn can_parse_property_vpath() {
    let ast = parse_text(
      r#"maintype {
        prop: table:variable
    }   
    "#,
    );
    assert!(ast.is_ok());
    let ast = ast.unwrap();
    if let Node::VPath(node) = &ast.nodes[3] {
      assert_eq!("table", node.table.as_ref().unwrap().to_string());
      assert_eq!("variable", node.variable.as_ref().unwrap().to_string());
    }
  }

  #[test]
  fn can_parse_property_vpath_table_only() {
    let ast = parse_text(
      r#"maintype {
        prop: table:
    }   
    "#,
    );
    assert!(ast.is_ok());
    let ast = ast.unwrap();
    if let Node::VPath(node) = &ast.nodes[3] {
      assert_eq!("table", node.table.as_ref().unwrap().to_string());
      assert_eq!(None, node.variable);
    }
  }
  #[test]
  fn can_parse_property_vpath_variable_only() {
    let ast = parse_text(
      r#"maintype {
        prop: :q1
    }   
    "#,
    );
    assert!(ast.is_ok());
    let ast = ast.unwrap();
    if let Node::VPath(node) = &ast.nodes[3] {
      assert_eq!(None, node.table);
      assert_eq!("q1", node.variable.as_ref().unwrap().to_string());
    }
  }

  #[test]
  fn can_parse_property_vpath_colon_only() {
    let ast = parse_text(
      r#"maintype {
        prop: :
    }   
    "#,
    );
    assert!(ast.is_ok());
    let ast = ast.unwrap();
    if let Node::VPath(node) = &ast.nodes[3] {
      assert_eq!(None, node.table);
      assert_eq!(None, node.variable);
    }
  }

  #[test]
  fn can_parse_property_vpath_full() {
    let ast = parse_text(
      r#"maintype {
        prop: p1234.table:variable.4
    }   
    "#,
    );
    assert!(ast.is_ok());
    let ast = ast.unwrap();
    if let Node::VPath(node) = &ast.nodes[3] {
      assert_eq!("p1234.table", node.table.as_ref().unwrap().to_string());
      assert_eq!("variable.4", node.variable.as_ref().unwrap().to_string());
    }
  }

  #[test]
  fn can_parse_nested_entity() {
    let ast = parse_text(
      r"maintype {
      otherMaintype {

      }
    }   
    ",
    );
    assert!(&ast.is_ok());
    let ast = ast.unwrap();
    if let Node::Entity(node) = &ast.nodes[1] {
      assert_eq!("maintype", node.terms[0].to_string());
      assert_eq!(NodeRef(2), node.children[0]);
    }
    if let Node::Entity(node) = &ast.nodes[2] {
      assert_eq!("otherMaintype", node.terms[0].to_string());
      assert_eq!(0, node.children.len());
    }
  }
}
