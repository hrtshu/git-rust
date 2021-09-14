use std::io::{self, Read, prelude::*};

use crate::api::objects::io::ObjectWriter;

pub fn write_blob_object<R>(stream: &mut R) -> io::Result<String> where R: Read {
    let mut content = Vec::new();
    let size = stream.read_to_end(&mut content)?;

    let mut writer = ObjectWriter::new();
    write!(writer, "blob {}\x00", size)?;
    writer.write(&content)?;
    let hash = writer.finalize()?;

    Ok(hash)
}
