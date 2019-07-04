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
  pub start_pos: usize,
  pub end_pos: usize,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Parser {
  pub entities: RefCell<Vec<AstEntity>>,
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

fn eat_token_if_available(tokens: &[Token], kind: TokenType) -> Option<usize> {
  if tokens[0].kind == kind {
    Some(1)
  } else {
    None
  }
}

impl Parser {
  pub fn new() -> Parser {
    Parser {
      entities: RefCell::new(Vec::new()),
      script_entity: 0,
    }
  }

  pub fn parse(&mut self, tokens: Vec<Token>) {
    let mut curr_pos = 0;
    let mut entity_refs = vec![];
    while is_tokens_left(&tokens, curr_pos) {
      if let Some(num) = eat_token_if_available(&tokens[curr_pos..], TokenType::EOL) {
        curr_pos += num;
        continue;
      }
      let res = self.parse_entity(&tokens[curr_pos..]);
      match res {
        Ok((0, _)) => println!("No mathc"),
        Ok((num, entity_ref)) => {
          curr_pos += num;
          entity_refs.push(entity_ref);
        }

        Err(e) => println!("{}", e)
      }
    }

    let script_entity_id = self.add_entity(AstEntity {
      child_entities: entity_refs,
      terms: vec![],
      refs: vec![],
      entity_id: "".to_string(),
      start_pos: 0,
      end_pos: tokens[tokens.len()-1].end,
    });
    self.script_entity = script_entity_id;
  }

  fn add_entity(&self, e: AstEntity) -> EntityRef {
    let mut ents = self.entities.borrow_mut();
    ents.push(e);
    return ents.len() - 1;
  }


  fn parse_entity(&self, tokens: &[Token]) -> Result<(usize, EntityRef), String> {
    let mut terms;
    let mut refs = vec![];
    let mut children = vec![];
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

    if let Some(num) = eat_token_if_available(&tokens[tokens_consumed..], TokenType::EOL) {
      tokens_consumed += num;
    } else {
      return Ok((0, 0));
    }

    loop {
      match self.parse_entity(&tokens[tokens_consumed..]) {
        Ok((0, _)) => break,
        Ok((consumed, child_ref)) => {
          tokens_consumed += consumed;
          children.push(child_ref);
        }
        Err(e) => return Err(e),
      }
    }

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
    let entity_ref = self.add_entity(AstEntity { child_entities: children, terms, refs, entity_id, start_pos, end_pos });

    return Ok((tokens_consumed, entity_ref));
  }
}

#[cfg(test)]
mod test {
  use crate::lexer::Lexer;
  use crate::parser::{AstEntity, Parser};

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
      start_pos: 0,
      end_pos: 28,
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
      start_pos: 0,
      end_pos: 29,
    });
    assert_eq!(n.entities.borrow()[1], AstEntity {
      terms: vec!["widget".to_string(), "kpi".to_string()],
      refs: vec!["default".to_string()],
      entity_id: "id2".to_string(),
      child_entities: vec![],
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
      start_pos: 30,
      end_pos: 50,
    });
    assert_eq!(n.entities.borrow()[2], AstEntity {
      terms: vec!["widget".to_string(), "kpi".to_string()],
      refs: vec!["default".to_string()],
      entity_id: "id".to_string(),
      child_entities: vec![0, 1],
      start_pos: 0,
      end_pos: 76,
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
      start_pos: 0,
      end_pos: 28,
    });
    assert_eq!(n.entities.borrow()[1], AstEntity {
      terms: vec![],
      refs: vec![],
      entity_id: "".to_string(),
      child_entities: vec![0],
      start_pos: 0,
      end_pos: 28,
    });
  }


}

