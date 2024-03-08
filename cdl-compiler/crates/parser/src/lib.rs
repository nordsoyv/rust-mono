mod ast_nodes;
mod parse_expr;
mod parser;
mod token_stream;
mod types;

use std::{fmt::Write, ops::Range};

use anyhow::Result;
use lexer::lex;
pub use parser::Node;
use parser::Parser;
use serde::Serialize;
use token_stream::TokenStream;
pub use types::NodeRef;

pub fn parse_text(text: &str) -> Result<Ast> {
  let tokens = lex(&text)?;
  let mut parser = Parser::new(text, TokenStream::new(tokens));
  let root_ref = parser.parse()?;

  Ok(Ast {
    nodes: parser.nodes.take(),
    locations: parser.locations.take(),
    script_entity: root_ref,
  })
}

#[derive(Debug, Serialize, Clone)]
pub struct Ast {
  pub nodes: Vec<Node>,
  pub locations: Vec<Range<usize>>,
  pub script_entity: NodeRef,
}

impl Ast {
  pub fn to_cdl(&self) -> Result<String> {
    //let script = &self.nodes[self.script_entity.0 as usize];
    let mut cdl = String::new();
    //write!(cdl, "");
    self.print_node(&mut cdl, self.script_entity, 0)?;
    return Ok(cdl);
  }

  fn print_node(
    &self,
    cdl: &mut dyn std::fmt::Write,
    node_ref: NodeRef,
    indent: usize,
  ) -> Result<()> {
    match &self.nodes[node_ref.0 as usize] {
      Node::Title(title) => self.title_to_cdl(cdl, title, indent)?,
      Node::Entity(entity) => self.entity_to_cdl(cdl, entity, indent)?,
      Node::Property(prop) => self.property_to_cdl(cdl, prop, indent)?,
      Node::Identifier(identifier) => self.identifier_to_cdl(cdl, identifier, indent)?,
      Node::Script(script) => self.script_to_cdl(cdl, script, indent)?,
      Node::String(string) => self.string_to_cdl(cdl, string, indent)?,
      Node::Number(number) => self.number_to_cdl(cdl, number, indent)?,
      Node::Boolean(boolean) => self.boolean_to_cdl(cdl, boolean, indent)?,
      Node::VPath(vpath) => self.vpath_to_cdl(cdl, vpath, indent)?,
      Node::Color(color) => self.color_to_cdl(cdl, color, indent)?,
      Node::Reference(r) => self.reference_to_cdl(cdl, r, indent)?,
      Node::Function(func) => self.func_to_cdl(cdl, func, indent)?,
      Node::Operator(op) => self.op_to_cdl(cdl, op, indent)?,
      Node::TableAlias(alias) => self.alias_to_cdl(cdl, alias, indent)?,
      Node::Formula(formula) =>self.formula_to_cdl(cdl, formula, indent)?,
    }
    Ok(())
  }

  fn script_to_cdl(
    &self,
    cdl: &mut dyn std::fmt::Write,
    s: &ast_nodes::AstScriptNode,
    indent: usize,
  ) -> Result<()> {
    for child in &s.children {
      self.print_node(cdl, *child, indent)?;
    }
    Ok(())
  }

  fn title_to_cdl(
    &self,
    cdl: &mut dyn std::fmt::Write,
    title: &ast_nodes::AstTitleNode,
    indent: usize,
  ) -> Result<()> {
    write!(cdl, "title: {}\n", title.title)?;
    Ok(())
  }

  fn entity_to_cdl(
    &self,
    cdl: &mut dyn std::fmt::Write,
    entity: &ast_nodes::AstEntityNode,
    indent: usize,
  ) -> Result<()> {
    let indent_str = create_indent(indent);
    // r#"maintype subtype "label" @ref1 @ref2 #id 3245{
    write!(cdl, "{}{}", indent_str, entity.terms.join(" "))?;

    if let Some(label) = &entity.label {
      write!(cdl, " {}", label)?;
    }

    for r in &entity.refs {
      write!(cdl, " @{}", r)?;
    }

    if let Some(id) = &entity.ident {
      write!(cdl, " #{}", id)?;
    }

    if let Some(num) = &entity.entity_number {
      write!(cdl, " {}", num)?;
    }
    write!(cdl, " {{\n")?;
    for child in &entity.children {
      self.print_node(cdl, *child, indent + 1)?;
    }
    write!(cdl, "{}}}\n", indent_str)?;
    Ok(())
  }

  fn property_to_cdl(
    &self,
    cdl: &mut dyn Write,
    prop: &ast_nodes::AstPropertyNode,
    indent: usize,
  ) -> Result<()> {
    let indent_str = create_indent(indent);
    write!(cdl, "{}{}: ", indent_str, prop.name)?;
    self.print_node(cdl, *prop.child.first().unwrap(), indent)?;
    write!(cdl, "\n")?;
    Ok(())
  }

  fn identifier_to_cdl(
    &self,
    cdl: &mut dyn Write,
    identifier: &ast_nodes::AstIdentifierNode,
    indent: usize,
  ) -> Result<()> {
    write!(cdl, "{}", identifier.identifier)?;
    Ok(())
  }

  fn string_to_cdl(
    &self,
    cdl: &mut dyn Write,
    string: &ast_nodes::AstStringNode,
    indent: usize,
  ) -> Result<()> {
    write!(cdl, "{}", string.text)?;
    Ok(())
  }

  fn number_to_cdl(
    &self,
    cdl: &mut dyn Write,
    number: &ast_nodes::AstNumberNode,
    indent: usize,
  ) -> Result<()> {
    write!(cdl, "{}", number.value.to_string())?;
    Ok(())
  }

  fn boolean_to_cdl(
    &self,
    cdl: &mut dyn Write,
    boolean: &ast_nodes::ast_boolean::AstBooleanNode,
    indent: usize,
  ) -> Result<()> {
    write!(cdl, "{}", boolean.value.to_string())?;
    Ok(())
  }

  fn color_to_cdl(
    &self,
    cdl: &mut dyn Write,
    color: &ast_nodes::AstColorNode,
    indent: usize,
  ) -> Result<()> {
    write!(cdl, "#{}", color.color)?;
    Ok(())
  }

  fn reference_to_cdl(
    &self,
    cdl: &mut dyn Write,
    r: &ast_nodes::AstReferenceNode,
    indent: usize,
  ) -> Result<()> {
    write!(cdl, "@{}", r.ident)?;
    Ok(())
  }

  fn vpath_to_cdl(
    &self,
    cdl: &mut dyn Write,
    vpath: &ast_nodes::AstVPathNode,
    indent: usize,
  ) -> Result<()> {
    if let Some(table) = &vpath.table {
      write!(cdl, "{}", table)?;
    }
    write!(cdl, ":")?;

    if vpath.is_hierarchy {
      write!(cdl, "^")?;
    }
    if let Some(variable) = &vpath.variable {
      write!(cdl, "{}", variable)?;
    }
    if let Some(func) = &vpath.function {
      write!(cdl, "{}()", func)?;
    }
    Ok(())
  }

  fn func_to_cdl(
    &self,
    cdl: &mut dyn Write,
    func: &ast_nodes::ast_function::AstFunctionNode,
    indent: usize,
  ) -> Result<()> {
    write!(cdl, "{}(", func.name)?;
    for child in &func.children {
      self.print_node(cdl, *child, indent)?;
      write!(cdl, ", ")?;
    }
    write!(cdl, ")")?;
    Ok(())
  }

  fn op_to_cdl(
    &self,
    cdl: &mut dyn Write,
    op: &ast_nodes::AstOperatorNode,
    indent: usize,
  ) -> Result<()> {
    self.print_node(cdl, op.left, indent)?;
    match op.operator {
      ast_nodes::ast_operator::Operator::Plus => write!(cdl, " + ")?,
      ast_nodes::ast_operator::Operator::Minus => write!(cdl, " - ")?,
      ast_nodes::ast_operator::Operator::Mul => write!(cdl, " * ")?,
      ast_nodes::ast_operator::Operator::Div => write!(cdl, " / ")?,
      ast_nodes::ast_operator::Operator::Equal => write!(cdl, " = ")?,
      ast_nodes::ast_operator::Operator::And => write!(cdl, " AND ")?,
      ast_nodes::ast_operator::Operator::Or => write!(cdl, " OR ")?,
      ast_nodes::ast_operator::Operator::NotEqual => write!(cdl, " != ")?,
      ast_nodes::ast_operator::Operator::LessThan => write!(cdl, " < ")?,
      ast_nodes::ast_operator::Operator::LessThanOrEqual => write!(cdl, " <= ")?,
      ast_nodes::ast_operator::Operator::MoreThan => write!(cdl, " > ")?,
      ast_nodes::ast_operator::Operator::MoreThanOrEqual => write!(cdl, " >= ")?,
    }
    self.print_node(cdl, op.right, indent)?;
    Ok(())
  }

  fn alias_to_cdl(
    &self,
    cdl: &mut dyn Write,
    alias: &ast_nodes::AstTableAliasNode,
    indent: usize,
  ) -> Result<()> {
    //table alias = dataset.table: // TODO: print this
    let indent_str = create_indent(indent);
    write!(cdl, "{}table {} = {}\n",indent_str, alias.alias, alias.table)?;
    Ok(())
  }
  
  fn formula_to_cdl(&self, cdl: &mut dyn Write, formula: &ast_nodes::AstFormulaNode, indent: usize) -> Result<()> {
    write!(cdl, "[")?;
    for child in &formula.children {
      self.print_node(cdl, *child, indent)?;
      write!(cdl, ", ")?;
    }
    write!(cdl, "]")?;
    Ok(())
    }
}

fn create_indent(indent_size: usize) -> String {
  "  ".repeat(indent_size)
}

#[cfg(test)]
mod tests {

  use super::*;

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
    //dbg!(&ast);
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
    if let Node::Entity(node) = &ast.nodes[1] {
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
    if let Node::Entity(node) = &ast.nodes[1] {
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
  fn can_parse_property_reference() {
    let ast = parse_text(
      r#"maintype {
        prop: @identifier
    }   
    "#,
    );

    assert!(ast.is_ok());
    let ast = ast.unwrap();
    if let Node::Reference(node) = &ast.nodes[3] {
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
    if let Node::Function(node) = &ast.nodes[3] {
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
    if let Node::Property(node) = &ast.nodes[3] {
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
    if let Node::Property(node) = &ast.nodes[2] {
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
    if let Node::Property(node) = &ast.nodes[2] {
      assert_eq!(vec![NodeRef(4)], node.child);
    }
    if let Node::Operator(node) = &ast.nodes[6] {
      assert_eq!(NodeRef(5), node.left);
      assert_eq!(NodeRef(7), node.right);
    }
  }

  #[test]
  fn can_parse_expressions_formula() {
    //simple_logger::SimpleLogger::new().init().unwrap();
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
    if let Node::TableAlias(node) = &ast.nodes[2] {
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
    if let Node::TableAlias(node) = &ast.nodes[2] {
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
