use std::cell::RefCell;

use serde_derive::{Deserialize, Serialize};

use crate::lexer::Token;
use crate::lexer::TokenType;

type EntityRef = usize;
type PropertyRef = usize;
type RhsRef = usize;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct AstEntity {
  pub terms: Vec<String>,
  pub refs: Vec<String>,
  pub entity_id: String,
  pub child_entities: Vec<EntityRef>,
  pub properties: Vec<PropertyRef>,
  pub start_pos: usize,
  pub end_pos: usize,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub enum Rhs {
  Identifier(AstIdentifier),
  String(AstString),
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct AstIdentifier {
  pub value: String,
  pub start_pos: usize,
  pub end_pos: usize,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct AstString {
  pub value: String,
  pub start_pos: usize,
  pub end_pos: usize,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct AstProperty {
  pub name: String,
  pub rhs: RhsRef,
  pub start_pos: usize,
  pub end_pos: usize,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Parser {
  pub entities: RefCell<Vec<AstEntity>>,
  pub properties: RefCell<Vec<AstProperty>>,
  pub rhs: RefCell<Vec<Rhs>>,
  pub script_entity: EntityRef,
}

fn get_tokens_of_kind(tokens: &[Token], kind: TokenType) -> Option<Vec<String>> {
  let mut curr_pos = 0;
  let mut terms = vec![];
  loop {
    if curr_pos == tokens.len() {
      break;
    }
    let curr_token = &tokens[curr_pos];
    if curr_token.kind != kind {
      break;
    }
    let text = curr_token.text.clone();
    terms.push(text.unwrap_or("".to_string()));
    curr_pos += 1;
  }
  if terms.len() == 0 {
    return None;
  }
  Some(terms)
}

fn _get_token_of_kind(tokens: &[Token], kind: TokenType) -> Option<String> {
  let curr_token = &tokens[0];
  if curr_token.kind != kind {
    return None;
  }
  return Some(curr_token.text.clone().unwrap_or("".to_string()));
}

fn get_terms(tokens: &[Token]) -> Option<Vec<String>> {
  get_tokens_of_kind(tokens, TokenType::Identifier)
}

fn get_refs(tokens: &[Token]) -> Option<Vec<String>> {
  get_tokens_of_kind(tokens, TokenType::Reference)
}

fn get_entity_id(tokens: &[Token]) -> Option<(String, usize)> {
  if tokens.len() < 2 {
    return None;
  }
  if tokens[0].kind == TokenType::Hash && tokens[1].kind == TokenType::Identifier {
    let text = tokens[1].text.clone();
    Some((text.unwrap_or("".to_string()), 2))
  } else {
    None
  }
}

fn is_tokens_left(tokens: &[Token], pos: usize) -> bool {
  tokens.len() > pos
}

fn _eat_tokens_if_available(tokens: &[Token], kind: TokenType) -> usize {
  let mut curr_pos = 0;
  loop {
    if tokens[curr_pos].kind == kind {
      curr_pos += 1;
      continue;
    } else {
      return curr_pos;
    }
  }
}

fn eat_token_if_available(tokens: &[Token], kind: TokenType) -> Option<usize> {
  if tokens[0].kind == kind {
    Some(1)
  } else {
    None
  }
}


fn eat_eol_and_comments(tokens: &[Token]) -> usize {
  let mut curr_pos = 0;
  loop {
    if tokens[curr_pos].kind == TokenType::EOL || tokens[curr_pos].kind == TokenType::Comment {
      curr_pos += 1;
      continue;
    } else {
      break;
    }
  }
  return curr_pos;
}

fn _eat_eol(tokens: &[Token]) -> usize {
  let mut curr_pos = 0;
  loop {
    if tokens[curr_pos].kind == TokenType::EOL {
      curr_pos += 1;
      continue;
    } else {
      break;
    }
  }
  return curr_pos;
}

impl Parser {
  pub fn new() -> Parser {
    Parser {
      entities: RefCell::new(Vec::new()),
      properties: RefCell::new(Vec::new()),
      rhs: RefCell::new(Vec::new()),
      script_entity: 0,
    }
  }

  pub fn parse(&mut self, tokens: Vec<Token>) -> Result<(), String> {
    let mut curr_pos = 0;
    let mut entity_refs = vec![];
    while is_tokens_left(&tokens, curr_pos) {
      if let Some(num) = eat_token_if_available(&tokens[curr_pos..], TokenType::EOL) {
        curr_pos += num;
        continue;
      }
      let res = self.parse_entity(&tokens[curr_pos..]);
      match res {
        Ok((_, 0)) => {
          println!("No match");
          return Err("No match".to_string());
        }
        Ok((entity_ref, num)) => {
          curr_pos += num;
          entity_refs.push(entity_ref);
        }

        Err(e) => {
          println!("{}", e);
          return Err(e);
        }
      }
    }

    let script_entity_id = self.add_entity(AstEntity {
      child_entities: entity_refs,
      terms: vec![],
      refs: vec![],
      properties: vec![],
      entity_id: "".to_string(),
      start_pos: 0,
      end_pos: tokens[tokens.len() - 1].end,
    });
    self.script_entity = script_entity_id;
    return Ok(());
  }

  fn add_entity(&self, e: AstEntity) -> EntityRef {
    let mut ents = self.entities.borrow_mut();
    ents.push(e);
    return ents.len() - 1;
  }

  fn add_property(&self, p: AstProperty) -> PropertyRef {
    let mut props = self.properties.borrow_mut();
    props.push(p);
    return props.len() - 1;
  }

  fn add_rhs(&self, r: Rhs) -> PropertyRef {
    let mut rhs = self.rhs.borrow_mut();
    rhs.push(r);
    return rhs.len() - 1;
  }


  pub fn parse_property(&self, tokens: &[Token]) -> Result<(PropertyRef, usize), String> {
    if tokens[0].kind == TokenType::Identifier && tokens[1].kind == TokenType::Colon {
      let rhs = self.parse_rhs(&tokens[2..]);
      match rhs {
        Ok((_, 0)) => return Ok((0, 0)),
        Ok((index, num)) => {
          let p = AstProperty {
            rhs: index,
            name: tokens[0].text.clone().unwrap_or("".to_string()),
            start_pos: tokens[0].start,
            end_pos: tokens[1 + num].end,
          };
          let p_index = self.add_property(p);
          return Ok((p_index, 2 + num));
        }
        Err(e) => return Err(e)
      }
    } else {
      return Ok((0, 0));
    }
  }

  fn parse_rhs(&self, tokens: &[Token]) -> Result<(RhsRef, usize), String> {
    let mut curr_pos = 0;
    let rhs;
    let curr_token = &tokens[curr_pos];
    match curr_token.kind {
      TokenType::Identifier => {
        let ast_ident = AstIdentifier {
          start_pos: tokens[0].start,
          end_pos: tokens[0].end,
          value: tokens[0].text.clone().unwrap_or("".to_string()),
        };
        rhs = Rhs::Identifier(ast_ident);
      }
      TokenType::String => {
        let ast_string = AstString {
          start_pos: tokens[0].start,
          end_pos: tokens[0].end,
          value: tokens[0].text.clone().unwrap_or("".to_string()),
        };
        rhs = Rhs::String(ast_string);
      }
      _ => return Err(format!("Unkwnon token when trying to parse RHS: {:?}", curr_token.kind)),
    }

    if tokens[1].kind == TokenType::EOL {
      let r_index = self.add_rhs(rhs);
      return Ok((r_index, 2));
    }
    return Ok((0, 0));
  }

  fn parse_entity(&self, tokens: &[Token]) -> Result<(EntityRef, usize), String> {
    let terms;
    let mut refs = vec![];
    let mut children = vec![];
    let mut properties = vec![];
    let mut tokens_consumed = 0;
    let mut entity_id = "".to_string();
    let start_pos = tokens[0].start;
    if let Some(t) = get_terms(&tokens[0..]) {
      tokens_consumed += t.len();
      terms = t;
    } else {
      return Ok((0, 0));
    }
    if is_tokens_left(tokens, tokens_consumed) {
      if let Some(r) = get_refs(&tokens[tokens_consumed..]) {
        tokens_consumed += r.len();
        refs = r;
      }
    }

    if is_tokens_left(tokens, tokens_consumed) {
      if let Some((parsed_entity_id, tokens_used)) = get_entity_id(&tokens[tokens_consumed..]) {
        tokens_consumed += tokens_used;
        entity_id = parsed_entity_id;
      }
    }

    if let Some(num) = eat_token_if_available(&tokens[tokens_consumed..], TokenType::OpenBracket) {
      tokens_consumed += num;
    } else {
      return Ok((0, 0));
    }


    loop {
      tokens_consumed += eat_eol_and_comments(&tokens[tokens_consumed..]);

      match self.parse_entity(&tokens[tokens_consumed..]) {
        Ok((_, 0)) => {}
        Ok((child_ref, consumed)) => {
          tokens_consumed += consumed;
          children.push(child_ref);
          continue;
        }
        Err(e) => return Err(e),
      }

      match self.parse_property(&tokens[tokens_consumed..]) {
        Ok((_, 0)) => {}
        Ok((prop_ref, consumed)) => {
          tokens_consumed += consumed;
          properties.push(prop_ref);
          continue;
        }
        Err(e) => return Err(e),
      }
      break;
    }
    tokens_consumed += eat_eol_and_comments(&tokens[tokens_consumed..]);

    if let Some(num) = eat_token_if_available(&tokens[tokens_consumed..], TokenType::CloseBracket) {
      tokens_consumed += num;
    } else {
      return Ok((0, 0));
    }

    if let Some(num) = eat_token_if_available(&tokens[tokens_consumed..], TokenType::EOL) {
      tokens_consumed += num;
    } else {
      return Ok((0, 0));
    }

    let end_pos = tokens[tokens_consumed - 1].end;
    let entity_ref = self.add_entity(AstEntity {
      child_entities: children,
      properties,
      terms,
      refs,
      entity_id,
      start_pos,
      end_pos,
    });

    return Ok((entity_ref, tokens_consumed));
  }
}

#[cfg(test)]
mod test {
  use crate::lexer::Lexer;
  use crate::parser::{AstEntity, Parser, Rhs, AstProperty};

  #[test]
  fn can_parse() {
    let mut n = Parser::new();
    let l = Lexer::new();
    let tokens = l.lex("widget kpi @default #id {\n}\n".to_string()).unwrap();
    n.parse(tokens);
    assert_eq!(n.entities.borrow().len(), 2);
    assert_eq!(n.entities.borrow()[0], AstEntity {
      terms: vec!["widget".to_string(), "kpi".to_string()],
      refs: vec!["default".to_string()],
      entity_id: "id".to_string(),
      child_entities: vec![],
      properties: vec![],
      start_pos: 0,
      end_pos: 28,
    });
  }

  #[test]
  fn can_parse_prop() {
    let n = Parser::new();
    let l = Lexer::new();
    let tokens = l.lex("label : hello\n".to_string()).unwrap();
    let _r = n.parse_property(&tokens);
    assert_eq!(n.entities.borrow().len(), 0);
    assert_eq!(n.rhs.borrow().len(), 1);
    assert_eq!(n.rhs.borrow()[0], Rhs::Identifier("hello".to_string()));
    assert_eq!(n.properties.borrow().len(), 1);
    assert_eq!(n.properties.borrow()[0], AstProperty {
      name: "label".to_string(),
      rhs: 0,
      start_pos: 0,
      end_pos: 14,
    });
  }

  #[test]
  fn can_parse_two() {
    let mut n = Parser::new();
    let l = Lexer::new();
    let tokens = l.lex("widget kpi @default #id {\n} \n widget kpi @default #id2 {\n}\n".to_string()).unwrap();
    n.parse(tokens);
    assert_eq!(n.entities.borrow().len(), 3);
    assert_eq!(n.entities.borrow()[0], AstEntity {
      terms: vec!["widget".to_string(), "kpi".to_string()],
      refs: vec!["default".to_string()],
      entity_id: "id".to_string(),
      child_entities: vec![],
      properties: vec![],
      start_pos: 0,
      end_pos: 29,
    });
    assert_eq!(n.entities.borrow()[1], AstEntity {
      terms: vec!["widget".to_string(), "kpi".to_string()],
      refs: vec!["default".to_string()],
      entity_id: "id2".to_string(),
      child_entities: vec![],
      properties: vec![],
      start_pos: 30,
      end_pos: 59,
    });
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
    n.parse(tokens);
    assert_eq!(n.entities.borrow().len(), 4);
    assert_eq!(n.entities.borrow()[0], AstEntity {
      terms: vec!["widget".to_string(), "list".to_string()],
      refs: vec![],
      entity_id: "".to_string(),
      child_entities: vec![],
      properties: vec![],
      start_pos: 30,
      end_pos: 50,
    });
    assert_eq!(n.entities.borrow()[2], AstEntity {
      terms: vec!["widget".to_string(), "kpi".to_string()],
      refs: vec!["default".to_string()],
      entity_id: "id".to_string(),
      child_entities: vec![0, 1],
      properties: vec![],
      start_pos: 0,
      end_pos: 76,
    });
  }

  #[test]
  fn can_parse_prop_in_entity() {
    let mut n = Parser::new();
    let l = Lexer::new();
    let tokens = l.lex("widget kpi {
   label : hello
}
".to_string()).unwrap();
    n.parse(tokens);
    assert_eq!(n.entities.borrow().len(), 2);
    assert_eq!(n.entities.borrow()[0], AstEntity {
      terms: vec!["widget".to_string(), "kpi".to_string()],
      refs: vec![],
      entity_id: "".to_string(),
      child_entities: vec![],
      properties: vec![0],
      start_pos: 0,
      end_pos: 32,
    });
    assert_eq!(n.properties.borrow()[0], AstProperty {
      name: "label".to_string(),
      rhs: 0,
      start_pos: 16,
      end_pos: 30,
    });
  }


  #[test]
  fn create_script_node() {
    let mut n = Parser::new();
    let l = Lexer::new();
    let tokens = l.lex("widget kpi @default #id {\n}\n".to_string()).unwrap();
    n.parse(tokens);
    assert_eq!(n.entities.borrow().len(), 2);
    assert_eq!(n.entities.borrow()[0], AstEntity {
      terms: vec!["widget".to_string(), "kpi".to_string()],
      refs: vec!["default".to_string()],
      entity_id: "id".to_string(),
      child_entities: vec![],
      properties: vec![],
      start_pos: 0,
      end_pos: 28,
    });
    assert_eq!(n.entities.borrow()[1], AstEntity {
      terms: vec![],
      refs: vec![],
      entity_id: "".to_string(),
      child_entities: vec![0],
      properties: vec![],
      start_pos: 0,
      end_pos: 28,
    });
  }
}

