mod ast_nodes;
mod parse_expr;
mod parser;
mod parser_logger;
mod token_stream;

use anyhow::Result;
use ast::Ast;
use lexer::lex;
use parser::Parser;
use parser_logger::MockLogger;
use token_stream::TokenStream;

pub fn parse_text(text: &str) -> Result<Ast> {
  let tokens = lex(&text)?;
  let mut parser = Parser::new(text, TokenStream::new(tokens), Box::new(MockLogger::new()));
  let root_ref = parser.parse()?;

  Ok(parser.ast)
}

#[cfg(test)]
mod tests {

  use ast::{Node, NodeRef};

  use super::*;

  macro_rules! node_data {
    ($ast:expr, $x:literal) => {{
      let node = $ast.get_node($x.into()).unwrap();
      node.clone().borrow().node_data.clone()
    }};
  }

  #[test]
  fn can_call_to_cdl() {
    let ast = parse_text(
      r#"title "dashboard title"
      config hub {
        table alias1 = dataset.table:
        table alias2 = dataset.table
      }
      
      maintype subtype "label" @ref1 @ref2 #id 3245{
        prop: ident
        prop: "string"
        prop: 'string'
        prop: 1234
        prop: true
        prop: #aabbcc
        prop: @foo.bar
        prop: table:variable
        prop: table:
        prop: dataset.table:variable.field
        prop: :^variable
        prop: :variable()
        prop: func()
        prop: func(foo, bar)
        prop: 1 + 2 
        prop: func(1 + 2 )
        prop: score[column = %.current] - score[] // should be this
      }
      "#,
    );
    assert!(ast.is_ok());
    let ast = ast.unwrap();
    let cdl = ast.to_cdl().unwrap();
    println!("{}", cdl);
  }

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
    if let Node::Entity(node) = node_data!(ast, 1) {
      assert_eq!("maintype", node.terms[0].to_string());
      assert_eq!(0, node.children.len());
    }
  }
  #[test]
  fn can_parse_entity_header() {
    let ast = parse_text(
      r#"maintype subtype "label" @ref1 @ref2 #id 3245{

    }   
    "#,
    );
    assert!(ast.is_ok());

    let ast = ast.unwrap();
    if let Node::Entity(node) = node_data!(ast, 1) {
      assert_eq!("maintype", node.terms[0].to_string());
      assert_eq!(0, node.children.len());
      assert_eq!("\"label\"", node.label.as_ref().unwrap().to_string());
      assert_eq!("ref1", node.refs[0].to_string());
      assert_eq!("ref2", node.refs[1].to_string());
      assert_eq!("id", node.ident.as_ref().unwrap().to_string());
      assert_eq!(3245.0, *node.entity_number.as_ref().unwrap());
    }
  }

  #[test]
  fn can_parse_entity_no_body() {
    let ast = parse_text(
      r#"maintype subtype  
    "#,
    );
    assert!(ast.is_ok());
  }
  #[test]
  fn can_parse_entity_single_line() {
    let ast = parse_text(
      r#"maintype subtype { prop: ident }
    "#,
    );
    assert!(ast.is_ok());
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
    if let Node::Identifier(node) = node_data!(ast, 3) {
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
    if let Node::String(node) = node_data!(ast, 3) {
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
    if let Node::Number(node) = node_data!(ast, 3) {
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
    if let Node::Color(node) = node_data!(ast, 3) {
      assert_eq!("00aabb", node.color.to_string());
    }
  }
  #[test]
  fn can_parse_property_reference() {
    let ast = parse_text(
      r#"maintype {
        prop: @identifier
    }   
    "#,
    );

    assert!(ast.is_ok());
    let ast = ast.unwrap();
    if let Node::Reference(node) = node_data!(ast, 3) {
      assert_eq!("identifier", node.ident.to_string());
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
    if let Node::VPath(node) = node_data!(ast, 3) {
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
    if let Node::VPath(node) = node_data!(ast, 3) {
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
    if let Node::VPath(node) = node_data!(ast, 3) {
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
    if let Node::VPath(node) = node_data!(ast, 3) {
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
    if let Node::VPath(node) = node_data!(ast, 3) {
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
    if let Node::Entity(node) = node_data!(ast, 1) {
      assert_eq!("maintype", node.terms[0].to_string());
      assert_eq!(NodeRef(2), node.children[0]);
    }
    if let Node::Entity(node) = node_data!(ast, 2) {
      assert_eq!("otherMaintype", node.terms[0].to_string());
      assert_eq!(0, node.children.len());
    }
  }

  #[test]
  fn can_parse_list_of_entities() {
    let ast = parse_text(
      r#"select #OpenEnd_selector {
        label: "Select Question"
        options: item {
          label: "Visit Comments"
        },
        item {
          label: "Lodging Comments"
        }
      }
    "#,
    );
    assert!(&ast.is_ok());
  }

  #[test]
  fn can_parse_function() {
    let ast = parse_text(
      r#"maintype {
        prop: func(12,12,"asdf")
    }   
    "#,
    );
    assert!(ast.is_ok());
    let ast = ast.unwrap();
    if let Node::Function(node) = node_data!(ast, 3) {
      assert_eq!("func", node.name.to_string());
      assert_eq!(vec![NodeRef(4), NodeRef(5), NodeRef(6)], node.children);
    }
  }

  #[test]
  fn can_parse_lists() {
    let ast = parse_text(
      r#"maintype {
        prop: 12,12,"asdf"
    }   
    "#,
    );
    assert!(ast.is_ok());
    let ast = ast.unwrap();
    if let Node::Property(node) = node_data!(ast, 3) {
      assert_eq!(vec![NodeRef(4), NodeRef(5), NodeRef(6)], node.child);
    }
  }
  #[test]
  fn can_parse_expressions() {
    let ast = parse_text(
      r#"maintype {
        prop: 1 + 1
    }   
    "#,
    );
    assert!(ast.is_ok());
    let ast = ast.unwrap();
    if let Node::Property(node) = node_data!(ast, 2) {
      assert_eq!(vec![NodeRef(4)], node.child);
    }
  }

  #[test]
  fn can_parse_expressions_parents() {
    let ast = parse_text(
      r#"maintype {
        prop: 1 + (2 - 3)
    }   
    "#,
    );
    assert!(ast.is_ok());
    let ast = ast.unwrap();
    if let Node::Property(node) = node_data!(ast, 2) {
      assert_eq!(vec![NodeRef(4)], node.child);
    }
    if let Node::Operator(node) = node_data!(ast, 6) {
      assert_eq!(NodeRef(5), node.left);
      assert_eq!(NodeRef(7), node.right);
    }
  }

  #[test]
  fn can_parse_expressions_formula() {
    let ast = parse_text(
      r#"maintype {
        value: (coefficient[] - min[]) / (max[] - min[]) * 100
    }   
    "#,
    );
    assert!(ast.is_ok());
  }

  #[test]
  fn can_parse_table_alias() {
    let ast = parse_text(
      r#"config hub {
        table alias = dataset.table
    }   
    "#,
    );
    assert!(ast.is_ok());
    let ast = ast.unwrap();
    if let Node::TableAlias(node) = node_data!(ast, 2) {
      assert_eq!("alias", node.alias.to_string());
      assert_eq!("dataset.table", node.table.to_string());
    }
  }
  #[test]
  fn can_parse_table_alias_vpath() {
    let ast = parse_text(
      r#"config hub {
        table alias = dataset.table:
    }   
    "#,
    );
    assert!(ast.is_ok());
    let ast = ast.unwrap();
    if let Node::TableAlias(node) = node_data!(ast, 2) {
      assert_eq!("alias", node.alias.to_string());
      assert_eq!("dataset.table", node.table.to_string());
    }
  }

  #[test]
  fn can_parse_large_file() {
    let file = include_str!("../../../test_script/workforce.cdl");
    simple_logger::SimpleLogger::new().init().unwrap();
    let ast = parse_text(file);
    assert!(ast.is_ok());
  }

  #[test]
  fn can_parse_large_expr() {
    let ast = parse_text(
      r#"option checkbox #a {
        item bar { question: survey:s50 }
      }
  "#,
    );
    assert!(ast.is_ok());
  }
}
