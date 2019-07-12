use crate::lexer::{Token, TokenType};

#[inline]
pub fn get_tokens_of_kind(tokens: &[Token], kind: TokenType) -> Vec<String> {
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

  terms
}

#[inline]
pub fn get_token_of_kind(tokens: &[Token], kind: TokenType) -> Option<String> {
  let curr_token = &tokens[0];
  if curr_token.kind != kind {
    return None;
  }
  return Some(curr_token.text.clone().unwrap_or("".to_string()));
}

#[inline]
pub fn get_terms(tokens: &[Token]) -> Vec<String> {
  get_tokens_of_kind(tokens, TokenType::Identifier)
}

#[inline]
pub fn get_refs(tokens: &[Token]) -> Vec<String> {
  get_tokens_of_kind(tokens, TokenType::Reference)
}

#[inline]
pub fn get_string(tokens: &[Token]) -> Vec<String> {
  get_tokens_of_kind(tokens, TokenType::String)
}


#[inline]
pub fn get_entity_id(tokens: &[Token]) -> Option<(String, usize)> {
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

#[inline]
pub fn is_tokens_left(tokens: &[Token], pos: usize) -> bool {
  tokens.len() > pos
}

#[inline]
pub fn _eat_tokens_if_available(tokens: &[Token], kind: TokenType) -> usize {
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

#[inline]
pub fn eat_token_if_available(tokens: &[Token], kind: TokenType) -> Option<usize> {
  if tokens[0].kind == kind {
    Some(1)
  } else {
    None
  }
}

#[inline]
pub fn is_next_token(tokens: &[Token], kind: TokenType) -> bool {
  tokens[0].kind == kind
}

#[inline]
pub fn eat_eol_and_comments(tokens: &[Token]) -> usize {
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

#[inline]
pub fn _eat_eol(tokens: &[Token]) -> usize {
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

#[inline]
pub fn can_start_prop(tokens: &[Token]) -> bool {
  tokens[0].kind == TokenType::Identifier && tokens[1].kind == TokenType::Colon
}


#[inline]
pub fn is_config_hub_entity(terms: &Vec<String>) -> bool {
  if terms.len() == 2 && terms[0] == "config" && terms[1] == "hub" {
    return true;
  }
  return false;
}

pub fn parse_number_token(token: &Token) -> f64 {
  if token.kind == TokenType::Number {
    return token.text.clone().unwrap_or("".to_string()).parse::<f64>().unwrap_or(0f64);
  }
  0.0
}

pub struct EntityHeader {
  pub terms: Vec<String>,
  pub refs: Vec<String>,
  pub entity_id: f64,
  pub identifier: String,
  pub size: usize,
  pub label: String,

}


pub fn parse_entity_header(tokens: &[Token]) -> Result<EntityHeader, String> {
  let terms;
  let mut refs = vec![];
  let mut label = String::new();
  let mut tokens_consumed = 0;
  let mut entity_id = 0.0;
  let mut identifier = "".to_string();

  terms = get_terms(&tokens[0..]);
  if terms.len() == 0 { // no terms, something is wrong
    return Err(format!("Error parsing entity header, at token {:?}", tokens[0]));
  }
  tokens_consumed += terms.len();

  if is_tokens_left(tokens, tokens_consumed) {
    let s = get_string(&tokens[tokens_consumed..]);
    if s.len() > 0 {
      label = s[0].clone();
      tokens_consumed += 1;
    }
  }

  if is_tokens_left(tokens, tokens_consumed) {
    refs = get_refs(&tokens[tokens_consumed..]);
    tokens_consumed += refs.len();
  }


  if is_tokens_left(tokens, tokens_consumed) {
    if let Some((parsed_entity_id, tokens_used)) = get_entity_id(&tokens[tokens_consumed..]) {
      tokens_consumed += tokens_used;
      identifier = parsed_entity_id;
    }
  }

  if is_next_token(&tokens[tokens_consumed..], TokenType::Number) {
    entity_id = parse_number_token(&tokens[tokens_consumed]);
    tokens_consumed +=1;
  }

  return Ok(EntityHeader {
    terms,
    refs,
    label,
    entity_id,
    identifier,
    size: tokens_consumed,
  });
}

