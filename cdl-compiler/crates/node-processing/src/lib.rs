use std::{collections::HashMap, hash::Hash, ops::Range, rc::Rc};

use anyhow::Result;
use parser::{Ast, Node, NodeRef};

#[derive(Debug)]
pub struct ProcessedAst {
  nodes: Vec<Node>,
  locations: Vec<Range<usize>>,
  script_entiity: NodeRef,
}

impl ProcessedAst {
  fn new(ast: Ast) -> ProcessedAst {
    ProcessedAst {
      nodes: ast.nodes,
      locations: ast.locations,
      script_entiity: ast.script_entity,
    }
  }
}

struct RefKey {
  path: Vec<Rc<str>>
}

#[derive()]
pub struct NodeProcessor {
  ast: Ast,
  ref_targets: HashMap<RefKey, NodeRef>,
}

impl NodeProcessor {
  pub fn new(ast: Ast) -> NodeProcessor {
    NodeProcessor {
      ast,
      ref_targets: HashMap::new(),
    }
  }
  pub fn process(mut self) -> Result<ProcessedAst> {
    self.resolve_refs()?;
    Ok(ProcessedAst {
      nodes: self.ast.nodes,
      locations: self.ast.locations,
      script_entiity: self.ast.script_entity,
    })
  }
  fn resolve_refs(&mut self) -> Result<()> {
    self.create_ref_targets();

    let all_refs = self.find_all_nodes_of_type();
    dbg!(&all_refs);

    Ok(())
  }

  fn find_all_nodes_of_type(&self) -> Vec<NodeRef> {
    self
      .ast
      .nodes
      .iter()
      .enumerate()
      .filter_map(|(index, node)| {
        if node.is_reference() {
          Some(NodeRef(index as isize))
        } else {
          None
        }
      })
      .collect()
  }

  fn create_ref_targets(&self) {
    self
      .ast
      .nodes
      .iter()
      .enumerate()
      .for_each(|(index, node)| match node {
        Node::Entity(ent) => return,
        Node::Property(prop) => return,
        _ => return,
      })
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
    let mut np = NodeProcessor::new(ast);
    let processed_ast = np.process().unwrap();
    // dbg!(&processed_ast);
    // dbg!(&processed_ast.nodes[11]);
    if let Node::Reference(node) = &processed_ast.nodes[11] {
      // dbg!(node);
      assert_eq!(NodeRef(6), node.resolved_node);
    }
  }
}
