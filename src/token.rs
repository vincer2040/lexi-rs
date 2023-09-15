
#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    BulkType,
    ArrayType,
    Len(String),
    Bulk(String),
    RetCar,
    NewL,
    Illegal,
    Eof,
}
