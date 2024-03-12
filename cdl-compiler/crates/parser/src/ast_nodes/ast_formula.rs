use ast::{AstFormulaNode, AstNode, Node, NodeRef};
use lexer::TokenKind;

use super::Parsable;

impl Parsable for AstFormulaNode {
  fn can_parse(parser: &crate::parser::Parser) -> bool {
    let curr_token = parser.get_current_token();
    if curr_token.is_err() {
      return false;
    }
    let curr_token = curr_token.unwrap();
    //parser.trace(&format!("formula kind: {:?}", curr_token.kind));
    if curr_token.kind == TokenKind::BracketOpen {
      return true;
    }
    false
  }

  fn parse(parser: &mut crate::parser::Parser, parent: NodeRef) -> anyhow::Result<NodeRef> {
    // parser.trace("Parsing formula");
    let open_bracket_token = parser.get_current_token()?;
    parser.eat_token()?;
    let node = AstFormulaNode { children: vec![] };
    loop {
      let next_token = parser.get_current_token()?;
      parser.eat_token()?;
      if next_token.kind == TokenKind::BracketClose {
        return Ok(parser.add_node(
          AstNode::new(Node::Formula(node), parent),
          open_bracket_token.pos.start..next_token.pos.end,
        ));
      }
    }
  }
}
