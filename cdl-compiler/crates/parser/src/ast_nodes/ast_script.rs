use std::process::Output;

use crate::{
  parser::{Node, Parser},
  types::NodeRef,
};
use anyhow::{bail, Result};
use serde::Serialize;

use super::{AstEntityNode, AstTitleNode, Parsable};

#[derive(Debug, Serialize, Clone)]
pub struct AstScriptNode {
  pub children: Vec<NodeRef>,
}

impl Parsable for AstScriptNode {
  fn can_parse(_: &Parser) -> bool {
    true
  }

  fn parse(parser: &mut Parser, _: NodeRef) -> Result<NodeRef> {
    let root_node = AstScriptNode { children: vec![] };
    let root_node_ref = parser.add_node(Node::Script(root_node), 0..usize::MAX, NodeRef(-1));
    while parser.is_tokens_left() {
      parser.eat_eol_and_comments();
      if AstTitleNode::can_parse(parser) {
        let node_ref = AstTitleNode::parse(parser, root_node_ref)?;
        parser.add_child_to_node(root_node_ref, node_ref);
        continue;
      }
      if AstEntityNode::can_parse(parser) {
        let node_ref = AstEntityNode::parse(parser, root_node_ref)?;
        parser.add_child_to_node(root_node_ref, node_ref);
        continue;
      }
      let token = parser.get_current_token()?;
      bail!(
        "Unknown token {:?} found while parsing Script Node",
        token.kind
      );
    }
    Ok(root_node_ref)
  }
}
