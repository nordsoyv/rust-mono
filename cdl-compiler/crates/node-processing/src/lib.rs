mod processing_context;
use std::{ cell::RefCell, collections::HashMap, rc::Rc};

use anyhow::Result;
use ast::{Ast, AstNode, Node, NodeRef};
use processing_context::{ProcessingContext, ProcessingStatus};

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
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
  ref_targets: RefCell<HashMap<RefKey, NodeRef>>,
}

impl NodeProcessor {
  pub fn new(ast: Ast) -> NodeProcessor {
    NodeProcessor {
      ast,
      ref_targets: RefCell::new(HashMap::new()),
    }
  }

  pub fn process(self) -> Result<Ast> {
    self.process_node(self.ast.script_entity, ProcessingContext::new());
    //dbg!(&self.ref_targets.borrow());
    Ok(self.ast)
  }

  fn process_node(
    &self,
    node_ref: NodeRef,
    processing_context: ProcessingContext,
  ) -> ProcessingStatus {
    let node = self.get_node(node_ref).unwrap();
    let node_data = (*node).borrow();
    let status = match &node_data.node_data {
      Node::Title(_) => ProcessingStatus::Complete,
      Node::Entity(_) => self.process_entity(node_ref, processing_context.create_for_child()),
      Node::Property(_) => self.process_property(node_ref, processing_context.create_for_child()),
      Node::Identifier(_) => ProcessingStatus::Complete,
      Node::Script(_) => self.process_script(node_ref, processing_context.create_for_child()),
      Node::String(_) => ProcessingStatus::Complete,
      Node::Number(_) => ProcessingStatus::Complete,
      Node::Boolean(_) => ProcessingStatus::Complete,
      Node::VPath(_) => ProcessingStatus::Complete,
      Node::Color(_) => ProcessingStatus::Complete,
      Node::Reference(_) => self.process_reference(node_ref, processing_context.create_for_child()),
      Node::Function(_) => ProcessingStatus::Complete,
      Node::Operator(_) => ProcessingStatus::Complete,
      Node::TableAlias(_) => ProcessingStatus::Complete,
      Node::Formula(_) => ProcessingStatus::Complete,
    };
    if status.is_complete() {
      self.set_node_processed(node_ref);
    }
    status
  }
  
  #[allow(dead_code)]
  fn get_parent(&self, node_ref: NodeRef) -> Option<NodeRef> {
    self.ast.get_parent(node_ref)
  }

  fn get_node(&self, node_ref: NodeRef) -> Option<Rc<RefCell<AstNode>>> {
    self.ast.get_node(node_ref)
  }

  fn set_node_processed(&self, node_ref: NodeRef) {
    self.ast.set_node_processed(node_ref);
  }

  fn process_script(
    &self,
    node_ref: NodeRef,
    processing_context: ProcessingContext,
  ) -> ProcessingStatus {
    let node = self
      .get_node(node_ref)
      .expect("Tried to get an script node, got None");
    let children = {
      match &(*node).borrow().node_data {
        Node::Script(script_data) => script_data.children.clone(),
        _ => panic!("Expected script node"),
      }
    };
    self.process_children(children, processing_context)
  }

  fn process_entity(
    &self,
    node_ref: NodeRef,
    processing_context: ProcessingContext,
  ) -> ProcessingStatus {
    let node = self
      .get_node(node_ref)
      .expect("Tried to get an entity node, got None");
    let children = {
      match &(*node).borrow().node_data {
        Node::Entity(entity_data) => entity_data.children.clone(),
        _ => panic!("Expected entity node"),
      }
    };
    self.process_children(children, processing_context)
  }
  fn process_property(
    &self,
    node_ref: NodeRef,
    processing_context: ProcessingContext,
  ) -> ProcessingStatus {
    let node = self
      .get_node(node_ref)
      .expect("Tried to get an property node, got None");
    let (children, name) = {
      match &(*node).borrow().node_data {
        Node::Property(property_data) => (property_data.child.clone(), property_data.name.clone()),
        _ => panic!("Expected property node"),
      }
    };
    let child = children[0];
    let status = self.process_children(children, processing_context);
    if !status.is_complete() {
      // add task here
      return status;
    }
    self.add_property_reference_target(child, name);
    status
  }

  fn process_children(
    &self,
    children: Vec<NodeRef>,
    processing_context: ProcessingContext,
  ) -> ProcessingStatus {
    let mut processing_status = ProcessingStatus::Complete;
    for node_ref in children {
      let result = self.process_node(node_ref, processing_context.create_for_child());
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

  fn add_property_reference_target(&self, property: NodeRef, name: Rc<str>) {
    let mut ref_key = RefKey::new();
    ref_key.add_name(&name);
    self
      .ref_targets
      .borrow_mut()
      .insert(ref_key.clone(), property);
    let mut current_node = property;
    while let Some(parent) = self.get_parent(current_node) {
      //        let mut next_key = ref_key;
      //      dbg!(&parent);
      let parent_name_option = {
        match &self.get_node(parent).unwrap().borrow().node_data {
          Node::Entity(prop) => prop.ident.clone(),
          Node::Script(_) => return,
          Node::Property(_) => {
            current_node = parent;
            continue;
          }
          _ => panic!("did not find entity as parent during ref target"),
        }
      };
      if let Some(parent_name) = parent_name_option {
        ref_key.add_name(&parent_name);
        self
          .ref_targets
          .borrow_mut()
          .insert(ref_key.clone(), property);
      }
      current_node = parent;
    }
  }

  fn process_reference(
    &self,
    node_ref: NodeRef,
    _processing_context: ProcessingContext,
  ) -> ProcessingStatus {
    let node = self
      .get_node(node_ref)
      .expect("Tried to get a node, got None");
    let refernce_str = {
      match &(*node).borrow().node_data {
        Node::Reference(ref_data) => ref_data.ident.clone(),
        _ => panic!("Expected reference node"),
      }
    };
    let target = self.get_reference_target(refernce_str);
    match &(*node).borrow().node_data {
      Node::Reference( ref_data) => ref_data.set_reference(target),
      _ => panic!("Expected reference node"),
    };
    ProcessingStatus::Complete
  }

  fn get_reference_target(&self, refernce_str: Rc<str>) -> NodeRef {
    let parts: Vec<_> = refernce_str.split('.').collect();
    let mut ref_key = RefKey::new();
    for part in parts.iter().rev() {
      let rc: Rc<str> = (*part).into();
      ref_key.add_name(&rc);
    }
    if let Some(target_node) = self.ref_targets.borrow().get(&ref_key) {
      return *target_node;
    }
    NodeRef(-1)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  macro_rules! node_data {
    ($ast:expr, $x:literal) => {{
      let node = $ast.get_node($x.into()).unwrap();
      node.clone().borrow().node_data.clone()
    }};
  }

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
    //print!("{}", processed_ast.to_cdl().unwrap());
    if let Node::Reference(node) = node_data!(processed_ast, 11) {
      assert_eq!(NodeRef(6), node.resolved_node.get());
    }
  }
}
