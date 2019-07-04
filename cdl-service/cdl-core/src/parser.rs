use serde_derive::{Deserialize, Serialize};
use crate::lexer::Token;
use crate::lexer::TokenType;
use std::cell::RefCell;

type EntityRef = usize;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct AstEntity {
  pub terms: Vec<String>,
  pub refs: Vec<String>,
  pub entity_id: String,
  pub child_entities: Vec<EntityRef>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Parser {
  pub entities: RefCell<Vec<AstEntity>>,
  tokens: Vec<Token>,
  curr_pos: usize,
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

impl Parser {
  pub fn new() -> Parser {
    Parser {
      entities: RefCell::new(Vec::new()),
      tokens: vec![],
      curr_pos: 0,
    }
  }

  pub fn parse(&self, tokens: Vec<Token>) {
    let mut curr_pos = 0;
    while is_tokens_left(&tokens, curr_pos) {
      if let Some(num) = self.eat_token_if_available(&tokens[curr_pos..], TokenType::EOL) {
        curr_pos += num;
        continue;
      }
      let res = self.parse_entity(&tokens[curr_pos..]);
      match res {
        Ok(0) => println!("No mathc"),
        Ok(num) => {
          dbg!(num);
          println!("Found match {}", num);
          curr_pos += num;
        }

        Err(e) => println!("{}", e)
      }
    }
  }

  fn add_entity(&self, e: AstEntity) {
    self.entities.borrow_mut().push(e);
  }

  fn eat_token_if_available(&self, tokens: &[Token], kind: TokenType) -> Option<usize> {
    if tokens[0].kind == kind {
      Some(1)
    } else {
      None
    }
  }

  fn parse_entity(&self, tokens: &[Token]) -> Result<usize, String> {
    println!("Parsing entity");
    let mut terms;
    let mut refs = vec![];
    let mut tokens_consumed = 0;
    let mut entity_id = "".to_string();
    if let Some(t) = get_terms(&tokens[0..]) {
      tokens_consumed += t.len();
      terms = t;
    } else {
      return Ok(0);
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

    self.add_entity(AstEntity { child_entities: vec![], terms, refs, entity_id });

    return Ok(tokens_consumed);
  }
}

#[cfg(test)]
mod test {
  use crate::lexer::Lexer;
  use crate::parser::{AstEntity, Parser};

  #[test]
  fn can_parse() {
    let n = Parser::new();
    let l = Lexer::new();
    let tokens = l.lex("widget kpi @default #id".to_string()).unwrap();
    n.parse(tokens);
    assert_eq!(n.entities.borrow().len(), 1);
    assert_eq!(n.entities.borrow()[0], AstEntity {
      terms: vec!["widget".to_string(), "kpi".to_string()],
      refs: vec!["default".to_string()],
      entity_id: "id".to_string(),
      child_entities: vec![],
    });
  }

  #[test]
  fn can_parse_two() {
    let n = Parser::new();
    let l = Lexer::new();
    let tokens = l.lex("widget kpi @default #id \n widget kpi @default #id2".to_string()).unwrap();
    n.parse(tokens);
    assert_eq!(n.entities.borrow().len(), 2);
    assert_eq!(n.entities.borrow()[0], AstEntity {
      terms: vec!["widget".to_string(), "kpi".to_string()],
      refs: vec!["default".to_string()],
      entity_id: "id".to_string(),
      child_entities: vec![],
    });
    assert_eq!(n.entities.borrow()[1], AstEntity {
      terms: vec!["widget".to_string(), "kpi".to_string()],
      refs: vec!["default".to_string()],
      entity_id: "id2".to_string(),
      child_entities: vec![],
    });
  }
}

