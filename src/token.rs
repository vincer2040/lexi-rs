use std::sync::Arc;


#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    BulkType,
    ArrayType,
    Len(Arc<str>),
    Bulk(Arc<str>),
    Int(Arc<[u8]>),
    RetCar,
    NewL,
    Illegal,
    Eof,
}
