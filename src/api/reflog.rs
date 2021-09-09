pub enum RefLogKind {
    COMMIT,
    CHECKOUT,
}

pub struct RefLog {
    pub prev_hash: String,
    pub hash: String,
    pub author: String,
    pub email: String,
    pub timestamp: u64,
    pub timezone: i16,
    pub kind: RefLogKind,
    pub description: String,
}
