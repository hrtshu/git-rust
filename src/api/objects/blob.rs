use std::io::prelude::*;

use crate::api::objects::base::ObjectBase;

pub struct BlobObject {
    pub content: Vec<u8>,
}

impl BlobObject {
    pub fn new(content: Vec<u8>) -> Self {
        Self {
            content
        }
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
