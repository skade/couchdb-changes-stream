use serde;
use serde::de::Deserialize;
use super::change::{Change,ChangeVisitor};
use super::last_seq::{LastSeq,LastSeqVisitor};

#[derive(Debug)]
pub enum ChangesLines {
    Change(Change),
    LastSeq(LastSeq)
}

impl Deserialize for ChangesLines {
    fn deserialize<D>(deserializer: &mut D) -> Result<ChangesLines, D::Error>
        where D: serde::Deserializer,
    {
        static CHANGES_FIELDS: &'static [&'static str] = &["seq", "id", "changes", "doc"];
        static LAST_SEQ_FIELD: &'static [&'static str] = &["last_seq"];

        deserializer.visit_struct("Changes", CHANGES_FIELDS, ChangeVisitor)
                    .map(|r| {
            ChangesLines::Change(r)
        }).or_else(|_| {
            deserializer.visit_struct("LastSeq", LAST_SEQ_FIELD, LastSeqVisitor)
                        .map(|r| {
                ChangesLines::LastSeq(r)
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use serde_json as json;
    use super::ChangesLines;

    #[test]
    fn parses_last_seq_line() {
        json::from_str::<ChangesLines>("{\"last_seq\":3}").unwrap();
    }
}
