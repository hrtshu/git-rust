use std::io::Write;

const HASH_SIZE: usize = 20;

pub struct TreeEntry {
    pub mode: String, // TODO: 文字列長をバリデーション
    pub name: String,
    pub hash: [u8; HASH_SIZE],
}

impl TreeEntry {
    pub fn write_to<T>(&self, writer: &mut T) -> std::io::Result<()> where T: Write {
        write!(writer, "{} {}\x00", self.mode, self.name)?;
        writer.write(&self.hash)?;

        Ok(())
    }

    pub fn size(&self) -> usize {
        6 + 1 + self.name.len() + 1 + HASH_SIZE
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

    pub fn size(&self) -> usize {
        let mut total: usize = 0;
        for entry in self.entries.iter() {
            total += entry.size();
        }
        total
    }

    pub fn write_to<T>(&self, writer: &mut T) -> std::io::Result<()> where T: Write {
        write!(writer, "tree {}\x00", self.size())?;

        for entry in self.entries.iter() {
            entry.write_to(writer)?
        }

        Ok(())
    }
}