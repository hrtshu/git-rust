use std::fs::File;
use std::io::prelude::*;

use super::base::ObjectBase;

pub struct BlobObject {
    pub content: Vec<u8>,
}

impl BlobObject {
    pub fn new(content: Vec<u8>) -> Self {
        Self {
            content
        }
    }

    pub fn from_path(path: &String) -> std::io::Result<Self> {
        let mut f = File::open(path)?;
        let mut buf = Vec::new();
        f.read_to_end(&mut buf)?;
        Ok(Self::new(buf))
    }
}

impl ObjectBase for BlobObject {
    fn obj_type(&self) -> &str {
        "blob"
    }

    fn body_size(&self) -> usize {
        self.content.len()
    }

    fn write_body_to<W>(&self, writer: &mut W) -> std::io::Result<()> where W: Write {
        writer.write_all(&self.content)
    }
}
