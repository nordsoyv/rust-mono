mod processing_context;
use std::{borrow::Borrow, cell::RefCell, collections::HashMap, rc::Rc};

use anyhow::Result;
use ast::{Ast, AstNode, Node, NodeRef};
use processing_context::{ProcessingContext, ProcessingStatus};

// #[derive(Debug)]
// pub struct ProcessedAst {
//   nodes: Vec<RefCell<AstNode>>,
//   locations: Vec<Range<usize>>,
//   script_entity: NodeRef,
// }

// impl ProcessedAst {
//   fn new(ast: Ast) -> ProcessedAst {
//     ProcessedAst {
//       nodes: ast.nodes,
//       locations: ast.locations,
//       script_entity: ast.script_entity,
//     }
//   }
// }

#[derive(Debug, PartialEq, Hash, Eq)]
struct RefKey {
  path: Vec<Rc<str>>,
}

impl RefKey {
  #[allow(dead_code)]
  fn new() -> RefKey {
    RefKey { path: Vec::new() }
  }

  #[allow(dead_code)]
  fn add_name(&mut self, name: &Rc<str>) {
    self.path.push(name.clone())
  }

  #[allow(dead_code)]
  fn is_empty(&self) -> bool {
    self.path.is_empty()
  }
}

#[derive(Debug)]
pub struct NodeProcessor {
  ast: Ast,
  //nodes: RefCell<Vec<Rc<RefCell<AstNode>>>>,
  //locations: Vec<Range<usize>>,
  //script_entity: NodeRef,
  _ref_targets: HashMap<RefKey, NodeRef>,
}

impl NodeProcessor {
  pub fn new(ast: Ast) -> NodeProcessor {
    NodeProcessor {
      ast,
      _ref_targets: HashMap::new(),
    }
  }

  pub fn process(self) -> Result<Ast> {
    //self.resolve_refs()?;
    self.process_node(self.ast.script_entity, ProcessingContext::new());
    Ok(self.ast)
  }

  fn process_node(
    &self,
    node_ref: NodeRef,
    processing_context: ProcessingContext,
  ) -> ProcessingStatus {
    let node = self.get_node(node_ref).unwrap();
    let node_data = (*node).borrow();
    match &node_data.node_data {
      Node::Title(_) => ProcessingStatus::Complete,
      Node::Entity(_) => self.process_entity(node.clone(), processing_context.create_for_child()),
      Node::Property(_) => ProcessingStatus::Complete,
      Node::Identifier(_) => ProcessingStatus::Complete,
      Node::Script(_) => self.process_script(node.clone(), processing_context.create_for_child()),
      Node::String(_) => ProcessingStatus::Complete,
      Node::Number(_) => ProcessingStatus::Complete,
      Node::Boolean(_) => ProcessingStatus::Complete,
      Node::VPath(_) => ProcessingStatus::Complete,
      Node::Color(_) => ProcessingStatus::Complete,
      Node::Reference(_) => ProcessingStatus::Complete,
      Node::Function(_) => ProcessingStatus::Complete,
      Node::Operator(_) => ProcessingStatus::Complete,
      Node::TableAlias(_) => ProcessingStatus::Complete,
      Node::Formula(_) => ProcessingStatus::Complete,
    }
  }

  // fn resolve_refs(&mut self) -> Result<()> {
  //   self.create_ref_targets();
  //   //    dbg!(&self.ref_targets);
  //   //println!("{:?}",&self.ref_targets);
  //   let all_refs = self.find_all_reference_nodes();
  //   // dbg!(&all_refs);

  //   for node_ref in all_refs {
  //     let nodes = self.nodes.borrow();
  //     let mut node = nodes[node_ref.0 as usize].borrow_mut();
  //     if let Node::Reference(ref mut reference_node) = node.node_data {
  //       let parts: Vec<&str> = reference_node.ident.split(".").collect();
  //       let mut key = RefKey::new();
  //       parts
  //         .into_iter()
  //         .rev()
  //         .for_each(|p| key.add_name(&Rc::from(p)));
  //       let found_node = self.ref_targets.get(&key);
  //       // dbg!(&found_node);
  //       if let Some(found_node_ref) = found_node {
  //         let target = nodes[found_node_ref.0 as usize].borrow();
  //         if let Node::Property(prop) = &target.node_data {
  //           reference_node.resolved_node = prop.child[0];
  //         }
  //       }
  //       // } else {
  //       //   println!("Could not find ref: {key:?}");
  //       // }
  //     }
  //   }

  //   Ok(())
  // }

  // fn find_all_reference_nodes(&self) -> Vec<NodeRef> {
  //   self
  //     .nodes
  //     .borrow()
  //     .iter()
  //     .enumerate()
  //     .filter_map(|(index, node)| {
  //       if node.borrow().node_data.is_reference() {
  //         Some(NodeRef(index as isize))
  //       } else {
  //         None
  //       }
  //     })
  //     .collect()
  // }

  // fn create_ref_targets(&mut self) {
  //   let reftargets: Vec<(RefKey, NodeRef)> = self
  //     .nodes
  //     .borrow()
  //     .iter()
  //     .enumerate()
  //     .filter_map(|(index, node)| match &node.borrow().node_data {
  //       Node::Entity(ent) => {
  //         let mut ref_key = RefKey::new();
  //         let mut current_id = index.into();
  //         if let Some(ident) = &ent.ident {
  //           ref_key.add_name(ident)
  //         }
  //         while let Some(node_id) = self.get_parent(current_id) {
  //           match &self.nodes.borrow()[node_id.0 as usize].borrow().node_data {
  //             Node::Entity(ent) => {
  //               if let Some(ident) = &ent.ident {
  //                 ref_key.add_name(ident)
  //               }
  //               current_id = node_id;
  //             }
  //             Node::Property(prop) => {
  //               ref_key.add_name(&prop.name);
  //               current_id = node_id;
  //             }
  //             _ => current_id = node_id,
  //           }
  //         }
  //         return Some((ref_key, NodeRef(index as isize)));
  //       }
  //       Node::Property(prop) => {
  //         let mut ref_key = RefKey::new();
  //         let mut current_id = index.into();
  //         ref_key.add_name(&prop.name);
  //         while let Some(node_id) = self.get_parent(current_id) {
  //           match &self.nodes.borrow()[node_id.0 as usize].borrow().node_data {
  //             Node::Entity(ent) => {
  //               if let Some(ident) = &ent.ident {
  //                 ref_key.add_name(ident)
  //               }
  //               current_id = node_id;
  //             }
  //             Node::Property(prop) => {
  //               ref_key.add_name(&prop.name);
  //               current_id = node_id;
  //             }
  //             _ => current_id = node_id,
  //           }
  //         }
  //         return Some((ref_key, NodeRef(index as isize)));
  //       }
  //       _ => return None,
  //     })
  //     .collect();
  //   reftargets.into_iter().for_each(|(key, node_ref)| {
  //     if !key.is_empty() {
  //       self.ref_targets.insert(key, node_ref);
  //     }
  //   });
  // }

  #[allow(dead_code)]
  fn get_parent(&self, node_ref: NodeRef) -> Option<NodeRef> {
    self.ast.get_parent(node_ref)
  }

  fn get_node(&self, node_ref: NodeRef) -> Option<Rc<RefCell<AstNode>>> {
    self.ast.get_node(node_ref)
  }

  fn process_script(
    &self,
    script: Rc<RefCell<AstNode>>,
    processing_context: ProcessingContext,
  ) -> ProcessingStatus {
    let mut processing_status = ProcessingStatus::Complete;
    let mut node = (*script).borrow_mut();

    match &node.borrow().node_data {
      Node::Script(script_data) => {
        for child_node in script_data.children.iter() {
          let result = self.process_node(*child_node, processing_context.create_for_child());
          match result {
            ProcessingStatus::Complete => continue,
            ProcessingStatus::CompleteWithWarning => {
              if processing_status < ProcessingStatus::CompleteWithWarning {
                processing_status = ProcessingStatus::CompleteWithWarning
              }
            }
            ProcessingStatus::Incomplete => {
              if processing_status < ProcessingStatus::Incomplete {
                processing_status = ProcessingStatus::Incomplete
              }
            }
            ProcessingStatus::ChildIncomplete => {
              if processing_status < ProcessingStatus::ChildIncomplete {
                processing_status = ProcessingStatus::ChildIncomplete
              }
            }
            ProcessingStatus::CompleteAndAbort => {
              if processing_status < ProcessingStatus::CompleteAndAbort {
                processing_status = ProcessingStatus::CompleteAndAbort
              }
            }
          }
        }
      }
      _ => panic!("Expected script node"),
    }
    if processing_status.is_complete() {
      node.processed = true;
    }

    processing_status
  }

  fn process_entity(
    &self,
    entity: Rc<RefCell<AstNode>>,
    processing_context: ProcessingContext,
  ) -> ProcessingStatus {
    let mut processing_status = ProcessingStatus::Complete;
    let node = (*entity).borrow();
    match &node.borrow().node_data {
      Node::Entity(e) => {
        for node in e.children.iter() {
          let result = self.process_node(*node, processing_context.create_for_child());
          match result {
            ProcessingStatus::Complete => continue,
            ProcessingStatus::CompleteWithWarning => {
              if processing_status < ProcessingStatus::CompleteWithWarning {
                processing_status = ProcessingStatus::CompleteWithWarning
              }
            }
            ProcessingStatus::Incomplete => {
              if processing_status < ProcessingStatus::Incomplete {
                processing_status = ProcessingStatus::Incomplete
              }
            }
            ProcessingStatus::ChildIncomplete => {
              if processing_status < ProcessingStatus::ChildIncomplete {
                processing_status = ProcessingStatus::ChildIncomplete
              }
            }
            ProcessingStatus::CompleteAndAbort => {
              if processing_status < ProcessingStatus::CompleteAndAbort {
                processing_status = ProcessingStatus::CompleteAndAbort
              }
            }
          }
        }
        processing_status
      }
      _ => panic!("Expected entity node"),
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
    let ref_node = processed_ast.get_node(NodeRef(11)).unwrap();
    let node_data = &(*ref_node).borrow().node_data;
    //let ref_node = ref_node;
    if let Node::Reference(node) = node_data {
      assert_eq!(NodeRef(6), node.resolved_node);
    }
  }
}
