use std::{cell::RefCell, collections::HashMap, ops::Range, rc::Rc};

use anyhow::Result;
use parser::{Ast, AstNode, Node, NodeRef};

#[derive(Debug)]
pub struct ProcessedAst {
  nodes: Vec<RefCell<AstNode>>,
  locations: Vec<Range<usize>>,
  script_entity: NodeRef,
}

impl ProcessedAst {
  fn new(ast: Ast) -> ProcessedAst {
    ProcessedAst {
      nodes: ast.nodes,
      locations: ast.locations,
      script_entity: ast.script_entity,
    }
  }
}

#[derive(Debug, PartialEq, Hash, Eq)]
struct RefKey {
  path: Vec<Rc<str>>,
}

impl RefKey {
  fn new() -> RefKey {
    RefKey { path: Vec::new() }
  }

  fn add_name(&mut self, name: &Rc<str>) {
    self.path.push(name.clone())
  }
  fn is_empty(&self) -> bool {
    self.path.is_empty()
  }
}

#[derive(Debug)]
pub struct NodeProcessor {
  nodes: RefCell<Vec<RefCell<AstNode>>>,
  locations: Vec<Range<usize>>,
  script_entity: NodeRef,
  ref_targets: HashMap<RefKey, NodeRef>,
}

impl NodeProcessor {
  pub fn new(ast: Ast) -> NodeProcessor {
    NodeProcessor {
      ref_targets: HashMap::new(),
      nodes: RefCell::from(ast.nodes),
      locations: ast.locations,
      script_entity: ast.script_entity,
    }
  }

  pub fn process(mut self) -> Result<Ast> {
    self.resolve_refs()?;
    Ok(Ast {
      nodes: self.nodes.take(),
      locations: self.locations,
      script_entity: self.script_entity,
    })
  }
  fn resolve_refs(&mut self) -> Result<()> {
    self.create_ref_targets();
    dbg!(&self.ref_targets);
    let all_refs = self.find_all_reference_nodes();
    dbg!(&all_refs);

    for node_ref in all_refs {
      let nodes = self.nodes.borrow();
      let mut node = nodes[node_ref.0 as usize].borrow_mut();
      if let Node::Reference(ref mut reference_node) = node.node_data {
        let parts: Vec<&str> = reference_node.ident.split(".").collect();
        let mut key = RefKey::new();
        parts
          .into_iter()
          .rev()
          .for_each(|p| key.add_name(&Rc::from(p)));
        let found_node = self.ref_targets.get(&key);
        dbg!(&found_node);
        if let Some(found_node_ref) = found_node {
          let target = nodes[found_node_ref.0 as usize].borrow();
          if let Node::Property(prop) = &target.node_data {
            reference_node.resolved_node = prop.child[0];
          }
        }
      }
    }

    Ok(())
  }

  fn find_all_reference_nodes(&self) -> Vec<NodeRef> {
    self
      .nodes
      .borrow()
      .iter()
      .enumerate()
      .filter_map(|(index, node)| {
        if node.borrow().node_data.is_reference() {
          Some(NodeRef(index as isize))
        } else {
          None
        }
      })
      .collect()
  }

  fn create_ref_targets(&mut self) {
    let reftargets: Vec<(RefKey, NodeRef)> = self
      .nodes
      .borrow()
      .iter()
      .enumerate()
      .filter_map(|(index, node)| match &node.borrow().node_data {
        Node::Entity(ent) => {
          let mut ref_key = RefKey::new();
          let mut current_id = index.into();
          if let Some(ident) = &ent.ident {
            ref_key.add_name(ident)
          }
          while let Some(node_id) = self.get_parent(current_id) {
            match &self.nodes.borrow()[node_id.0 as usize].borrow().node_data {
              Node::Entity(ent) => {
                if let Some(ident) = &ent.ident {
                  ref_key.add_name(ident)
                }
                current_id = node_id;
              }
              Node::Property(prop) => {
                ref_key.add_name(&prop.name);
                current_id = node_id;
              }
              _ => current_id = node_id,
            }
          }
          return Some((ref_key, NodeRef(index as isize)));
        }
        Node::Property(prop) => {
          let mut ref_key = RefKey::new();
          let mut current_id = index.into();
          ref_key.add_name(&prop.name);
          while let Some(node_id) = self.get_parent(current_id) {
            match &self.nodes.borrow()[node_id.0 as usize].borrow().node_data {
              Node::Entity(ent) => {
                if let Some(ident) = &ent.ident {
                  ref_key.add_name(ident)
                }
                current_id = node_id;
              }
              Node::Property(prop) => {
                ref_key.add_name(&prop.name);
                current_id = node_id;
              }
              _ => current_id = node_id,
            }
          }
          return Some((ref_key, NodeRef(index as isize)));
        }
        _ => return None,
      })
      .collect();
    reftargets.into_iter().for_each(|(key, node_ref)| {
      if !key.is_empty() {
        self.ref_targets.insert(key, node_ref);
      }
    });
  }

  fn get_parent(&self, node_ref: NodeRef) -> Option<NodeRef> {
    if node_ref == NodeRef(0) {
      None
    } else {
      Some(self.nodes.borrow()[node_ref.0 as usize].borrow().parent)
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn can_process_value_ref() {
    let text = r#"config hub {
      hub : 4
    }
    
    custom properties #cr {
      foo : "hello"
    }
    
    page #page1 {
      widget kpi #foo {
        tile kpi {
          value : @cr.foo
        }
      }
    }"#;
    let ast = parser::parse_text(text).unwrap();
    let np = NodeProcessor::new(ast);
    let processed_ast = np.process().unwrap();
    print!("{}", processed_ast.to_cdl().unwrap());
    let ref_node = processed_ast.nodes[11].borrow();
    if let Node::Reference(node) = &ref_node.node_data {
      assert_eq!(NodeRef(6), node.resolved_node);
    }
  }
}
