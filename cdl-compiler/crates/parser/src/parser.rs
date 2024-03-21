use std::ops::Range;

use ast::{Ast, AstNode, AstScriptNode, NodeRef};
use lexer::{get_location_from_position, Token, TokenKind};

use crate::{ast_nodes::Parsable, token_stream::TokenStream};
use anyhow::{Context, Result};

#[derive(Debug)]
pub struct Parser {
  text: String,
  tokens: TokenStream,
  pub ast: Ast,
}

impl Parser {
  pub fn new(text: &str, tokens: TokenStream) -> Parser {
    Parser {
      tokens,
      text: text.to_string(),
      ast: Ast::new(),
    }
  }
  pub fn parse(&mut self) -> Result<NodeRef> {
    AstScriptNode::parse(self, NodeRef(-1)).context(self.get_top_level_error_message())
  }

  fn get_top_level_error_message(&self) -> String {
    let token = self.get_current_token();
    if token.is_err() {
      "Unknown error".to_string()
    } else {
      let token = token.unwrap();
      let location = get_location_from_position(&self.text, &token.pos);
      format!(
        "Error while parsing at {}:{}",
        location.start_line, location.start_pos
      )
    }
  }

  pub(crate) fn get_current_token(&self) -> Result<&Token> {
    self.tokens.get_current_token()
  }

  pub(crate) fn get_next_token(&self, num: usize) -> Result<&Token> {
    self.tokens.get_nth_token(num)
  }

  pub(crate) fn eat_token(&self) -> Result<Range<usize>> {
    self.tokens.eat_token()
  }

  pub(crate) fn eat_tokens(&self, num: usize) -> Result<Range<usize>> {
    self.tokens.eat_tokens(num)
  }

  pub(crate) fn eat_token_of_type(&self, kind: TokenKind) -> Result<Range<usize>> {
    self.tokens.eat_token_of_type(kind)
  }

  pub(crate) fn is_next_token_of_type(&self, kind: TokenKind) -> bool {
    self.tokens.is_next_token_of_type(kind)
  }

  pub(crate) fn add_node(&self, n: AstNode, location: Range<usize>) -> NodeRef {
    self.ast.add_node(n, location)
  }

  pub(crate) fn get_tokens_of_kind(&self, kind: TokenKind) -> &[Token] {
    self.tokens.get_tokens_of_kind(kind)
  }

  pub(crate) fn add_child_to_node(&self, parent: NodeRef, child: NodeRef) {
    self.ast.add_child_to_node(parent, child);
  }

  pub(crate) fn is_tokens_left(&self) -> bool {
    self.tokens.is_tokens_left()
  }

  pub(crate) fn eat_eol_and_comments(&mut self) {
    while self.is_tokens_left() {
      let curr_token = self.get_current_token().unwrap();
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

  pub(crate) fn update_location_on_node(&self, node_ref: NodeRef, start: usize, end: usize) {
    self.ast.update_location_on_node(node_ref, start, end);
  }

  pub(crate) fn get_pos_for_node(&self, node_ref: NodeRef) -> Range<usize> {
    self.ast.get_pos_for_node(node_ref)
  }
}
