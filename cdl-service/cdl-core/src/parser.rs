use std::cell::RefCell;

use serde_derive::{Deserialize, Serialize};

use crate::lexer::Token;
use crate::lexer::TokenType;
use crate::parser::ast_nodes::{AstEntity, AstIdentifier, AstOperator, AstProperty, AstString, EntityRef, Operator, PropertyRef, Rhs, RhsRef, AstNumber};
use crate::parser::utils::{can_start_prop, eat_eol_and_comments, eat_token_if_available, get_entity_id, get_refs, get_terms, is_tokens_left};

mod ast_nodes;
mod utils;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Parser {
  pub entities: RefCell<Vec<AstEntity>>,
  pub properties: RefCell<Vec<AstProperty>>,
  pub rhs: RefCell<Vec<Rhs>>,
  pub script_entity: EntityRef,
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
            properties.push(prop_ref);
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


  pub fn parse_property(&self, tokens: &[Token]) -> Result<(PropertyRef, usize), String> {
    if tokens[0].kind == TokenType::Identifier && tokens[1].kind == TokenType::Colon {
      let rhs = self.parse_expr(&tokens[2..])?;
//      println!("Got expr {} {}", rhs.0, rhs.1);
      match rhs {
        (_, 0) => return Ok((0, 0)),
        (index, num) => {
          let p = AstProperty {
            rhs: index,
            name: tokens[0].text.clone().unwrap_or("".to_string()),
            start_pos: tokens[0].start,
            end_pos: tokens[1 + num].end,
          };
//          println!("adding prop");
          let p_index = self.add_property(p);
          return Ok((p_index, 2 + num));
        }
      }
    } else {
      return Ok((0, 0));
    }
  }

  fn parse_expr(&self, tokens: &[Token]) -> Result<(RhsRef, usize), String> {
    let mut curr_pos = 0;
    let mut curr_rhs_index;
//    println!("Parsing expr. current token is : {:?}", tokens[0]);
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
          let expr_ref = self.add_rhs(Rhs::Operator(AstOperator {
            left: curr_rhs_index,
            right: term_index,
            op: Operator::Mul,
            start_pos: tokens[0].start,
            end_pos: tokens[curr_pos - 1].end,
          }));
          curr_rhs_index = expr_ref;
        }
        TokenType::Div => {
          let (term_index, tokens_consumed) = self.parse_term(&tokens[(curr_pos + 1)..])?;
          curr_pos += tokens_consumed + 1; //  +1 for the Div token
          let expr_ref = self.add_rhs(Rhs::Operator(AstOperator {
            left: curr_rhs_index,
            right: term_index,
            op: Operator::Del,
            start_pos: tokens[0].start,
            end_pos: tokens[curr_pos - 1].end,
          }));
          curr_rhs_index = expr_ref;
        }
        TokenType::EOL => {
          return Ok((curr_rhs_index, curr_pos));
        }
        _ => {
//          println!("returning expr {}", curr_pos);
          return Ok((curr_rhs_index, curr_pos));
        }
      }
    }
  }
  fn parse_term(&self, tokens: &[Token]) -> Result<(RhsRef, usize), String> {
    let mut curr_pos = 0;
    let mut curr_rhs_index;
//    println!("Parsing term. current token is : {:?}", tokens[0]);
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
          let term_ref = self.add_rhs(Rhs::Operator(AstOperator {
            left: curr_rhs_index,
            right: factor_index,
            op: Operator::Plus,
            start_pos: tokens[0].start,
            end_pos: tokens[curr_pos - 1].end,
          }));
          curr_rhs_index = term_ref;
        }
        TokenType::Minus => {
          let (factor_index, tokens_consumed) = self.parse_factor(&tokens[(curr_pos + 1)..])?;
          curr_pos += tokens_consumed + 1; //  +1 for the Minus token
          let term_ref = self.add_rhs(Rhs::Operator(AstOperator {
            left: curr_rhs_index,
            right: factor_index,
            op: Operator::Minus,
            start_pos: tokens[0].start,
            end_pos: tokens[curr_pos - 1].end,
          }));
          curr_rhs_index = term_ref;
        }
        TokenType::EOL => {
          return Ok((curr_rhs_index, curr_pos));
        }
        _ => {
          return Ok((curr_rhs_index, curr_pos));
          //return Err(format!("not implemented in Term pos: {}, token: {:?}", tokens[tokens_consumed].start, tokens[tokens_consumed].kind));
        }
      }
    }
  }
  fn parse_factor(&self, tokens: &[Token]) -> Result<(RhsRef, usize), String> {
    let mut curr_pos = 0;
//    println!("Parsing factor. current token is : {:?}", tokens[0]);
    let rhs;
    let curr_token = &tokens[curr_pos];
    match curr_token.kind {
      TokenType::Identifier => {
//        println!("Found ident");
        let ast_ident = AstIdentifier {
          start_pos: tokens[0].start,
          end_pos: tokens[0].end,
          value: tokens[0].text.clone().unwrap_or("".to_string()),
        };
        rhs = self.add_rhs(Rhs::Identifier(ast_ident));
        return Ok((rhs, 1));
      }
      TokenType::String => {
        let ast_string = AstString {
          start_pos: tokens[0].start,
          end_pos: tokens[0].end,
          value: tokens[0].text.clone().unwrap_or("".to_string()),
        };
        rhs = self.add_rhs(Rhs::String(ast_string));
        return Ok((rhs, 1));
      }
      TokenType::Number => {
        let ast_number = AstNumber {
          start_pos: tokens[0].start,
          end_pos: tokens[0].end,
          value :tokens[0].text.clone().unwrap_or("".to_string()).parse::<f64>().unwrap_or(0f64)

        };
        rhs = self.add_rhs(Rhs::Number(ast_number));
        return Ok((rhs, 1));
      }
      _ => return Err(format!("Unknown token when trying to parse factor: {:?} , at pos {:?}", curr_token.kind, curr_token.start)),
    }
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

  fn add_rhs(&self, r: Rhs) -> RhsRef {
    let mut rhs = self.rhs.borrow_mut();
    rhs.push(r);
    return rhs.len() - 1;
  }
}

#[cfg(test)]
mod test {
  use crate::lexer::Lexer;
  use crate::parser::{AstEntity, AstIdentifier, AstProperty, Parser, Rhs};

  #[test]
  fn can_parse() {
    let mut n = Parser::new();
    let l = Lexer::new();
    let tokens = l.lex("widget kpi @default #id {\n}\n".to_string()).unwrap();
    let _r = n.parse(tokens);
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
    assert_eq!(n.rhs.borrow()[0], Rhs::Identifier(AstIdentifier {
      value: "hello".to_string(),
      start_pos: 8,
      end_pos: 13,
    }));
    assert_eq!(n.properties.borrow().len(), 1);
    assert_eq!(n.properties.borrow()[0], AstProperty {
      name: "label".to_string(),
      rhs: 0,
      start_pos: 0,
      end_pos: 13,
    });
  }

  #[test]
  fn can_parse_two() {
    let mut n = Parser::new();
    let l = Lexer::new();
    let tokens = l.lex("widget kpi @default #id {\n} \n widget kpi @default #id2 {\n}\n".to_string()).unwrap();
    let _r = n.parse(tokens);
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
    let _r = n.parse(tokens);
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
    let _r = n.parse(tokens);
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
      end_pos: 29,
    });
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
    assert_eq!(n.entities.borrow().len(), 2);
    assert_eq!(n.entities.borrow()[0], AstEntity {
      terms: vec!["widget".to_string(), "kpi".to_string()],
      refs: vec![],
      entity_id: "".to_string(),
      child_entities: vec![],
      properties: vec![0],
      start_pos: 0,
      end_pos: 55,
    });
    assert_eq!(n.properties.borrow()[0], AstProperty {
      name: "label".to_string(),
      rhs: 8,
      start_pos: 16,
      end_pos: 52,
    });
  }


  #[test]
  fn create_script_node() {
    let mut n = Parser::new();
    let l = Lexer::new();
    let tokens = l.lex("widget kpi @default #id {\n}\n".to_string()).unwrap();
    let _r = n.parse(tokens);
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

