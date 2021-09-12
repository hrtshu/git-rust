use std::io;
use std::io::prelude::*;

use crate::api::objects::raw::ObjectWriter;

pub fn write_blob_object(content: &Vec<u8>) -> io::Result<String> {
    let mut writer = ObjectWriter::new();
    write!(writer, "blob {}\x00", content.len())?;
    writer.write(&content)?;
    let hash = writer.finalize()?;

    Ok(hash)
}
