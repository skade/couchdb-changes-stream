use serde;
use serde::de::Deserialize;
use serde_json as json;
use super::revision::Revision;

#[derive(Debug)]
pub struct Change {
    seq: i64,
    id: String,
    changes: Vec<Revision>,
    doc: Option<json::Value>,
}

impl Deserialize for Change {
    fn deserialize<D>(deserializer: &mut D) -> Result<Change, D::Error>
        where D: serde::Deserializer,
    {
        deserializer.visit(ChangeVisitor)
    }
}

enum ChangeField {
    Seq,
    Id,
    Changes,
    Doc
}

impl serde::Deserialize for ChangeField {
    fn deserialize<D>(deserializer: &mut D) -> Result<ChangeField, D::Error>
        where D: serde::de::Deserializer
    {
        struct ChangeFieldVisitor;

        impl serde::de::Visitor for ChangeFieldVisitor {
            type Value = ChangeField;

            fn visit_str<E>(&mut self, value: &str) -> Result<ChangeField, E>
                where E: serde::de::Error
            {
                match value {
                    "seq" => Ok(ChangeField::Seq),
                    "id" => Ok(ChangeField::Id),
                    "changes" => Ok(ChangeField::Changes),
                    "doc" => Ok(ChangeField::Doc),
                    _ => Err(serde::de::Error::syntax("expected seq, id or changes field")),
                }
            }
        }

        deserializer.visit(ChangeFieldVisitor)
    }
}

pub struct ChangeVisitor;

impl serde::de::Visitor for ChangeVisitor {
    type Value = Change;

    fn visit_map<V>(&mut self, mut visitor: V) -> Result<Change, V::Error>
        where V: serde::de::MapVisitor
    {
        let mut seq = None;
        let mut id = None;
        let mut changes = None;
        let mut doc = None;

        loop {
            match try!(visitor.visit_key()) {
                Some(ChangeField::Seq) => { seq = Some(try!(visitor.visit_value())); }
                Some(ChangeField::Id) => { id = Some(try!(visitor.visit_value())); }
                Some(ChangeField::Changes) => { changes = Some(try!(visitor.visit_value())); }
                Some(ChangeField::Doc) => { doc = Some(try!(visitor.visit_value())); }
                None => { break; }
            }
        }

        let seq = match seq {
            Some(seq) => seq,
            None => try!(visitor.missing_field("seq")),
        };

        let id = match id {
            Some(id) => id,
            None => try!(visitor.missing_field("id")),
        };

        let changes = match changes {
            Some(changes) => changes,
            None => try!(visitor.missing_field("changes")),
        };

        let doc = match doc {
            Some(doc) => doc,
            None => try!(visitor.missing_field("doc")),
        };

        try!(visitor.end());

        Ok(Change { seq: seq, id: id, changes: changes, doc: doc })
    }
}
