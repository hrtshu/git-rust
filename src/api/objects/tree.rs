use std::fmt::Display;
use std::io::Write;

use super::base::ObjectBase;
use super::io::{HASH_SIZE, Hash};

const MODE_LEN: usize = 6;

pub struct Mode(pub u32);

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s= format!("{:06o}", self.0); // TODO: もし可能であれば6をMODE_LEN定数で埋め込む
        write!(f, "{}", &s[(s.len() - MODE_LEN)..])
    }
}

pub struct TreeEntry {
    pub mode: Mode,
    pub name: String,
    pub hash: Hash,
}

impl TreeEntry {
    pub fn write_to<T>(&self, writer: &mut T) -> std::io::Result<()> where T: Write {
        write!(writer, "{} {}\0", self.mode, self.name)?;
        writer.write(self.hash.as_bytes())?;

        Ok(())
    }

    pub fn size(&self) -> usize {
        MODE_LEN + 1 + self.name.len() + 1 + HASH_SIZE
    }
}

pub struct TreeObject {
    entries: Vec<TreeEntry>,
}

impl TreeObject {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn add(&mut self, entry: TreeEntry) {
        self.entries.push(entry);
    }
}

impl ObjectBase for TreeObject {
    fn obj_type(&self) -> &str {
        "tree"
    }

    fn body_size(&self) -> usize {
        let mut total: usize = 0;
        for entry in self.entries.iter() {
            total += entry.size();
        }
        total
    }

    fn write_body_to<W>(&self, writer: &mut W) -> std::io::Result<()> where W: Write {
        for entry in self.entries.iter() {
            entry.write_to(writer)?;
        }

        Ok(())
    }
}
