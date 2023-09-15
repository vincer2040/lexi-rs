
#[derive(Debug, PartialEq, Eq)]
pub enum LexiType {
    BulkString(String),
    Array(Vec<LexiType>),
    Int(i64),
}

impl Into<LexiType> for &str {
    fn into(self) -> LexiType {
        LexiType::BulkString(self.to_owned())
    }
}

impl Into<LexiType> for String {
    fn into(self) -> LexiType {
        LexiType::BulkString(self)
    }
}
