#[derive(Debug, PartialEq, Eq)]
pub enum SimpleString {
    Ok,
    Pong,
    None,
}

#[derive(Debug)]
pub enum LexiData {
    Simple(SimpleString),
    Int(i64),
    Double(f64),
    Bulk(String),
    Error(String),
    Array(Vec<LexiData>),
}

impl Into<LexiData> for &str {
    fn into(self) -> LexiData {
        LexiData::Bulk(self.to_string())
    }
}

impl Into<LexiData> for String {
    fn into(self) -> LexiData {
        LexiData::Bulk(self)
    }
}

impl Into<LexiData> for i64 {
    fn into(self) -> LexiData {
        LexiData::Int(self)
    }
}

impl Into<LexiData> for i32 {
    fn into(self) -> LexiData {
        LexiData::Int(self as i64)
    }
}

impl Into<LexiData> for i16 {
    fn into(self) -> LexiData {
        LexiData::Int(self as i64)
    }
}

impl Into<LexiData> for i8 {
    fn into(self) -> LexiData {
        LexiData::Int(self as i64)
    }
}

impl Into<LexiData> for u32 {
    fn into(self) -> LexiData {
        LexiData::Int(self as i64)
    }
}

impl Into<LexiData> for u16 {
    fn into(self) -> LexiData {
        LexiData::Int(self as i64)
    }
}

impl Into<LexiData> for u8 {
    fn into(self) -> LexiData {
        LexiData::Int(self as i64)
    }
}

impl Into<LexiData> for f64 {
    fn into(self) -> LexiData {
        LexiData::Double(self)
    }
}

impl Into<LexiData> for f32 {
    fn into(self) -> LexiData {
        LexiData::Double(self as f64)
    }
}
