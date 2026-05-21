#[derive(Debug, Clone)]
pub enum Value {
    Int(i64),
    Text(String),
    Bool(bool),
    Null,
}

#[derive(Debug, Clone)]
pub struct Row {
    pub values: Vec<Value>,
}

#[derive(Debug, Clone, Copy)]
pub struct TransactionId(pub u64);

#[derive(Debug, Clone)]
pub struct Tuple {
    pub row: Row,
    pub tx_created: u64,
    pub tx_deleted: Option<u64>,
}
