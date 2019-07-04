use serde_derive::{Deserialize, Serialize};
use crate::lexer::Token;
use crate::lexer::TokenType;


type EntityRef = usize;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct AstEntity {
  pub terms: Vec<String>,
  pub refs: Vec<String>,
  pub child_entities: Vec<EntityRef>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct NodeManager {
  pub entities: Vec<AstEntity>,
  pub script: EntityRef,
  tokens: Vec<Token>,
  curr_pos: usize,
}

fn get_tokens_of_kind(tokens : &[Token], kind: TokenType) -> Option<Vec<String>>{
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

fn get_terms(tokens : &[Token]) -> Option<Vec<String>> {
  get_tokens_of_kind(tokens, TokenType::Identifier)
}

fn get_refs(tokens : &[Token]) -> Option<Vec< String>> {
  get_tokens_of_kind(tokens, TokenType::Reference)
}

impl NodeManager {
  pub fn new() -> NodeManager {
    NodeManager {
      entities: vec![],
      script: 0,
      tokens: vec![],
      curr_pos: 0,
    }
  }

  pub fn parse(&mut self, tokens: Vec<Token>) {
    self.tokens = tokens;
    let res = self.parse_entity();
    match res {
      Ok(0) => println!("No mathc"),
      Ok(num) => {
        println!("Found match {}", num);
        self.curr_pos = self.curr_pos + num;
      }

      Err(e) => println!("{}", e)
    }
  }

  fn parse_entity(&mut self) -> Result<usize, String> {
    let mut terms;
    let mut refs = vec![];
    let mut tokens_consumed = 0;
    match get_terms(&self.tokens[self.curr_pos..]) {
      Some(t) => {
        tokens_consumed += t.len();
        terms = t;

      }
      None => return Ok(0)
    }
    match get_refs(&self.tokens[(self.curr_pos + tokens_consumed)..] ) {
      Some(r) => {
        tokens_consumed += r.len();
        refs = r;
      }
      None => {}
    }

    self.entities.push(AstEntity { child_entities: vec![], terms, refs });

    return Ok(tokens_consumed);
  }
}

#[cfg(test)]
mod test {
  use crate::lexer::Lexer;
  use crate::parser::{NodeManager, AstEntity};

  #[test]
  fn can_parse() {
    let mut n = NodeManager::new();
    let l = Lexer::new();
    let tokens = l.lex("widget kpi @default".to_string()).unwrap();
    n.parse(tokens);
    assert_eq!(n.entities.len(), 1);
    assert_eq!(n.entities[0], AstEntity{
      terms: vec!["widget".to_string(),"kpi".to_string()],
      refs: vec!["default".to_string()],
      child_entities: vec![]
    });
  }
}

