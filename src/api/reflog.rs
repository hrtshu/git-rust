
use std::fs::{OpenOptions};
use std::io::{BufWriter, Write};
use std::fmt;

pub enum RefLogKind {
    COMMIT,
    CHECKOUT,
}

impl fmt::Display for RefLogKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RefLogKind::COMMIT => write!(f, "commit"),
            RefLogKind::CHECKOUT => write!(f, "checkout"),
        }
    }
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

pub fn append_reflog(target: &str, log: RefLog) {
    let mut f = BufWriter::new(
        OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(target).unwrap()
        );
    writeln!(
        &mut f,
        "{} {} {} <{}> {} {}    {}: {}",
        log.prev_hash,
        log.hash,
        log.author,
        log.email,
        log.timestamp,
        log.timezone,
        log.kind,
        log.description
    );
}
