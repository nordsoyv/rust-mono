use crate::lexer::{Token, TokenType};

pub fn get_tokens_of_kind(tokens: &[Token], kind: TokenType) -> Option<Vec<String>> {
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

pub fn _get_token_of_kind(tokens: &[Token], kind: TokenType) -> Option<String> {
  let curr_token = &tokens[0];
  if curr_token.kind != kind {
    return None;
  }
  return Some(curr_token.text.clone().unwrap_or("".to_string()));
}

pub fn get_terms(tokens: &[Token]) -> Option<Vec<String>> {
  get_tokens_of_kind(tokens, TokenType::Identifier)
}

pub fn get_refs(tokens: &[Token]) -> Option<Vec<String>> {
  get_tokens_of_kind(tokens, TokenType::Reference)
}

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

pub fn is_tokens_left(tokens: &[Token], pos: usize) -> bool {
  tokens.len() > pos
}

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

pub fn eat_token_if_available(tokens: &[Token], kind: TokenType) -> Option<usize> {
  if tokens[0].kind == kind {
    Some(1)
  } else {
    None
  }
}


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
