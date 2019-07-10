use std::cell::RefCell;

use serde_derive::{Deserialize, Serialize};

use crate::lexer::Token;
use crate::lexer::TokenType;
use crate::parser::ast_nodes::{
  AstEntity,
  AstProperty,
  AstIdentifier,
  AstNumber,
  AstOperator,
  AstString,
  AstUnaryOp,
  Operator,
  NodeRef};
use crate::parser::utils::{
  can_start_prop,
  eat_eol_and_comments,
  eat_token_if_available,
  get_entity_id,
  get_refs,
  get_terms,
  is_next_token,
  is_tokens_left};

mod ast_nodes;
mod utils;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Parser {
  pub entities: RefCell<Vec<AstEntity>>,
  pub properties: RefCell<Vec<AstProperty>>,
  pub identifiers: RefCell<Vec<AstIdentifier>>,
  pub numbers: RefCell<Vec<AstNumber>>,
  pub operators: RefCell<Vec<AstOperator>>,
  pub strings: RefCell<Vec<AstString>>,
  pub unary_ops: RefCell<Vec<AstUnaryOp>>,
  pub script_entity: NodeRef,
}

impl Parser {
  pub fn new() -> Parser {
    Parser {
      entities: RefCell::new(Vec::new()),
      properties: RefCell::new(Vec::new()),
      identifiers: RefCell::new(Vec::new()),
      numbers: RefCell::new(Vec::new()),
      operators: RefCell::new(Vec::new()),
      strings: RefCell::new(Vec::new()),
      unary_ops: RefCell::new(Vec::new()),
      script_entity: NodeRef::None,
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
      parent: NodeRef::None,
      children: entity_refs,
      terms: vec![],
      refs: vec![],
      entity_id: "".to_string(),
      start_pos: 0,
      end_pos: tokens[tokens.len() - 1].end,
    });
    self.script_entity = script_entity_id;
    return Ok(());
  }

  fn parse_entity(&self, tokens: &[Token]) -> Result<(NodeRef, usize), String> {
    let terms;
    let mut refs = vec![];
    let mut children = vec![];
    let mut tokens_consumed = 0;
    let mut entity_id = "".to_string();
    let start_pos = tokens[0].start;
    if let Some(t) = get_terms(&tokens[0..]) {
      tokens_consumed += t.len();
      terms = t;
    } else {
      return Ok((NodeRef::None, 0));
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
      return Ok((NodeRef::None, 0));
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
    if let Some(num) = eat_token_if_available(&tokens[tokens_consumed..], TokenType::CloseBracket) {
      tokens_consumed += num;
    } else {
      return Ok((NodeRef::None, 0));
    }

    if let Some(num) = eat_token_if_available(&tokens[tokens_consumed..], TokenType::EOL) {
      tokens_consumed += num;
    } else {
      return Ok((NodeRef::None, 0));
    }

    let end_pos = tokens[tokens_consumed - 1].end;
    let entity_ref = self.add_entity(AstEntity {
      parent: NodeRef::None,
      children,
      terms,
      refs,
      entity_id,
      start_pos,
      end_pos,
    });

    return Ok((entity_ref, tokens_consumed));
  }

  pub fn parse_property(&self, tokens: &[Token]) -> Result<(NodeRef, usize), String> {
    if tokens[0].kind == TokenType::Identifier && tokens[1].kind == TokenType::Colon {
      let rhs = self.parse_expr(&tokens[2..])?;
      match rhs {
        (_, 0) => return Ok((NodeRef::None, 0)),
        (node_ref, num) => {
          let p = AstProperty {
            parent: NodeRef::None,
            rhs: node_ref,
            name: tokens[0].text.clone().unwrap_or("".to_string()),
            start_pos: tokens[0].start,
            end_pos: tokens[1 + num].end,
          };
          let prop_ref = self.add_property(p);
          // self.set_parent_rhs(&node_ref, &prop_ref);
          return Ok((prop_ref, 2 + num));
        }
      }
    } else {
      return Ok((NodeRef::None, 0));
    }
  }

  fn parse_expr(&self, tokens: &[Token]) -> Result<(NodeRef, usize), String> {
    let mut curr_pos = 0;
    let mut curr_node_ref;
    let (term_node_ref, tokens_consumed) = self.parse_term(tokens)?;
    if tokens_consumed == 0 {
      return Err(format!("Error when parsing expression at pos {}, token: {:?}", tokens[0].start, tokens[0].kind));
    }
    curr_node_ref = term_node_ref;
    curr_pos += tokens_consumed;
    loop {
      match tokens[curr_pos].kind {
        TokenType::Mul => {
          let (term_node_ref, tokens_consumed) = self.parse_term(&tokens[(curr_pos + 1)..])?;
          curr_pos += tokens_consumed + 1; //  +1 for the Mul token
          let node_expr_ref = self.add_operator(AstOperator {
            parent: NodeRef::None,
            left: curr_node_ref,
            right: term_node_ref,
            op: Operator::Mul,
            start_pos: tokens[0].start,
            end_pos: tokens[curr_pos - 1].end,
          });
          curr_node_ref = node_expr_ref;
        }
        TokenType::Div => {
          let (term_node_ref, tokens_consumed) = self.parse_term(&tokens[(curr_pos + 1)..])?;
          curr_pos += tokens_consumed + 1; //  +1 for the Div token
          let expr_ref = self.add_operator(AstOperator {
            parent: NodeRef::None,
            left: curr_node_ref,
            right: term_node_ref,
            op: Operator::Del,
            start_pos: tokens[0].start,
            end_pos: tokens[curr_pos - 1].end,
          });
          curr_node_ref = expr_ref;
        }
        TokenType::EOL => {
          return Ok((curr_node_ref, curr_pos));
        }
        _ => {
          return Ok((curr_node_ref, curr_pos));
        }
      }
    }
  }

  fn parse_term(&self, tokens: &[Token]) -> Result<(NodeRef, usize), String> {
    let mut curr_pos = 0;
    let mut curr_node_ref;
    let (left_node_ref, tokens_consumed) = self.parse_factor(tokens)?;
    if tokens_consumed == 0 {
      return Err(format!("Error when parsing term at pos {}, token: {:?}", tokens[0].start, tokens[0].kind));
    }
    curr_node_ref = left_node_ref;
    curr_pos += tokens_consumed;
    loop {
      match tokens[curr_pos].kind {
        TokenType::Plus => {
          let (factor_node_ref, tokens_consumed) = self.parse_factor(&tokens[(curr_pos + 1)..])?;
          curr_pos += tokens_consumed + 1; //  +1 for the Plus token
          let term_ref = self.add_operator(AstOperator {
            parent: NodeRef::None,
            left: curr_node_ref,
            right: factor_node_ref,
            op: Operator::Plus,
            start_pos: tokens[0].start,
            end_pos: tokens[curr_pos - 1].end,
          });
          curr_node_ref = term_ref;
        }
        TokenType::Minus => {
          let (factor_node_ref, tokens_consumed) = self.parse_factor(&tokens[(curr_pos + 1)..])?;
          curr_pos += tokens_consumed + 1; //  +1 for the Minus token
          let term_ref = self.add_operator(AstOperator {
            parent: NodeRef::None,
            left: curr_node_ref,
            right: factor_node_ref,
            op: Operator::Minus,
            start_pos: tokens[0].start,
            end_pos: tokens[curr_pos - 1].end,
          });
          curr_node_ref = term_ref;
        }
        TokenType::EOL => {
          return Ok((curr_node_ref, curr_pos));
        }
        _ => {
          return Ok((curr_node_ref, curr_pos));
        }
      }
    }
  }

  fn parse_factor(&self, tokens: &[Token]) -> Result<(NodeRef, usize), String> {
    let mut curr_pos = 0;
    let node_ref;
    let curr_token = &tokens[curr_pos];
    match curr_token.kind {
      TokenType::Identifier => {
        let ast_ident = AstIdentifier {
          parent: NodeRef::None,
          start_pos: tokens[0].start,
          end_pos: tokens[0].end,
          value: tokens[0].text.clone().unwrap_or("".to_string()),
        };
        node_ref = self.add_identifier(ast_ident);
        return Ok((node_ref, 1));
      }
      TokenType::String => {
        let ast_string = AstString {
          parent: NodeRef::None,
          start_pos: tokens[0].start,
          end_pos: tokens[0].end,
          value: tokens[0].text.clone().unwrap_or("".to_string()),
        };
        node_ref = self.add_string(ast_string);
        return Ok((node_ref, 1));
      }
      TokenType::Number => {
        let ast_number = AstNumber {
          parent: NodeRef::None,
          start_pos: tokens[0].start,
          end_pos: tokens[0].end,
          value: tokens[0].text.clone().unwrap_or("".to_string()).parse::<f64>().unwrap_or(0f64),
        };

        node_ref = self.add_number(ast_number);
        return Ok((node_ref, 1));
      }
      TokenType::OpenParen => {
        let expr = self.parse_expr(&tokens[1..])?;
        match expr {
          (_, 0) => return Err(format!("Error parsing factor after '(', token : {:?}, pos {}", curr_token.kind, curr_token.start)),
          (expr_node_ref, tokens_consumed) => {
            if is_next_token(&tokens[1 + tokens_consumed..], TokenType::CloseParen) {
              return Ok((expr_node_ref, 1 + tokens_consumed + 1));
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
          (expr_node_ref, tokens_consumed) => {
            let op_index = self.add_unary_op(AstUnaryOp {
              parent: NodeRef::None,
              right: expr_node_ref,
              op: Operator::Minus,
              start_pos: curr_token.start,
              end_pos: tokens[tokens_consumed + 1].end,
            });
            return Ok((op_index, tokens_consumed + 1)); // plus 1 for the minus token
          }
        }
      }
      _ => return Err(format!("Unknown token when trying to parse factor: {:?} , at pos {:?}", curr_token.kind, curr_token.start)),
    }
  }

  fn add_entity(&self, e: AstEntity) -> NodeRef {
    let mut ents = self.entities.borrow_mut();
    ents.push(e);
    return NodeRef::Entity(ents.len() - 1);
  }
  fn add_property(&self, p: AstProperty) -> NodeRef {
    let mut props = self.properties.borrow_mut();
    props.push(p);
    return NodeRef::Property(props.len() - 1);
  }
  fn add_identifier(&self, i: AstIdentifier) -> NodeRef {
    let mut idents = self.identifiers.borrow_mut();
    idents.push(i);
    return NodeRef::Identifier(idents.len() - 1);
  }
  fn add_number(&self, n: AstNumber) -> NodeRef {
    let mut numbers = self.numbers.borrow_mut();
    numbers.push(n);
    return NodeRef::Number(numbers.len() - 1);
  }
  fn add_operator(&self, n: AstOperator) -> NodeRef {
    let mut operators = self.operators.borrow_mut();
    operators.push(n);
    return NodeRef::Operator(operators.len() - 1);
  }
  fn add_string(&self, s: AstString) -> NodeRef {
    let mut strings = self.strings.borrow_mut();
    strings.push(s);
    return NodeRef::String(strings.len() - 1);
  }
  fn add_unary_op(&self, u: AstUnaryOp) -> NodeRef {
    let mut unary = self.unary_ops.borrow_mut();
    unary.push(u);
    return NodeRef::UnaryOperator(unary.len() - 1);
  }

  fn set_parent(&self, node_to_change: NodeRef, new_parent: NodeRef) {
    match node_to_change {
      NodeRef::Identifier(index) => {
        let new_i = {
          let i = &self.identifiers.borrow()[index];
          AstIdentifier {
            end_pos: i.end_pos,
            start_pos: i.start_pos,
            value: i.value.clone(),
            parent: new_parent.clone(),
          }
        };
        {
          let mut ident_vec = self.identifiers.borrow_mut();
          ident_vec[index] = new_i;
        }
      }
      NodeRef::String(index) => {
        let new_s = {
          let s = &self.strings.borrow()[index];
          AstString {
            end_pos: s.end_pos,
            start_pos: s.start_pos,
            value: s.value.clone(),
            parent: new_parent.clone(),
          }
        };
        {
          let mut string_vec = self.strings.borrow_mut();
          string_vec[index] = new_s;
        }
      }
      NodeRef::Property(index) => {
        let new_p = {
          let p = &self.properties.borrow()[index];
          AstProperty {
            rhs: p.rhs.clone(),
            name : p.name.clone(),
            end_pos: p.end_pos,
            start_pos: p.start_pos,
            parent: new_parent.clone(),
          }
        };
        {
          let mut prop_vec = self.properties.borrow_mut();
          prop_vec[index] = new_p;
        }
      }

      _ => {}
    }
  }
//  fn set_parent_rhs(&self, node_ref: &NodeRef, new_parent: &NodeRef) {
//    let new_rhs = {
//      let r = &self.rhs.borrow()[rhs_index];
//      match r {
//        Rhs::Operator(o) => {
//          let new_op = AstOperator {
//            end_pos: o.end_pos,
//            start_pos: o.start_pos,
//            op: o.op.clone(),
//            right: o.right,
//            left: o.left,
//            parent: new_parent.clone(),
//          };
//          Rhs::Operator(new_op)
//        }
//        Rhs::UnaryOp(o) => {
//          let new_op = AstUnaryOp {
//            end_pos: o.end_pos,
//            start_pos: o.start_pos,
//            op: o.op.clone(),
//            right: o.right,
//            parent: new_parent.clone(),
//          };
//          Rhs::UnaryOp(new_op)
//        }
//        Rhs::String(o) => {
//          let new_op = AstString {
//            end_pos: o.end_pos,
//            start_pos: o.start_pos,
//            value: o.value.clone(),
//            parent: new_parent.clone(),
//          };
//          Rhs::String(new_op)
//        }
//        Rhs::Identifier(o) => {
//          let new_op = AstIdentifier {
//            end_pos: o.end_pos,
//            start_pos: o.start_pos,
//            value: o.value.clone(),
//            parent: new_parent.clone(),
//          };
//          Rhs::Identifier(new_op)
//        }
//        Rhs::Number(o) => {
//          let new_op = AstNumber {
//            end_pos: o.end_pos,
//            start_pos: o.start_pos,
//            value: o.value,
//            parent: new_parent.clone(),
//          };
//          Rhs::Number(new_op)
//        }
//      }
//    };
//    {
//      let mut rhs_vec = self.rhs.borrow_mut();
//      rhs_vec[rhs_index] = new_rhs;
//    }
//  }
}

#[cfg(test)]
mod test {
  use crate::lexer::Lexer;
  use crate::parser::{AstEntity, AstIdentifier, AstProperty, Parser};
  use crate::parser::ast_nodes::NodeRef;

  #[test]
  fn can_parse() {
    let mut n = Parser::new();
    let l = Lexer::new();
    let tokens = l.lex("widget kpi @default #id {\n}\n".to_string()).unwrap();
    let _r = n.parse(tokens);
    assert_eq!(n.entities.borrow().len(), 2);
    assert_eq!(n.entities.borrow()[0], AstEntity {
      parent: NodeRef::None,
      terms: vec!["widget".to_string(), "kpi".to_string()],
      refs: vec!["default".to_string()],
      entity_id: "id".to_string(),
      children: vec![],
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
    assert_eq!(n.identifiers.borrow().len(), 1);
    assert_eq!(n.identifiers.borrow()[0], AstIdentifier {
      parent: NodeRef::None,
      value: "hello".to_string(),
      start_pos: 8,
      end_pos: 13,
    });
    assert_eq!(n.properties.borrow().len(), 1);
    assert_eq!(n.properties.borrow()[0], AstProperty {
      parent: NodeRef::None,
      name: "label".to_string(),
      rhs: NodeRef::Identifier(0),
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
      parent: NodeRef::None,
      terms: vec!["widget".to_string(), "kpi".to_string()],
      refs: vec!["default".to_string()],
      entity_id: "id".to_string(),
      children: vec![],
      start_pos: 0,
      end_pos: 29,
    });
    assert_eq!(n.entities.borrow()[1], AstEntity {
      parent: NodeRef::None,
      terms: vec!["widget".to_string(), "kpi".to_string()],
      refs: vec!["default".to_string()],
      entity_id: "id2".to_string(),
      children: vec![],
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
      parent: NodeRef::None,
      terms: vec!["widget".to_string(), "list".to_string()],
      refs: vec![],
      entity_id: "".to_string(),
      children: vec![],
      start_pos: 30,
      end_pos: 50,
    });
    assert_eq!(n.entities.borrow()[2], AstEntity {
      parent: NodeRef::None,
      terms: vec!["widget".to_string(), "kpi".to_string()],
      refs: vec!["default".to_string()],
      entity_id: "id".to_string(),
      children: vec![NodeRef::Entity(0), NodeRef::Entity(1)],
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
      parent: NodeRef::None,
      terms: vec!["widget".to_string(), "kpi".to_string()],
      refs: vec![],
      entity_id: "".to_string(),
      children: vec![NodeRef::Property(0)],
      start_pos: 0,
      end_pos: 32,
    });
    assert_eq!(n.properties.borrow()[0], AstProperty {
      parent: NodeRef::None,
      name: "label".to_string(),
      rhs: NodeRef::Identifier(0),
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
      parent: NodeRef::None,
      terms: vec!["widget".to_string(), "kpi".to_string()],
      refs: vec![],
      entity_id: "".to_string(),
      children: vec![NodeRef::Property(0)],
      start_pos: 0,
      end_pos: 55,
    });
    assert_eq!(n.properties.borrow()[0], AstProperty {
      parent: NodeRef::None,
      name: "label".to_string(),
      rhs: NodeRef::Operator(3),
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
      parent: NodeRef::None,
      terms: vec!["widget".to_string(), "kpi".to_string()],
      refs: vec!["default".to_string()],
      entity_id: "id".to_string(),
      children: vec![],
      start_pos: 0,
      end_pos: 28,
    });
    assert_eq!(n.entities.borrow()[1], AstEntity {
      parent: NodeRef::None,
      terms: vec![],
      refs: vec![],
      entity_id: "".to_string(),
      children: vec![NodeRef::Entity(0)],
      start_pos: 0,
      end_pos: 28,
    });
  }
}

