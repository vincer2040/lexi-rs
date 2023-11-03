#[derive(Debug, PartialEq, Eq, serde::Serialize)]
pub enum LexiType {
    BulkString(String),
    Array(Vec<LexiType>),
    Int(i64),
    Simple(String),
    Error(String),
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

impl Into<LexiType> for i64 {
    fn into(self) -> LexiType {
        LexiType::Int(self)
    }
}

impl Into<LexiType> for i32 {
    fn into(self) -> LexiType {
        LexiType::Int(self as i64)
    }
}

impl Into<LexiType> for i16 {
    fn into(self) -> LexiType {
        LexiType::Int(self as i64)
    }
}

impl Into<LexiType> for i8 {
    fn into(self) -> LexiType {
        LexiType::Int(self as i64)
    }
}

impl Into<LexiType> for u32 {
    fn into(self) -> LexiType {
        LexiType::Int(self as i64)
    }
}

impl Into<LexiType> for u16 {
    fn into(self) -> LexiType {
        LexiType::Int(self as i64)
    }
}

impl Into<LexiType> for u8 {
    fn into(self) -> LexiType {
        LexiType::Int(self as i64)
    }
}
