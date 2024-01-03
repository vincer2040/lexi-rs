#[derive(Debug, PartialEq, Eq)]
pub enum SimpleString {
    Ok,
    Pong,
}

#[derive(Debug)]
pub enum LexiData {
    Simple(SimpleString),
    Int(i64),
    Bulk(String),
    Error(String),
    Array(Vec<LexiData>),
}

impl Into<LexiData> for i64 {
    fn into(self) -> LexiData {
        LexiData::Int(self)
    }
}
