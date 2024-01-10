use std::{cell::RefCell, ops::Range, rc::Rc};

use cdl_lexer::{get_location_from_position, Token, TokenKind, TokenStream};

use crate::{
  ast_nodes::{
    ast_boolean::AstBooleanNode, ast_function::AstFunctionNode, AstColorNode, AstEntityNode,
    AstIdentifierNode, AstNumberNode, AstOperatorNode, AstPropertyNode, AstReferenceNode,
    AstScriptNode, AstStringNode, AstTableAliasNode, AstTitleNode, AstVPathNode, Parsable,
  },
  types::NodeRef,
};
use anyhow::{Context, Result};

#[derive(Debug)]
pub enum Node {
  Title(AstTitleNode),
  Entity(AstEntityNode),
  Property(AstPropertyNode),
  Identifier(AstIdentifierNode),
  Script(AstScriptNode),
  String(AstStringNode),
  Number(AstNumberNode),
  Boolean(AstBooleanNode),
  VPath(AstVPathNode),
  Color(AstColorNode),
  Reference(AstReferenceNode),
  Function(AstFunctionNode),
  Operator(AstOperatorNode),
  TableAlias(AstTableAliasNode),
}

#[derive(Debug)]
pub struct Parser<'a> {
  text: String,
  tokens: RefCell<TokenStream<'a>>,
  pub nodes: RefCell<Vec<Node>>,
  pub locations: RefCell<Vec<Range<usize>>>,
}

impl<'a> Parser<'a> {
  pub fn new(text: &'a str, tokens: TokenStream<'a>) -> Parser<'a> {
    Parser {
      nodes: RefCell::new(Vec::new()),
      tokens: RefCell::new(tokens),
      text: text.to_string(),
      locations: RefCell::new(Vec::new()),
    }
  }
  pub fn parse(&mut self) -> Result<NodeRef> {
    AstScriptNode::parse(self, NodeRef(-1)).context(self.get_top_level_error_message())
  }

  fn get_top_level_error_message(&self) -> String {
    let pos = self.get_current_token();
    if pos.is_err() {
      "Unknown error".to_string()
    } else {
      let pos = pos.unwrap();
      let location = get_location_from_position(&self.text, &pos.pos);
      format!(
        "Error while parsing at {}:{}",
        location.start_line, location.start_pos
      )
    }
  }

  pub(crate) fn get_current_token(&self) -> Result<Token> {
    Ok(self.tokens.borrow_mut().get_current_token()?.clone())
  }

  // pub(crate) fn get_current_token_text(&self) -> Result<Option<Rc<str>>> {
  //   let token = self.tokens.borrow_mut().get_current_token()?;
  //   Ok(token.text)
  // }

  // pub(crate) fn get_current_token_pos(&self) -> Result<&Range<usize>> {
  //   let token = self.tokens.borrow_mut().get_current_token()?;
  //   Ok(&token.pos)
  // }

  pub(crate) fn get_next_token(&self, num: usize) -> Result<Token> {
    Ok(self.tokens.borrow_mut().get_nth_token(num)?.clone())
  }
  // pub(crate) fn get_next_token_text(&self, num: usize) -> Result<Option<Rc<str>>> {
  //   let token = self.tokens.borrow_mut().get_nth_token(num)?;
  //   Ok(token.text)
  // }
  // pub(crate) fn get_next_token_pos(&self, num: usize) -> Result<&Range<usize>> {
  //   let token = self.tokens.borrow_mut().get_nth_token(num)?;
  //   Ok(&token.pos)
  // }

  #[allow(dead_code)]
  pub(crate) fn eat_token(&self) -> Result<Range<usize>> {
    self.tokens.borrow_mut().eat_token()
  }

  pub(crate) fn eat_tokens(&self, num: usize) -> Result<Range<usize>> {
    self.tokens.borrow_mut().eat_tokens(num)
  }

  pub(crate) fn eat_token_of_type(&self, kind: TokenKind) -> Result<Range<usize>> {
    self.tokens.borrow_mut().eat_token_of_type(kind)
  }

  pub(crate) fn is_next_token_of_type(&self, kind: TokenKind) -> bool {
    return self.tokens.borrow_mut().is_next_token_of_type(kind);
  }

  pub(crate) fn add_node(&self, n: Node, location: Range<usize>) -> NodeRef {
    let mut nodes = self.nodes.borrow_mut();
    nodes.push(n);
    let mut locations = self.locations.borrow_mut();
    locations.push(location);
    return (nodes.len() - 1).into();
  }

  pub(crate) fn get_tokens_of_kind(&self, kind: TokenKind) -> Vec<Token> {
    let tokens = self.tokens.borrow_mut().get_tokens_of_kind(kind);
    let cloned_tokens =tokens.iter().map(|t|t.clone()).collect::<Vec<Token>>();
    return cloned_tokens;
  }

  pub(crate) fn add_child_to_node(&self, parent: NodeRef, child: NodeRef) {
    let mut nodes = self.nodes.borrow_mut();
    let node = nodes.get_mut(parent.0 as usize).unwrap();
    match node {
      Node::Entity(ent) => ent.children.push(child),
      Node::Script(script) => script.children.push(child),
      Node::Property(prop) => prop.child.push(child),
      Node::Function(func) => func.children.push(child),
      Node::Operator(op) => op.right = child,
      _ => panic!("Unknown type to set as parent {:?}", node),
    }
  }

  pub(crate) fn eat_eol_and_comments(&mut self) {
    loop {
      let curr_token = self.get_current_token();
      if curr_token.is_ok() {
        let curr_token = curr_token.unwrap();
        if curr_token.kind == TokenKind::EOL
          || curr_token.kind == TokenKind::LineComment
          || curr_token.kind == TokenKind::MultiLineComment
        {
          let _ = self.eat_token();
        } else {
          break;
        }
      }
    }
  }

  pub(crate) fn update_location_on_node(&self, node_ref: NodeRef, start: usize, end: usize) {
    let mut locations = self.locations.borrow_mut();
    locations[node_ref.0 as usize] = start..end;
  }

  pub(crate) fn get_pos_for_node(&self, node_ref: NodeRef) -> Range<usize> {
    let locations = self.locations.borrow();
    locations[node_ref.0 as usize].clone()
  }

  pub(crate) fn is_tokens_left(&self) -> bool {
    let token = self.get_current_token();
    if token.is_err() {
      return false;
    }
    
    let token = token.unwrap();
    if token.kind == TokenKind::EOF {
      return false
    }
    // if kind.kind == TokenKind::EOL {
    //   return false;
    // }
    true
  }
}
