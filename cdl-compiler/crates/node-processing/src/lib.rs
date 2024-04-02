mod processing_context;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use anyhow::Result;
use ast::{Ast, AstNode, Node, NodeRef};
use lexer::LexedStr;
use processing_context::{ProcessingContext, ProcessingStatus};
use tracing::trace;

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
struct RefKey {
  path: Vec<LexedStr>,
}

impl RefKey {
  #[allow(dead_code)]
  fn new() -> RefKey {
    RefKey { path: Vec::new() }
  }

  #[allow(dead_code)]
  fn add_name(&mut self, name: &LexedStr) {
    self.path.push(name.clone())
  }

  #[allow(dead_code)]
  fn is_empty(&self) -> bool {
    self.path.is_empty()
  }
}

#[derive(Debug)]
struct Task {
  node_ref: NodeRef,
  processing_context: ProcessingContext,
}

impl Task {
  fn new(node_ref: NodeRef, processing_context: ProcessingContext) -> Task {
    Task {
      node_ref,
      processing_context,
    }
  }
}

#[derive(Debug)]
pub struct NodeProcessor {
  ast: Ast,
  ref_targets: RefCell<HashMap<RefKey, NodeRef>>,
  tasks: RefCell<Vec<Task>>,
}

impl NodeProcessor {
  pub fn new(ast: Ast) -> NodeProcessor {
    NodeProcessor {
      ast,
      ref_targets: RefCell::new(HashMap::new()),
      tasks: RefCell::new(Vec::new()),
    }
  }

  pub fn process(self) -> Result<Ast> {
    let status = self.process_node(self.ast.script_entity, ProcessingContext::new());
    println!("processing status: {:?}", status);
    if self.tasks.borrow().len() > 0 {
      let tasks = self.tasks.take();
      for task in tasks {
        self.process_node(task.node_ref, task.processing_context);
      }
    }
    Ok(self.ast)
  }

  #[tracing::instrument(
    name = "node-processing",
    skip(self, processing_context),
    level = "debug"
  )]
  fn process_node(
    &self,
    node_ref: NodeRef,
    processing_context: ProcessingContext,
  ) -> ProcessingStatus {
    trace!("processing node: {:?}", node_ref);
    let node = self.get_node(node_ref).unwrap();
    let node_data = &(*node).node_data;
    let status = match node_data {
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
      Node::Reference(_) => self.process_reference(node_ref),
      Node::Function(_) => ProcessingStatus::Complete,
      Node::Operator(_) => ProcessingStatus::Complete,
      Node::TableAlias(_) => ProcessingStatus::Complete,
      Node::Formula(_) => ProcessingStatus::Complete,
    };
    // dbg!("processing status", &status);
    if status.is_complete() {
      self.set_node_processed(node_ref);
    } else {
      self.create_task(node_ref, processing_context);
    }
    status
  }

  fn get_parent(&self, node_ref: NodeRef) -> Option<NodeRef> {
    self.ast.get_parent(node_ref)
  }

  fn get_node(&self, node_ref: NodeRef) -> Option<Rc<AstNode>> {
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
      match &(*node).node_data {
        Node::Script(script_data) => script_data.children.borrow().clone(),
        _ => panic!("Expected script node"),
      }
    };
    self.process_children(children, processing_context.create_for_child())
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
      match &node.node_data {
        Node::Entity(entity_data) => entity_data.children.borrow().clone(),
        _ => panic!("Expected entity node"),
      }
    };
    self.process_children(children, processing_context.create_for_child())
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
      match &node.node_data {
        Node::Property(property_data) => (
          property_data.children.borrow().clone(),
          property_data.name.clone(),
        ),
        _ => panic!("Expected property node"),
      }
    };
    let child = children[0];
    let status = self.process_children(children, processing_context.create_for_child());
    if !status.is_complete() {
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

  #[tracing::instrument(name = "ref-resolving", skip(self), level = "debug")]
  fn add_property_reference_target(&self, property: NodeRef, name: LexedStr) {
    let mut ref_key = RefKey::new();
    let mut current_node = property;
    trace!("Starting looking for parents with names for node {}", &name);
    while let Some(parent) = self.get_parent(current_node) {
      let parent_name_option = {
        match &self.get_node(parent).unwrap().node_data {
          Node::Entity(entity) => {
            trace!("Found entity as parent {:?}", &entity.ident);
            entity.ident.clone()
          }
          Node::Script(_) => return,
          Node::Property(prop) => {
            trace!("Found property as parent {:?}", &prop.name);
            Some(prop.name.clone())
          }
          _ => panic!("did not find entity as parent during ref target"),
        }
      };
      if let Some(parent_name) = parent_name_option {
        ref_key.add_name(&parent_name);
        trace!("Adding keys {:?}", &ref_key);
        self
          .ref_targets
          .borrow_mut()
          .insert(ref_key.clone(), property);
      }
      current_node = parent;
    }
  }

  fn process_reference(&self, node_ref: NodeRef) -> ProcessingStatus {
    let node = self
      .get_node(node_ref)
      .expect("Tried to get a node, got None");
    let refernce_str = {
      match &node.node_data {
        Node::Reference(ref_data) => ref_data.ident.clone(),
        _ => panic!("Expected reference node"),
      }
    };
    let target = self.get_reference_target(refernce_str);
    if target == NodeRef(-1) {
      return ProcessingStatus::Incomplete;
    }
    match &node.node_data {
      Node::Reference(ref_data) => ref_data.set_reference(target),
      _ => panic!("Expected reference node"),
    };
    ProcessingStatus::Complete
  }

  #[tracing::instrument(name = "ref-resolving", skip(self), level = "debug")]
  fn get_reference_target(&self, refernce_str: LexedStr) -> NodeRef {
    let parts: Vec<_> = refernce_str.0.split('.').collect();
    let mut ref_key = RefKey::new();
    for part in parts.iter().rev() {
      let rc: LexedStr = (*part).into();
      ref_key.add_name(&rc);
    }
    if let Some(target_node) = self.ref_targets.borrow().get(&ref_key) {
      return *target_node;
    }
    NodeRef(-1)
  }

  fn create_task(&self, node_ref: NodeRef, processing_context: ProcessingContext) {
    self
      .tasks
      .borrow_mut()
      .push(Task::new(node_ref, processing_context));
  }
}

#[cfg(test)]
mod tests {
  use ast::select_property_value;
  use tracing::Level;

  use super::*;

  macro_rules! node_data {
    ($ast:expr, $x:literal) => {{
      let node = $ast.get_node($x.into()).unwrap();
      &(*node.clone()).node_data
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
    //  print!("{}", processed_ast.to_cdl().unwrap());
    if let Node::Reference(node) = node_data!(processed_ast, 11) {
      assert_eq!(NodeRef(6), node.resolved_node.get());
    }
  }

  #[test]
  fn should_resolve_value_refs_declared_after_use() {
    let text = r#"config hub {
      hub : 4
    }
    
    page #page1 {
      widget kpi #foo {
        tile kpi {
          value : @cr.foo
        }
      }
    }
    
    custom properties #cr {
      foo : "hello"
    }"#;
    let ast = parser::parse_text(text).unwrap();
    let np = NodeProcessor::new(ast);
    let processed_ast = np.process().unwrap();
    print!("{}", processed_ast.to_cdl().unwrap());
    let selected = select_property_value(&processed_ast, "value");
    let s = processed_ast.get_node(selected[0]).unwrap();
    if let Node::Reference(node) = &(*s).node_data {
      assert_eq!(NodeRef(11), node.resolved_node.get());
    }
  }

  #[test]
  fn should_resolve_references_which_points_on_same_target_with_different_paths() {
    // tracing_subscriber::fmt().with_max_level(Level::TRACE).init();

    let text = r#"
custom properties #cp {
   item: {
     value: 1
   }
   first: @value
   second: @item.value
   third: @cp.item.value
}
"#;
    let ast = parser::parse_text(text).unwrap();
    let np = NodeProcessor::new(ast);
    let processed_ast = np.process().unwrap();
    //print!("{}", processed_ast.to_cdl().unwrap());
    let value = select_property_value(&processed_ast, "value")[0];
    let first = select_property_value(&processed_ast, "first");
    let s = processed_ast.get_node(first[0]).unwrap();
    if let Node::Reference(node) = &(*s).node_data {
      assert_eq!(value, node.resolved_node.get());
    }
    let second = select_property_value(&processed_ast, "second");
    let s = processed_ast.get_node(second[0]).unwrap();
    if let Node::Reference(node) = &(*s).node_data {
      assert_eq!(value, node.resolved_node.get());
    }
    let third = select_property_value(&processed_ast, "third");
    let s = processed_ast.get_node(third[0]).unwrap();
    if let Node::Reference(node) = &(*s).node_data {
      assert_eq!(value, node.resolved_node.get());
    }
  }

  #[test]
  fn should_resolve_references_which_depends_on_result_of_other_references() {
    let cdl = r#"
    custom properties #first @second {
    }
    custom properties #second {
      value: 1
    }
    custom properties #third {
      value: @first.value
    }
    "#;
    let ast = parser::parse_text(cdl).unwrap();
    let np = NodeProcessor::new(ast);
    let processed_ast = np.process().unwrap();
    print!("{}", processed_ast.to_cdl().unwrap());
  }
}
