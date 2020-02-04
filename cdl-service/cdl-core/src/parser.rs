use std::cell::RefCell;

use serde_derive::{Deserialize, Serialize};

use crate::{
  lexer::Token,
  lexer::TokenType,
  parser::{
    ast_nodes::{
      AstEntity,
      AstFunctionCall,
      AstIdentifier,
      AstNumber,
      AstOperator,
      AstProperty,
      AstString,
      AstUnaryOp,
      NodeRef,
      Operator,
    },
    utils::{
      can_start_prop,
      eat_eol_and_comments,
      eat_token_if_available,
      is_next_token,
      is_tokens_left},
  },
};
use crate::parser::ast_nodes::{AstColor, AstList, AstReference, AstTableDecl, AstTitle, AstVPath};
use crate::parser::utils::{is_config_hub_entity, parse_entity_header};

pub mod ast_nodes;
mod utils;

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub enum Node {
  Entity(AstEntity),
  Identifier(AstIdentifier),
  Number(AstNumber),
  Operator(AstOperator),
  Property(AstProperty),
  String(AstString),
  UnaryOp(AstUnaryOp),
  FunctionCall(AstFunctionCall),
  List(AstList),
  VPath(AstVPath),
  Title(AstTitle),
  TableDecl(AstTableDecl),
  Reference(AstReference),
  Color(AstColor),
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Parser {
  pub(crate) nodes: RefCell<Vec<Node>>,
  pub(crate) script_entity: NodeRef,
}

pub fn parser_to_ast(parser: Parser) -> Ast {
  Ast {
    script_entity: parser.script_entity,
    nodes: parser.nodes.into_inner(),
  }
}


#[derive(Debug, Deserialize, Serialize)]
pub struct Ast {
  pub nodes: Vec<Node>,
  pub script_entity: NodeRef,
}

impl Parser {
  pub fn new() -> Parser {
    Parser {
      nodes: RefCell::new(Vec::new()),
      script_entity: 0,
    }
  }

  pub fn get_node(&self,index: NodeRef)-> Node {
    let n = &self.nodes.borrow()[index];
    return n.clone();
  }

  pub fn parse(&mut self, tokens: Vec<Token>) -> Result<(), String> {
    let mut curr_pos = 0;
    let mut children = vec![];
    while is_tokens_left(&tokens, curr_pos) {
      curr_pos += eat_eol_and_comments(&tokens[curr_pos..]);
      let res = self.parse_entity(&tokens[curr_pos..])?;
      match res {
        (_, 0) => {}
        (entity_ref, num) => {
          curr_pos += num;
          children.push(entity_ref);
          continue;
        }
      }
      let title = self.parse_title(&tokens[curr_pos..])?;
      match title {
        (_, 0) => {}
        (title_ref, num) => {
          curr_pos += num;
          children.push(title_ref);
          continue;
        }
      }
      return Err(format!("Can not parse script . Current symbol is {:?}:{:?} at pos {:?}",
                         tokens[curr_pos].kind,
                         tokens[curr_pos].text,
                         tokens[curr_pos].start));
    }

    let copy = children.clone();
    let script_entity_id = self.add_node(Node::Entity(AstEntity {
      parent: 0,
      children,
      terms: vec![],
      refs: vec![],
      entity_id: 0.0,
      start_pos: 0,
      identifier: String::new(),
      label: String::new(),
      end_pos: tokens[tokens.len() - 1].end,
    }));
    self.set_parents(copy, script_entity_id);
    self.script_entity = script_entity_id;
    return Ok(());
  }

  fn parse_title(&self, tokens: &[Token]) -> Result<(NodeRef, usize), String> {
    if tokens[0].kind == TokenType::Identifier
      && tokens[0].text == Some("title".to_string())
      && tokens[1].kind == TokenType::String
      && tokens[2].kind == TokenType::EOL {
      let n = self.add_node(Node::Title(AstTitle {
        start_pos: tokens[0].start,
        end_pos: tokens[2].end,
        parent: 0,
        title: tokens[1].text.clone().unwrap_or("".to_string()),
      }));
      return Ok((n, 3));
    }
    return Ok((0, 0));
  }

  fn parse_entity(&self, tokens: &[Token]) -> Result<(NodeRef, usize), String> {
    let mut tokens_consumed = 0;
    let start_pos = tokens[0].start;

    let header = parse_entity_header(&tokens[0..])?;

    tokens_consumed += header.size;

    if let Some(num) = eat_token_if_available(&tokens[tokens_consumed..], TokenType::OpenBracket) {
      tokens_consumed += num;
    } else {
      return Ok((0, 0));
    }

    let (children, body_tokens_consumed) = match is_config_hub_entity(&header.terms) {
      true => self.parse_config_hub_body(&tokens[tokens_consumed..])?,
      false => self.parse_entity_body(&tokens[tokens_consumed..])?,
    };

    tokens_consumed += body_tokens_consumed;

    if let Some(num) = eat_token_if_available(&tokens[tokens_consumed..], TokenType::CloseBracket) {
      tokens_consumed += num;
    } else {
      return Ok((0, 0));
    }


    if is_tokens_left(&tokens, tokens_consumed) && is_next_token(&tokens[tokens_consumed..], TokenType::Comment) {
      tokens_consumed += 1;
    }


    if let Some(num) = eat_token_if_available(&tokens[tokens_consumed..], TokenType::EOL) {
      tokens_consumed += num;
    } else {
      return Ok((0, 0));
    }

    // fixme : Do we really need this copy? its just for iterating after node creation
    let child_copy = children.clone();

    let end_pos = tokens[tokens_consumed - 1].end;
    let entity_ref = self.add_node(Node::Entity(AstEntity {
      parent: 0,
      children,
      terms: header.terms,
      refs: header.refs,
      entity_id: header.entity_id,
      label: header.label,
      identifier: header.identifier,
      start_pos,
      end_pos,
    }));
    self.set_parents(child_copy, entity_ref);
    return Ok((entity_ref, tokens_consumed));
  }


  fn parse_config_hub_body(&self, tokens: &[Token]) -> Result<(Vec<NodeRef>, usize), String> {
    let mut tokens_consumed = 0;
    let mut children = vec![];
    loop {
      // skip empty lines and comments
      tokens_consumed += eat_eol_and_comments(&tokens[tokens_consumed..]);

      // are we done?
      if tokens[tokens_consumed].kind == TokenType::CloseBracket {
        break;
      }

      if can_start_prop(&tokens[tokens_consumed..]) {
        match self.parse_property(&tokens[tokens_consumed..])? {
          (_, 0) => {}
          (prop_ref, consumed) => {
            tokens_consumed += consumed;
            children.push(prop_ref);
            continue;
          }
        }
      }
      match self.parse_table_decl(&tokens[tokens_consumed..])? {
        (_, 0) => {}
        (table_decl, consumed) => {
          tokens_consumed += consumed;
          children.push(table_decl);
          continue;
        }
      }
      match self.parse_entity(&tokens[tokens_consumed..])? {
        (_, 0) => {}
        (child_ref, consumed) => {
          tokens_consumed += consumed;
          children.push(child_ref);
          continue;
        }
      }
      return Err(format!("Can not parse entity body. Current symbol is {:?}:{:?} at pos {:?}",
                         tokens[tokens_consumed].kind,
                         tokens[tokens_consumed].text,
                         tokens[tokens_consumed].start));
    }

    return Ok((children, tokens_consumed));
  }


  fn parse_entity_body(&self, tokens: &[Token]) -> Result<(Vec<NodeRef>, usize), String> {
    let mut tokens_consumed = 0;
    let mut children = vec![];
    loop {
      // skip empty lines and comments
      tokens_consumed += eat_eol_and_comments(&tokens[tokens_consumed..]);

      // are we done?
      if tokens[tokens_consumed].kind == TokenType::CloseBracket {
        break;
      }

      if can_start_prop(&tokens[tokens_consumed..]) {
        match self.parse_property(&tokens[tokens_consumed..])? {
          (_, 0) => {}
          (prop_ref, consumed) => {
            tokens_consumed += consumed;
            children.push(prop_ref);
            continue;
          }
        }
      }
      match self.parse_entity(&tokens[tokens_consumed..])? {
        (_, 0) => {}
        (child_ref, consumed) => {
          tokens_consumed += consumed;
          children.push(child_ref);
          continue;
        }
      }
      return Err(format!("Can not parse entity body. Current symbol is {:?}:{:?} at pos {:?}",
                         tokens[tokens_consumed].kind,
                         tokens[tokens_consumed].text,
                         tokens[tokens_consumed].start));
    }

    return Ok((children, tokens_consumed));
  }

  pub fn parse_property(&self, tokens: &[Token]) -> Result<(NodeRef, usize), String> {
    if tokens[0].kind == TokenType::Identifier && tokens[1].kind == TokenType::Colon {
      let rhs = self.parse_list(&tokens[2..])?;
      match rhs {
        (_, 0) => return Ok((0, 0)),
        (index, num) => {
          let p = AstProperty {
            parent: 0,
            rhs: index,
            name: tokens[0].text.clone().unwrap_or("".to_string()),
            start_pos: tokens[0].start,
            end_pos: tokens[1 + num].end,
          };
          let p_index = self.add_node(Node::Property(p));
          self.set_parent(index, p_index);
          return Ok((p_index, 2 + num));
        }
      }
    } else {
      return Ok((0, 0));
    }
  }

  fn parse_table_decl(&self, tokens: &[Token]) -> Result<(NodeRef, usize), String> {
    if is_next_token(tokens, TokenType::Identifier) {
      if Some("table".to_string()) != tokens[0].text {
        return Ok((0, 0));
      }
    }

    if !is_next_token(&tokens[1..], TokenType::Identifier) {
      return Ok((0, 0));
    }

    let table_name = tokens[1].text.clone().unwrap_or("".to_string());

    if !is_next_token(&tokens[2..], TokenType::Equal) {
      return Ok((0, 0));
    }
    if !is_next_token(&tokens[3..], TokenType::Identifier) {
      return Ok((0, 0));
    }
    let path_name = tokens[3].text.clone().unwrap_or("".to_string());

    let r = self.add_node(Node::TableDecl(AstTableDecl {
      parent: 0,
      name: table_name,
      path: path_name,
      start_pos: tokens[0].start,
      end_pos: tokens[3].end,
    }));
    return Ok((r, 4));
  }

  fn parse_expr(&self, tokens: &[Token]) -> Result<(NodeRef, usize), String> {
    let mut curr_pos = 0;
    let mut curr_rhs_index;
    let (term_index, tokens_consumed) = self.parse_term(tokens)?;
    if tokens_consumed == 0 {
      return Err(format!("Error when parsing expression at pos {}, token: {:?}", tokens[0].start, tokens[0].kind));
    }
    curr_rhs_index = term_index;
    curr_pos += tokens_consumed;
    loop {
      match tokens[curr_pos].kind {
        TokenType::Mul => {
          let (term_index, tokens_consumed) = self.parse_term(&tokens[(curr_pos + 1)..])?;
          curr_pos += tokens_consumed + 1; //  +1 for the Mul token
          let expr_ref = self.add_node(Node::Operator(AstOperator {
            parent: 0,
            left: curr_rhs_index,
            right: term_index,
            op: Operator::Mul,
            start_pos: tokens[0].start,
            end_pos: tokens[curr_pos - 1].end,
          }));
          self.set_parent(curr_rhs_index, expr_ref);
          self.set_parent(term_index, expr_ref);
          curr_rhs_index = expr_ref;
        }
        TokenType::Div => {
          let (term_index, tokens_consumed) = self.parse_term(&tokens[(curr_pos + 1)..])?;
          curr_pos += tokens_consumed + 1; //  +1 for the Div token
          let expr_ref = self.add_node(Node::Operator(AstOperator {
            parent: 0,
            left: curr_rhs_index,
            right: term_index,
            op: Operator::Del,
            start_pos: tokens[0].start,
            end_pos: tokens[curr_pos - 1].end,
          }));
          self.set_parent(curr_rhs_index, expr_ref);
          self.set_parent(term_index, expr_ref);
          curr_rhs_index = expr_ref;
        }
        TokenType::Equal => {
          let (term_index, tokens_consumed) = self.parse_term(&tokens[(curr_pos + 1)..])?;
          curr_pos += tokens_consumed + 1; //  +1 for the equal token
          let expr_ref = self.add_node(Node::Operator(AstOperator {
            parent: 0,
            left: curr_rhs_index,
            right: term_index,
            op: Operator::Equal,
            start_pos: tokens[0].start,
            end_pos: tokens[curr_pos - 1].end,
          }));
          self.set_parent(curr_rhs_index, expr_ref);
          self.set_parent(term_index, expr_ref);
          curr_rhs_index = expr_ref;
        }
        TokenType::LessThan => {
          let (term_index, tokens_consumed) = self.parse_term(&tokens[(curr_pos + 1)..])?;
          curr_pos += tokens_consumed + 1; //  +1 for the equal token
          let expr_ref = self.add_node(Node::Operator(AstOperator {
            parent: 0,
            left: curr_rhs_index,
            right: term_index,
            op: Operator::LessThan,
            start_pos: tokens[0].start,
            end_pos: tokens[curr_pos - 1].end,
          }));
          self.set_parent(curr_rhs_index, expr_ref);
          self.set_parent(term_index, expr_ref);
          curr_rhs_index = expr_ref;
        }
        TokenType::LessThanOrEqual => {
          let (term_index, tokens_consumed) = self.parse_term(&tokens[(curr_pos + 1)..])?;
          curr_pos += tokens_consumed + 1; //  +1 for the equal token
          let expr_ref = self.add_node(Node::Operator(AstOperator {
            parent: 0,
            left: curr_rhs_index,
            right: term_index,
            op: Operator::LessThanOrEqual,
            start_pos: tokens[0].start,
            end_pos: tokens[curr_pos - 1].end,
          }));
          self.set_parent(curr_rhs_index, expr_ref);
          self.set_parent(term_index, expr_ref);
          curr_rhs_index = expr_ref;
        }
        TokenType::MoreThan => {
          let (term_index, tokens_consumed) = self.parse_term(&tokens[(curr_pos + 1)..])?;
          curr_pos += tokens_consumed + 1; //  +1 for the equal token
          let expr_ref = self.add_node(Node::Operator(AstOperator {
            parent: 0,
            left: curr_rhs_index,
            right: term_index,
            op: Operator::MoreThan,
            start_pos: tokens[0].start,
            end_pos: tokens[curr_pos - 1].end,
          }));
          self.set_parent(curr_rhs_index, expr_ref);
          self.set_parent(term_index, expr_ref);
          curr_rhs_index = expr_ref;
        }
        TokenType::MoreThanOrEqual => {
          let (term_index, tokens_consumed) = self.parse_term(&tokens[(curr_pos + 1)..])?;
          curr_pos += tokens_consumed + 1; //  +1 for the equal token
          let expr_ref = self.add_node(Node::Operator(AstOperator {
            parent: 0,
            left: curr_rhs_index,
            right: term_index,
            op: Operator::MoreThanOrEqual,
            start_pos: tokens[0].start,
            end_pos: tokens[curr_pos - 1].end,
          }));
          self.set_parent(curr_rhs_index, expr_ref);
          self.set_parent(term_index, expr_ref);
          curr_rhs_index = expr_ref;
        }
        TokenType::Identifier => {
          let i_text = tokens[curr_pos].text.clone().unwrap_or("NONE".to_string());
          if i_text == "AND" || i_text == "OR" {
            let op = if i_text == "AND" {
              Operator::And
            } else {
              Operator::Or
            };
            let (term_index, tokens_consumed) = self.parse_term(&tokens[(curr_pos + 1)..])?;
            curr_pos += tokens_consumed + 1; //  +1 for the ident token
            let expr_ref = self.add_node(Node::Operator(AstOperator {
              parent: 0,
              left: curr_rhs_index,
              right: term_index,
              op,
              start_pos: tokens[0].start,
              end_pos: tokens[curr_pos - 1].end,
            }));
            self.set_parent(curr_rhs_index, expr_ref);
            self.set_parent(term_index, expr_ref);
            curr_rhs_index = expr_ref;
          } else {
            return Ok((curr_rhs_index, curr_pos)); // CHECK: is this right ?
          }
        }
        TokenType::EOL => {
          return Ok((curr_rhs_index, curr_pos));
        }
        TokenType::Comment => {
          return Ok((curr_rhs_index, curr_pos));
        }
        TokenType::Comma => {
          return Ok((curr_rhs_index, curr_pos));
        }
        _ => {
          return Ok((curr_rhs_index, curr_pos));
        }
      }
    }
  }

  fn parse_term(&self, tokens: &[Token]) -> Result<(NodeRef, usize), String> {
    let mut curr_pos = 0;
    let mut curr_rhs_index;
    let (left_factor_index, tokens_consumed) = self.parse_factor(tokens)?;
    if tokens_consumed == 0 {
      return Err(format!("Error when parsing term at pos {}, token: {:?}", tokens[0].start, tokens[0].kind));
    }
    curr_rhs_index = left_factor_index;
    curr_pos += tokens_consumed;
    loop {
      match tokens[curr_pos].kind {
        TokenType::Plus => {
          let (factor_index, tokens_consumed) = self.parse_factor(&tokens[(curr_pos + 1)..])?;
          curr_pos += tokens_consumed + 1; //  +1 for the Plus token
          let term_ref = self.add_node(Node::Operator(AstOperator {
            parent: 0,
            left: curr_rhs_index,
            right: factor_index,
            op: Operator::Plus,
            start_pos: tokens[0].start,
            end_pos: tokens[curr_pos - 1].end,
          }));
          self.set_parent(curr_rhs_index, term_ref);
          self.set_parent(factor_index, term_ref);
          curr_rhs_index = term_ref;
        }
        TokenType::Minus => {
          let (factor_index, tokens_consumed) = self.parse_factor(&tokens[(curr_pos + 1)..])?;
          curr_pos += tokens_consumed + 1; //  +1 for the Minus token
          let term_ref = self.add_node(Node::Operator(AstOperator {
            parent: 0,
            left: curr_rhs_index,
            right: factor_index,
            op: Operator::Minus,
            start_pos: tokens[0].start,
            end_pos: tokens[curr_pos - 1].end,
          }));
          self.set_parent(curr_rhs_index, term_ref);
          self.set_parent(factor_index, term_ref);
          curr_rhs_index = term_ref;
        }
        TokenType::EOL => {
          return Ok((curr_rhs_index, curr_pos));
        }
        TokenType::Comment => {
          return Ok((curr_rhs_index, curr_pos));
        }
        TokenType::Comma => {
          return Ok((curr_rhs_index, curr_pos));
        }
        _ => {
          return Ok((curr_rhs_index, curr_pos));
        }
      }
    }
  }

  fn parse_factor(&self, tokens: &[Token]) -> Result<(NodeRef, usize), String> {
    let curr_pos = 0;
    let rhs;
    let curr_token = &tokens[curr_pos];
    match curr_token.kind {
      TokenType::Identifier => {
        if tokens[curr_pos + 1].kind == TokenType::OpenParen {
          let func = self.parse_func(&tokens[curr_pos..])?;
          if func.1 == 0 {
            return Err(format!("Error parsing function token : {:?}, pos {}", curr_token.kind, curr_token.start));
          }
          return Ok(func);
        } else if tokens[curr_pos + 1].kind == TokenType::Colon {
          let vpath = self.parse_vpath(&tokens[curr_pos..])?;
          return Ok(vpath);
        } else {
          let ast_ident = AstIdentifier {
            parent: 0,
            start_pos: tokens[0].start,
            end_pos: tokens[0].end,
            value: tokens[0].text.clone().unwrap_or("".to_string()),
          };
          rhs = self.add_node(Node::Identifier(ast_ident));
          return Ok((rhs, 1));
        }
      }
      TokenType::String => {
        let ast_string = AstString {
          parent: 0,
          start_pos: tokens[0].start,
          end_pos: tokens[0].end,
          value: tokens[0].text.clone().unwrap_or("".to_string()),
        };
        rhs = self.add_node(Node::String(ast_string));
        return Ok((rhs, 1));
      }
      TokenType::Color => {
        let r = AstColor {
          parent: 0,
          start_pos: tokens[0].start,
          end_pos: tokens[0].end,
          value: tokens[0].text.clone().unwrap_or("".to_string()),
        };
        rhs = self.add_node(Node::Color(r));
        return Ok((rhs, 1));
      }
      TokenType::Reference => {
        let ast_ref = AstReference {
          parent: 0,
          start_pos: tokens[0].start,
          end_pos: tokens[0].end,
          value: tokens[0].text.clone().unwrap_or("".to_string()),
        };
        rhs = self.add_node(Node::Reference(ast_ref));
        return Ok((rhs, 1));
      }
      TokenType::Colon => {
        let vpath_ref = self.parse_vpath(tokens)?;
        return Ok(vpath_ref);
      }
      TokenType::Comment => {
        return Ok((0, 1));
      }
      TokenType::Number => {
        let num_string = tokens[0].text.clone().unwrap_or("".to_string());
        let num_value;
        if num_string.ends_with('%') {
          num_value = num_string[..(num_string.len() - 1)].parse::<f64>().unwrap_or(0f64) / 100.0;
        } else {
          num_value = num_string.parse::<f64>().unwrap_or(0f64);
        }
        let ast_number = AstNumber {
          parent: 0,
          start_pos: tokens[0].start,
          end_pos: tokens[0].end,
          value: num_value,
        };

        rhs = self.add_node(Node::Number(ast_number));
        return Ok((rhs, 1));
      }
      TokenType::OpenParen => {
        let expr = self.parse_expr(&tokens[1..])?;
        match expr {
          (_, 0) => return Err(format!("Error parsing factor after '(', token : {:?}, pos {}", curr_token.kind, curr_token.start)),
          (expr_index, tokens_consumed) => {
            if is_next_token(&tokens[1 + tokens_consumed..], TokenType::CloseParen) {
              return Ok((expr_index, 1 + tokens_consumed + 1));
            } else {
              return Err(format!("Error parsing factor found token : {:?} at pos {}, expected CloseParen", curr_token.kind, curr_token.start));
            }
          }
        }
      }
      TokenType::Minus => {
        let expr = self.parse_expr(&tokens[1..])?;
        match expr {
          (_, 0) => return Err(format!("Error parsing factor after '-', token : {:?}, pos {}", curr_token.kind, curr_token.start)),
          (expr_index, tokens_consumed) => {
            let op_index = self.add_node(Node::UnaryOp(AstUnaryOp {
              parent: 0,
              right: expr_index,
              op: Operator::Minus,
              start_pos: curr_token.start,
              end_pos: tokens[tokens_consumed + 1].end,
            }));
            self.set_parent(expr_index, op_index);
            return Ok((op_index, tokens_consumed + 1)); // plus 1 for the minus token
          }
        }
      }
      _ => return Err(format!("Unknown token when trying to parse factor: {:?} , at pos {:?}", curr_token.kind, curr_token.start)),
    }
  }

  fn parse_func(&self, tokens: &[Token]) -> Result<(NodeRef, usize), String> {
    if !(is_next_token(&tokens, TokenType::Identifier) && is_next_token(&tokens[1..], TokenType::OpenParen)) {
      return Ok((0, 0));
    }
    let name_token = &tokens[0];
    let (arg_list, tokens_consumed) = match self.parse_arg_list(&tokens[2..])? {
      (None, tokens_consumed) => (None, tokens_consumed),
      (list_ref, tokens_consumed) => (list_ref, tokens_consumed)
    };
    let f = AstFunctionCall {
      parent: 0,
      name: name_token.text.clone().unwrap_or("".to_string()),
      args: arg_list,
      start_pos: tokens[0].start,
      end_pos: tokens[tokens_consumed + 2].end,
    };
    let f_index = self.add_node(Node::FunctionCall(f));
    if let Some(list_index) = arg_list {
      self.set_parent(list_index, f_index);
    }
    return Ok((f_index, tokens_consumed + 2));
  }

  fn parse_list(&self, tokens: &[Token]) -> Result<(NodeRef, usize), String> {
    let mut list: Vec<NodeRef> = vec![];
    let mut curr_pos = 0;
    loop {
      let curr_token = &tokens[curr_pos];
      match curr_token.kind {
        TokenType::Comma => {
          curr_pos += 1;
          continue;
        }
        TokenType::EOL => {
          if list.len() == 1 {
            return Ok((list[0], curr_pos));
          } else {
            let list_copy = list.clone();
            let r = self.add_node(Node::List(AstList {
              parent: 0,
              items: list,
              start_pos: tokens[0].start,
              end_pos: tokens[curr_pos].end,
            }));
            self.set_parents(list_copy, r);
            return Ok((r, curr_pos));
          }
        }
        _ => {
          let res = self.parse_expr(&tokens[curr_pos..])?;
          match res {
            (expr_ref, tokens_consumed) => {
              list.push(expr_ref);
              curr_pos += tokens_consumed;
            }
          }
        }
      }
    }
  }

  fn parse_arg_list(&self, tokens: &[Token]) -> Result<(Option<NodeRef>, usize), String> {
    let mut list = vec![];
    let mut curr_pos = 0;
    loop {
      let curr_token = &tokens[curr_pos];
      match curr_token.kind {
        TokenType::CloseParen => {
          curr_pos += 1;
          break;
        }
        TokenType::Comma => curr_pos += 1,
        _ => {
          let expr = self.parse_expr(&tokens[curr_pos..])?;
          match expr {
            (_, 0) => return Err(format!("Error parsing expression in argument list, token : {:?}, pos {}", curr_token.kind, curr_token.start)),
            (node_index, tokens_consumed) => {
              list.push(node_index);
              curr_pos += tokens_consumed;
            }
          }
        }
      }
    }
    if list.len() == 0 {
      return Ok((None, curr_pos));
    }
    let list_copy = list.clone();
    let list_index = self.add_node(Node::List(AstList {
      parent: 0,
      items: list,
      start_pos: tokens[0].start,
      end_pos: tokens[curr_pos].end,
    }));
    self.set_parents(list_copy, list_index);
    return Ok((Some(list_index), curr_pos));
  }

  fn parse_vpath(&self, tokens: &[Token]) -> Result<(NodeRef, usize), String> {
    let mut curr_pos = 0;
    let token = &tokens[curr_pos];
    let source = match token.kind {
      TokenType::Identifier => {
        curr_pos += 1;
        token.text.clone().unwrap_or("".to_string())
      }
      TokenType::Colon => {
        "".to_string()
      }
      _ => return Err(format!("Error parsing vpath at token {:?}", tokens[curr_pos]))
    };


    if !is_next_token(&tokens[curr_pos..], TokenType::Colon) {
      return Err(format!("Error parsing vpath at token {:?}", tokens[1]));
    }

    curr_pos += 1;
    let q_token = &tokens[curr_pos];

    let question = match q_token.kind {
      TokenType::Identifier => {
        curr_pos += 1;
        q_token.text.clone().unwrap_or("".to_string())
      }
      _ => {
        "".to_string()
      }
    };
    let i = self.add_node(Node::VPath(AstVPath {
      parent: 0,
      source,
      question,
      start_pos: tokens[0].start,
      end_pos: tokens[curr_pos - 1].end,
    }));

    return Ok((i, curr_pos));
  }


  fn add_node(&self, e: Node) -> NodeRef {
    let mut nodes = self.nodes.borrow_mut();
    nodes.push(e);
    return nodes.len() - 1;
  }

  fn set_parents(&self, nodes_to_change: Vec<NodeRef>, new_parent: NodeRef) {
    for n in nodes_to_change {
      self.set_parent(n, new_parent);
    }
  }

  fn set_parent(&self, node_to_change: NodeRef, new_parent: NodeRef) {
    let n = &mut self.nodes.borrow_mut()[node_to_change];
    match n {
      Node::Entity(ref mut inner) => {
        inner.parent = new_parent;
      }
      Node::Property(ref mut inner) => {
        inner.parent = new_parent;
      }
      Node::Identifier(ref mut inner) => {
        inner.parent = new_parent;
      }
      Node::String(ref mut inner) => {
        inner.parent = new_parent;
      }
      Node::Operator(ref mut inner) => {
        inner.parent = new_parent;
      }
      Node::UnaryOp(ref mut inner) => {
        inner.parent = new_parent;
      }
      Node::Number(ref mut inner) => {
        inner.parent = new_parent;
      }
      Node::FunctionCall(ref mut inner) => {
        inner.parent = new_parent;
      }
      Node::List(ref mut inner) => {
        inner.parent = new_parent;
      }
      Node::VPath(ref mut inner) => {
        inner.parent = new_parent;
      }
      Node::Title(ref mut inner) => {
        inner.parent = new_parent;
      }
      Node::TableDecl(ref mut inner) => {
        inner.parent = new_parent;
      }
      Node::Reference(ref mut inner) => {
        inner.parent = new_parent;
      }
      Node::Color(ref mut inner) => {
        inner.parent = new_parent;
      }
    }
  }
}

#[cfg(test)]
mod test {
  use crate::lexer::Lexer;
  use crate::parser::{ Node, Parser};
  use crate::parser::ast_nodes::{AstList, AstNumber, AstString, AstTitle, AstVPath,AstEntity, AstIdentifier, AstProperty,};

  #[test]
  fn can_parse() {
    let mut n = Parser::new();
    let l = Lexer::new();
    let tokens = l.lex("widget kpi @default #id {\n}\n".to_string()).unwrap();
    let _r = n.parse(tokens);
    assert_eq!(n.nodes.borrow().len(), 2);
    assert_eq!(n.nodes.borrow()[0], Node::Entity(AstEntity {
      parent: 1,
      terms: vec!["widget".to_string(), "kpi".to_string()],
      refs: vec!["default".to_string()],
      entity_id: 0.0,
      label: String::new(),
      identifier: "id".to_string(),
      children: vec![],
      start_pos: 0,
      end_pos: 28,
    }));
  }

  #[test]
  fn can_parse_minimal_entity() {
    let mut n = Parser::new();
    let l = Lexer::new();
    let tokens = l.lex("summary {\n}\n".to_string()).unwrap();
    let _r = n.parse(tokens);
    assert_eq!(n.nodes.borrow().len(), 2);
    assert_eq!(n.nodes.borrow()[0], Node::Entity(AstEntity {
      parent: 1,
      terms: vec!["summary".to_string()],
      refs: vec![],
      entity_id: 0.0,
      identifier: String::new(),
      label: String::new(),
      children: vec![],
      start_pos: 0,
      end_pos: 12,
    }));
  }


  #[test]
  fn can_parse_prop() {
    let n = Parser::new();
    let l = Lexer::new();
    let tokens = l.lex("label : hello\n".to_string()).unwrap();
    let _r = n.parse_property(&tokens);
    assert_eq!(n.nodes.borrow().len(), 2);
    assert_eq!(n.nodes.borrow()[0], Node::Identifier(AstIdentifier {
      parent: 1,
      value: "hello".to_string(),
      start_pos: 8,
      end_pos: 13,
    }));
    assert_eq!(n.nodes.borrow()[1], Node::Property(AstProperty {
      parent: 0,
      name: "label".to_string(),
      rhs: 0,
      start_pos: 0,
      end_pos: 13,
    }));
  }

  #[test]
  fn can_parse_two() {
    let mut n = Parser::new();
    let l = Lexer::new();
    let tokens = l.lex("widget kpi @default #id {\n} \n widget kpi @default #id2 {\n}\n".to_string()).unwrap();
    let _r = n.parse(tokens);
    assert_eq!(n.nodes.borrow().len(), 3);
    assert_eq!(n.nodes.borrow()[0], Node::Entity(AstEntity {
      parent: 2,
      terms: vec!["widget".to_string(), "kpi".to_string()],
      refs: vec!["default".to_string()],
      entity_id: 0.0,
      identifier: "id".to_string(),
      label: String::new(),

      children: vec![],
      start_pos: 0,
      end_pos: 29,
    }));
    assert_eq!(n.nodes.borrow()[1], Node::Entity(AstEntity {
      parent: 2,
      terms: vec!["widget".to_string(), "kpi".to_string()],
      refs: vec!["default".to_string()],
      entity_id: 0.0,
      identifier: "id2".to_string(),
      label: String::new(),
      children: vec![],
      start_pos: 30,
      end_pos: 59,
    }));
  }

  #[test]
  fn can_parse_sub_ent() {
    let mut n = Parser::new();
    let l = Lexer::new();
    let tokens = l.lex("widget kpi @default #id {
    widget list {
    }
    widget list {
    }
}
".to_string()).unwrap();
    let _r = n.parse(tokens);
    assert_eq!(n.nodes.borrow().len(), 4);
    assert_eq!(n.nodes.borrow()[0], Node::Entity(AstEntity {
      parent: 2,
      terms: vec!["widget".to_string(), "list".to_string()],
      refs: vec![],
      entity_id: 0.0,
      identifier: String::new(),
      label: String::new(),

      children: vec![],
      start_pos: 30,
      end_pos: 50,
    }));
    assert_eq!(n.nodes.borrow()[2], Node::Entity(AstEntity {
      parent: 3,
      terms: vec!["widget".to_string(), "kpi".to_string()],
      refs: vec!["default".to_string()],
      entity_id: 0.0,
      identifier: "id".to_string(),
      label: String::new(),

      children: vec![0, 1],
      start_pos: 0,
      end_pos: 76,
    }));
  }

  #[test]
  fn can_parse_prop_in_entity() {
    let mut n = Parser::new();
    let l = Lexer::new();
    let tokens = l.lex("widget kpi {
   label : hello
}
".to_string()).unwrap();
    let _r = n.parse(tokens);
    assert_eq!(n.nodes.borrow().len(), 4);
    assert_eq!(n.nodes.borrow()[2], Node::Entity(AstEntity {
      parent: 3,
      terms: vec!["widget".to_string(), "kpi".to_string()],
      refs: vec![],
      entity_id: 0.0,
      identifier: String::new(),
      label: String::new(),

      children: vec![1],
      start_pos: 0,
      end_pos: 32,
    }));
    assert_eq!(n.nodes.borrow()[1], Node::Property(AstProperty {
      parent: 2,
      name: "label".to_string(),
      rhs: 0,
      start_pos: 16,
      end_pos: 29,
    }));
  }

  #[test]
  fn can_parse_prop_expr() {
    let mut n = Parser::new();
    let l = Lexer::new();
    let tokens = l.lex("widget kpi {
   label : 1 + 2 * 3 + hello + \"string\"
}
".to_string()).unwrap();
    let _r = n.parse(tokens);
    assert_eq!(n.nodes.borrow().len(), 12);
    assert_eq!(n.nodes.borrow()[10], Node::Entity(AstEntity {
      parent: 11,
      terms: vec!["widget".to_string(), "kpi".to_string()],
      refs: vec![],
      entity_id: 0.0,
      identifier: String::new(),
      label: String::new(),
      children: vec![9],
      start_pos: 0,
      end_pos: 55,
    }));
    assert_eq!(n.nodes.borrow()[9], Node::Property(AstProperty {
      parent: 10,
      name: "label".to_string(),
      rhs: 8,
      start_pos: 16,
      end_pos: 52,
    }));
  }


  #[test]
  fn create_script_node() {
    let mut n = Parser::new();
    let l = Lexer::new();
    let tokens = l.lex("widget kpi @default #id {\n}\n".to_string()).unwrap();
    let _r = n.parse(tokens);
    assert_eq!(n.nodes.borrow().len(), 2);
    assert_eq!(n.nodes.borrow()[0], Node::Entity(AstEntity {
      parent: 1,
      terms: vec!["widget".to_string(), "kpi".to_string()],
      refs: vec!["default".to_string()],
      entity_id: 0.0,
      identifier: "id".to_string(),
      label: String::new(),
      children: vec![],
      start_pos: 0,
      end_pos: 28,
    }));
    assert_eq!(n.nodes.borrow()[1], Node::Entity(AstEntity {
      parent: 0,
      terms: vec![],
      refs: vec![],
      entity_id: 0.0,
      identifier: String::new(),
      label: String::new(),
      children: vec![0],
      start_pos: 0,
      end_pos: 28,
    }));
  }

  #[test]
  fn can_parse_title() {
    let mut n = Parser::new();
    let l = Lexer::new();
    let tokens = l.lex("title \"hello title\" \n widget kpi { \n } \n".to_string()).unwrap();
    let _r = n.parse(tokens);
    assert_eq!(n.nodes.borrow().len(), 3);
    assert_eq!(n.nodes.borrow()[0], Node::Title(AstTitle {
      parent: 2,
      title: "hello title".to_string(),
      start_pos: 0,
      end_pos: 21,
    }));
  }

  #[test]
  fn can_parse_list() {
    let mut n = Parser::new();
    let l = Lexer::new();
    let tokens = l.lex(" widget kpi {\n list : 5 , \"hello\" ,7 + 8 \n } \n".to_string()).unwrap();
    let _r = n.parse(tokens);
    let num_nodes = n.nodes.borrow().len();
    assert_eq!(num_nodes, 9);
    assert_eq!(n.nodes.borrow()[0], Node::Number(AstNumber {
      start_pos: 22,
      end_pos: 23,
      parent: 5,
      value: 5.0,
    }));
    assert_eq!(n.nodes.borrow()[1], Node::String(AstString {
      start_pos: 26,
      end_pos: 33,
      parent: 5,
      value: "hello".to_string(),
    }));
    assert_eq!(n.nodes.borrow()[num_nodes - 4], Node::List(AstList {
      start_pos: 22,
      end_pos: 42,
      parent: 6,
      items: vec![0, 1, 4],
    }));
  }

  #[test]
  fn can_parse_threshold() {
    let mut n = Parser::new();
    let l = Lexer::new();
    let tokens = l.lex("widget kpi {\n prop : 100 > 100%\n }\n".to_string()).unwrap();
    let _r = n.parse(tokens);
    let num_nodes = n.nodes.borrow().len();
    assert_eq!(num_nodes, 6);
  }

  #[test]
  fn can_parse_vpaths() {
    let mut n = Parser::new();
    let l = Lexer::new();
    let tokens = l.lex("widget kpi {\n prop : survey:q1\n prop2 : survey: \n prop3 : :q1 \n }\n".to_string()).unwrap();
    let _r = n.parse(tokens);
    let num_nodes = n.nodes.borrow().len();
    assert_eq!(num_nodes, 8);
    assert_eq!(n.nodes.borrow()[0], Node::VPath(AstVPath {
      parent: 1,
      start_pos: 21,
      end_pos: 30,
      source: "survey".to_string(),
      question: "q1".to_string(),
    }));
    assert_eq!(n.nodes.borrow()[2], Node::VPath(AstVPath {
      parent: 3,
      start_pos: 40,
      end_pos: 47,
      source: "survey".to_string(),
      question: "".to_string(),
    }));
    assert_eq!(n.nodes.borrow()[4], Node::VPath(AstVPath {
      parent: 5,
      start_pos: 58,
      end_pos: 61,
      source: "".to_string(),
      question: "q1".to_string(),
    }));
  }
}

