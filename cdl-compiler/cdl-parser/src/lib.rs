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
  fn can_parse_properties() {
    let ast = parse_text(
      r#"maintype {
      prop: identifier
      prop2: "string"
      prop3: 1234
      prop4: table:variable
      prop5: p1234.table:variable.4
      prop6: p1234.table:
    }   
    "#,
    );
    dbg!(&ast);
    assert!(ast.is_ok());
    let ast = ast.unwrap();
    if let Node::Property(prop) = &ast.nodes[2] {
      assert_eq!("prop", prop.name.to_string());
      assert_eq!(NodeRef(3), prop.child);
    }
    if let Node::Identifier(ident) = &ast.nodes[3] {
      assert_eq!("identifier", ident.identifier.to_string());
    }
    if let Node::Property(prop) = &ast.nodes[4] {
      assert_eq!("prop2", prop.name.to_string());
      assert_eq!(NodeRef(5), prop.child);
    }
    if let Node::String(str) = &ast.nodes[5] {
      assert_eq!("\"string\"", str.text.to_string());
    }
    if let Node::Property(prop) = &ast.nodes[6] {
      assert_eq!("prop3", prop.name.to_string());
      assert_eq!(NodeRef(7), prop.child);
    }
    if let Node::Number(number) = &ast.nodes[7] {
      assert_eq!(1234f64,number.value);
    }
    if let Node::Property(prop) = &ast.nodes[8] {
      assert_eq!("prop4", prop.name.to_string());
      assert_eq!(NodeRef(9), prop.child);
    }
    if let Node::VPath(number) = &ast.nodes[9] {
      assert_eq!("table",number.table.to_string());
      assert_eq!("variable",number.variable.as_ref().unwrap().to_string());
    }
    if let Node::Property(prop) = &ast.nodes[10] {
      assert_eq!("prop5", prop.name.to_string());
      assert_eq!(NodeRef(11), prop.child);
    }
    if let Node::VPath(number) = &ast.nodes[11] {
      assert_eq!("p1234.table",number.table.to_string());
      assert_eq!("variable.4",number.variable.as_ref().unwrap().to_string());
    }
    if let Node::Property(prop) = &ast.nodes[12] {
      assert_eq!("prop6", &prop.name.to_string());
      assert_eq!(NodeRef(13), prop.child);
    }
    if let Node::VPath(number) = &ast.nodes[13] {
      assert_eq!("p1234.table",number.table.to_string());
      assert_eq!(None,number.variable);
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
