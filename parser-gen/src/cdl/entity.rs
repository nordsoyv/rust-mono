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
    header: EntityHeader,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EntityBody {
    children: Vec<Entity>
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EntityHeader {
    entity_type: Vec<String>,
    entity_refs: Vec<String>,
    entity_id: String,
}

fn entity_type<'a>() -> impl Parser<'a, String> {
    whitespace_wrap(identifier)
}

fn entity_types<'a>() -> impl Parser<'a, Vec<String>> {
    one_or_more(entity_type())
}

fn entity_ref<'a>() -> impl Parser<'a, String> {
    whitespace_wrap(right(match_literal("@"), identifier))
}

fn entity_refs<'a>() -> impl Parser<'a, Vec<String>> {
    zero_or_more(entity_ref())
}

fn entity_id<'a>() -> impl Parser<'a, String> {
    whitespace_wrap(right(match_literal("#"), identifier))
}

fn entity_identifier<'a>() -> impl Parser<'a, String> {
    optional(entity_id(), "".to_string())
}

fn open_bracket<'a>() -> impl Parser<'a, ()> {
    whitespace_wrap(match_literal("{"))
}

fn close_bracket<'a>() -> impl Parser<'a, ()> {
    whitespace_wrap(match_literal("}"))
}

fn entity_header<'a>() -> impl Parser<'a, EntityHeader> {
    three(
        entity_types(),
        entity_refs(),
        entity_identifier(),
    )
        .map(move |(types, refs, id)| {
            EntityHeader {
                entity_id: id,
                entity_refs: refs,
                entity_type: types,
            }
        })
}

fn entity_body<'a>() -> impl Parser<'a, EntityBody> {
    zero_or_more(entity()).map(move |children| {
        EntityBody {
            children
        }
    })
}

fn entity<'a>() -> impl Parser<'a, Entity> {
    three(entity_header(), open_bracket(), close_bracket())
        .map(move |(header, _, _)| {
            Entity {
                header
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
        entity_header().parse("widget kpi @default #id")
    );
    assert_eq!(
        Ok(("",
            EntityHeader {
                entity_type: vec!["widget".to_string(), "kpi".to_string()],
                entity_refs: vec!["default".to_string(), "other".to_string()],
                entity_id: "id".to_string(),
            })),
        entity_header().parse("widget kpi @default @other #id")
    );
    assert_eq!(
        Ok(("",
            EntityHeader {
                entity_type: vec!["widget".to_string(), "kpi".to_string()],
                entity_refs: vec![],
                entity_id: "id".to_string(),
            })),
        entity_header().parse("widget kpi #id")
    );
    assert_eq!(
        Ok(("",
            EntityHeader {
                entity_type: vec!["widget".to_string(), "kpi".to_string()],
                entity_refs: vec![],
                entity_id: "".to_string(),
            })),
        entity_header().parse("widget kpi")
    );
    assert_eq!(
        Ok(("",
            EntityHeader {
                entity_type: vec!["widget".to_string()],
                entity_refs: vec![],
                entity_id: "".to_string(),
            })),
        entity_header().parse("widget ")
    );
}

#[test]
fn match_entity() {
    assert_eq!(
        Ok(("",
            Entity {
                header: EntityHeader {
                    entity_type: vec!["widget".to_string(), "kpi".to_string()],
                    entity_refs: vec!["default".to_string()],
                    entity_id: "id".to_string(),
                }
            }
        )),
        entity().parse("widget kpi @default #id {}")
    );
    assert_eq!(
        Ok(("",
            Entity {
                header: EntityHeader {
                    entity_type: vec!["widget".to_string(), "kpi".to_string()],
                    entity_refs: vec!["default".to_string()],
                    entity_id: "id".to_string(),
                }
            }
        )),
        entity().parse("widget kpi @default #id { } ")
    );
}


#[test]
fn match_entity_body() {
    assert_eq!(
        Ok(("",
            EntityBody {
                children: vec![
                    Entity {
                        header: EntityHeader {
                            entity_type: vec!["widget".to_string(), "kpi".to_string()],
                            entity_refs: vec![],
                            entity_id: "".to_string(),
                        }
                    }
                    ,
                    Entity {
                        header: EntityHeader {
                            entity_type: vec!["widget".to_string(), "test".to_string()],
                            entity_refs: vec![],
                            entity_id: "".to_string(),
                        }
                    }
                ]
            }
        )),
        entity_body().parse("widget kpi {}\
        widget test {}")
    );
//    assert_eq!(
//        Ok(("",
//            Entity {
//                header: EntityHeader {
//                    entity_type: vec!["widget".to_string(), "kpi".to_string()],
//                    entity_refs: vec!["default".to_string()],
//                    entity_id: "id".to_string(),
//                }
//            }
//        )),
//        entity().parse("widget kpi @default #id { } ")
//    );
}



