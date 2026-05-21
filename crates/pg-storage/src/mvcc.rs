pub type TxId = u64;

#[derive(Clone, Debug)]
pub struct VersionedTuple {
    pub created_at: TxId,
    pub deleted_at: Option<TxId>,
    pub data: Vec<u8>,
}

pub struct MVCC;

impl MVCC {
    pub fn visible(&self, tuple: &VersionedTuple, tx: TxId) -> bool {
        tuple.created_at <= tx && tuple.deleted_at.map_or(true, |d| d > tx)
    }
}
