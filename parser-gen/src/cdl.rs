use crate::common::{
    Parser,
    zero_or_more,
    one_or_more,
    match_literal,
    right,
    pair,
    three,
    four,
    space0,
    space1,
    quoted_string,
    identifier,
    left,
    either,
    whitespace_wrap,
    optional,
};


#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Entity {
    entity_type: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EntityHeader {
    entity_type: Vec<String>,
    entity_refs: Vec<String>,
    entity_id: String,
}

fn entity_type<'a>() -> impl Parser<'a, String> {
    left(identifier, space0())
}

fn entity_types<'a>() -> impl Parser<'a, Vec<String>> {
    one_or_more(left(identifier, space0()))
}

fn entity_ref<'a>() -> impl Parser<'a, String> {
    right(match_literal("@"), left(identifier, space0()))
}

fn entity_refs<'a>() -> impl Parser<'a, Vec<String>> {
    zero_or_more(entity_ref())
}

fn entity_id<'a>() -> impl Parser<'a, String> {
    optional(right(match_literal("#"), left(identifier, space0())), "".to_string())
}

fn open_bracket<'a>() -> impl Parser<'a, ()> {
    match_literal("{")
}

fn close_bracket<'a>() -> impl Parser<'a, ()> {
    match_literal("}")
}

fn entity_header<'a>() -> impl Parser<'a, EntityHeader> {
    four(
        entity_types(),
        entity_refs(),
        entity_id(),
        open_bracket())
        .map(move |(types, refs, id, ())| {
            EntityHeader {
                entity_id: id,
                entity_refs: refs,
                entity_type: types,
            }
        })
}


#[test]
fn entity_type_one_type() {
    assert_eq!(
        Ok(("",
            "widget".to_string())),
        entity_type().parse("widget     ")
    )
}

#[test]
fn entity_type_two_type() {
    assert_eq!(
        Ok(("", vec![
            "widget".to_string(), "kpi".to_string()])),
        entity_types().parse("widget kpi")
    )
}

#[test]
fn match_open_bracket() {
    assert_eq!(
        Ok(("", ())),
        open_bracket().parse("{")
    )
}

#[test]
fn err_open_bracket() {
    assert_eq!(
        Err("<"),
        open_bracket().parse("<")
    )
}

#[test]
fn match_close_bracket() {
    assert_eq!(
        Ok(("", ())),
        close_bracket().parse("}")
    )
}

#[test]
fn match_entity_ref() {
    assert_eq!(
        Ok(("",
            "ref".to_string())),
        entity_ref().parse("@ref")
    )
}

#[test]
fn match_entity_refs() {
    assert_eq!(
        Ok(("", vec!["ref".to_string(), "ref2".to_string()]
        )),
        entity_refs().parse("@ref @ref2")
    )
}

#[test]
fn match_entity_id() {
    assert_eq!(
        Ok(("",
            "id".to_string())),
        entity_id().parse("#id")
    )
}

#[test]
fn match_entity_header() {
    assert_eq!(
        Ok(("",
            EntityHeader {
                entity_type: vec!["widget".to_string(), "kpi".to_string()],
                entity_refs: vec!["default".to_string()],
                entity_id: "id".to_string(),
            })),
        entity_header().parse("widget kpi @default #id {")
    );
    assert_eq!(
        Ok(("",
            EntityHeader {
                entity_type: vec!["widget".to_string(), "kpi".to_string()],
                entity_refs: vec!["default".to_string(), "other".to_string()],
                entity_id: "id".to_string(),
            })),
        entity_header().parse("widget kpi @default @other #id {")
    );
    assert_eq!(
        Ok(("",
            EntityHeader {
                entity_type: vec!["widget".to_string(), "kpi".to_string()],
                entity_refs: vec![],
                entity_id: "id".to_string(),
            })),
        entity_header().parse("widget kpi #id {")
    );
    assert_eq!(
        Ok(("",
            EntityHeader {
                entity_type: vec!["widget".to_string(), "kpi".to_string()],
                entity_refs: vec![],
                entity_id: "".to_string(),
            })),
        entity_header().parse("widget kpi {")
    );
    assert_eq!(
        Ok(("",
            EntityHeader {
                entity_type: vec!["widget".to_string()],
                entity_refs: vec![],
                entity_id: "".to_string(),
            })),
        entity_header().parse("widget  {")
    );
}


//#[test]
//fn attribute_parser() {
//    assert_eq!(
//        Ok((
//            "",
//            vec![
//                ("one".to_string(), "1".to_string()),
//                ("two".to_string(), "2".to_string())
//            ]
//        )),
//        attributes().parse(" one=\"1\" two=\"2\"")
//    );
//}
