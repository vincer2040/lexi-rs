
#[derive(Debug, PartialEq, Eq)]
pub enum LexiType {
    BulkString(String),
    Array(Vec<LexiType>),
}
