use anyhow::Result;
use serde::Serialize;
use std::{cell::RefCell, fmt::Write, ops::Range, rc::Rc};

use crate::{
  ast_nodes::Operator, AstBooleanNode, AstColorNode, AstEntityNode, AstFormulaNode,
  AstFunctionNode, AstIdentifierNode, AstNode, AstNumberNode, AstOperatorNode, AstPropertyNode,
  AstReferenceNode, AstScriptNode, AstStringNode, AstTableAliasNode, AstTitleNode, AstVPathNode,
  Node, NodeRef,
};

#[derive(Debug, Serialize, Clone)]
pub struct Ast {
  pub nodes: RefCell<Vec<Rc<RefCell<AstNode>>>>,
  pub locations: RefCell<Vec<Range<usize>>>,
  pub script_entity: NodeRef,
}

impl Default for Ast {
    fn default() -> Self {
        Self::new()
    }
}

impl Ast {
  pub fn new() -> Ast {
    Ast {
      nodes: RefCell::new(Vec::new()),
      locations: RefCell::new(Vec::new()),
      script_entity: NodeRef(0),
    }
  }
  pub fn get_parent(&self, node_ref: NodeRef) -> Option<NodeRef> {
    if node_ref == NodeRef(0) {
      None
    } else {
      self.get_node(node_ref).map(|node| (*node).borrow().parent)
    }
  }

  pub fn get_node(&self, node_ref: NodeRef) -> Option<Rc<RefCell<AstNode>>> {
    let nodes = self.nodes.borrow();
    let node = nodes.get(node_ref.0 as usize);
    node.cloned()
  }


  pub fn add_node(&self, n: AstNode, location: Range<usize>) -> NodeRef {
    let mut nodes = self.nodes.borrow_mut();
    nodes.push(Rc::new(RefCell::new(n)));
    let mut locations = self.locations.borrow_mut();
    locations.push(location);
    (nodes.len() - 1).into()
  }

  pub fn add_child_to_node(&self, parent: NodeRef, child: NodeRef) {
    let nodes = self.nodes.borrow();
    let node = &nodes[parent.0 as usize];

    let mut node = node.borrow_mut();
    node.add_child_to_node(child);
  }

  pub fn update_location_on_node(&self, node_ref: NodeRef, start: usize, end: usize) {
    let mut locations = self.locations.borrow_mut();
    locations[node_ref.0 as usize] = start..end;
  }

  pub fn get_pos_for_node(&self, node_ref: NodeRef) -> Range<usize> {
    let locations = self.locations.borrow();
    locations[node_ref.0 as usize].clone()
  }

  pub fn to_cdl(&self) -> Result<String> {
    let mut cdl = String::new();
    self.print_node(&mut cdl, self.script_entity, 0)?;
    Ok(cdl)
  }

  fn print_node(
    &self,
    cdl: &mut dyn std::fmt::Write,
    node_ref: NodeRef,
    indent: usize,
  ) -> Result<()> {
    let nodes = self.nodes.borrow();
    let node_data = &nodes[node_ref.0 as usize].borrow().node_data;
    match node_data {
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
      Node::Formula(formula) => self.formula_to_cdl(cdl, formula, indent)?,
    }
    Ok(())
  }

  fn script_to_cdl(
    &self,
    cdl: &mut dyn std::fmt::Write,
    s: &AstScriptNode,
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
    title: &AstTitleNode,
    _indent: usize,
  ) -> Result<()> {
    writeln!(cdl, "title: {}", title.title)?;
    Ok(())
  }

  fn entity_to_cdl(
    &self,
    cdl: &mut dyn std::fmt::Write,
    entity: &AstEntityNode,
    indent: usize,
  ) -> Result<()> {
    let indent_str = create_indent(indent);
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
    writeln!(cdl, " {{")?;
    for child in &entity.children {
      self.print_node(cdl, *child, indent + 1)?;
    }
    writeln!(cdl, "{}}}", indent_str)?;
    Ok(())
  }

  fn property_to_cdl(
    &self,
    cdl: &mut dyn Write,
    prop: &AstPropertyNode,
    indent: usize,
  ) -> Result<()> {
    let indent_str = create_indent(indent);
    write!(cdl, "{}{}: ", indent_str, prop.name)?;
    self.print_node(cdl, *prop.child.first().unwrap(), indent)?;
    writeln!(cdl)?;
    Ok(())
  }

  fn identifier_to_cdl(
    &self,
    cdl: &mut dyn Write,
    identifier: &AstIdentifierNode,
    _indent: usize,
  ) -> Result<()> {
    write!(cdl, "{}", identifier.identifier)?;
    Ok(())
  }

  fn string_to_cdl(
    &self,
    cdl: &mut dyn Write,
    string: &AstStringNode,
    _indent: usize,
  ) -> Result<()> {
    write!(cdl, "{}", string.text)?;
    Ok(())
  }

  fn number_to_cdl(
    &self,
    cdl: &mut dyn Write,
    number: &AstNumberNode,
    _indent: usize,
  ) -> Result<()> {
    write!(cdl, "{}", number.value)?;
    Ok(())
  }

  fn boolean_to_cdl(
    &self,
    cdl: &mut dyn Write,
    boolean: &AstBooleanNode,
    _indent: usize,
  ) -> Result<()> {
    write!(cdl, "{}", boolean.value)?;
    Ok(())
  }

  fn color_to_cdl(&self, cdl: &mut dyn Write, color: &AstColorNode, _indent: usize) -> Result<()> {
    write!(cdl, "#{}", color.color)?;
    Ok(())
  }

  fn reference_to_cdl(
    &self,
    cdl: &mut dyn Write,
    r: &AstReferenceNode,
    indent: usize,
  ) -> Result<()> {
    if r.resolved_node == NodeRef(-1) {
      write!(cdl, "@{}", r.ident)?;
    } else {
      self.print_node(cdl, r.resolved_node, indent)?;
    }
    Ok(())
  }

  fn vpath_to_cdl(&self, cdl: &mut dyn Write, vpath: &AstVPathNode, _indent: usize) -> Result<()> {
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

  fn func_to_cdl(&self, cdl: &mut dyn Write, func: &AstFunctionNode, indent: usize) -> Result<()> {
    write!(cdl, "{}(", func.name)?;
    for child in &func.children {
      self.print_node(cdl, *child, indent)?;
      write!(cdl, ", ")?;
    }
    write!(cdl, ")")?;
    Ok(())
  }

  fn op_to_cdl(&self, cdl: &mut dyn Write, op: &AstOperatorNode, indent: usize) -> Result<()> {
    self.print_node(cdl, op.left, indent)?;
    match op.operator {
      Operator::Plus => write!(cdl, " + ")?,
      Operator::Minus => write!(cdl, " - ")?,
      Operator::Mul => write!(cdl, " * ")?,
      Operator::Div => write!(cdl, " / ")?,
      Operator::Equal => write!(cdl, " = ")?,
      Operator::And => write!(cdl, " AND ")?,
      Operator::Or => write!(cdl, " OR ")?,
      Operator::NotEqual => write!(cdl, " != ")?,
      Operator::LessThan => write!(cdl, " < ")?,
      Operator::LessThanOrEqual => write!(cdl, " <= ")?,
      Operator::MoreThan => write!(cdl, " > ")?,
      Operator::MoreThanOrEqual => write!(cdl, " >= ")?,
    }
    self.print_node(cdl, op.right, indent)?;
    Ok(())
  }

  fn alias_to_cdl(
    &self,
    cdl: &mut dyn Write,
    alias: &AstTableAliasNode,
    indent: usize,
  ) -> Result<()> {
    let indent_str = create_indent(indent);
    writeln!(
      cdl,
      "{}table {} = {}",
      indent_str, alias.alias, alias.table
    )?;
    Ok(())
  }

  fn formula_to_cdl(
    &self,
    cdl: &mut dyn Write,
    formula: &AstFormulaNode,
    indent: usize,
  ) -> Result<()> {
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
