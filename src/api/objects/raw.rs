use std::io;
use std::io::prelude::*;
use std::fs::{File, OpenOptions, create_dir_all};
use std::path::{Path, PathBuf};
use flate2::Compression;
use flate2::write::{ZlibEncoder, ZlibDecoder};
use sha1::{Sha1, Digest};

const OBJECTS_DIR: &str = "git2/objects/";

pub struct ObjectWriter {
    encoder: ZlibEncoder<Vec<u8>>,
    hasher: Sha1,
}

fn get_object_path(hash: &str, create_dir: bool) -> io::Result<PathBuf> {
    let hash1 = &hash[0..2];
    let hash2 = &hash[2..];

    let hash_dir = Path::new(OBJECTS_DIR).join(hash1);
    if create_dir {
        create_dir_all(&hash_dir)?;
    };

    let object_file = hash_dir.join(hash2);

    Ok(object_file)
}

impl ObjectWriter {
    pub fn new() -> Self {
        Self {
            encoder: ZlibEncoder::new(Vec::new(), Compression::default()),
            hasher: Sha1::new(),
        }
    }

    pub fn finalize(self) -> io::Result<String> {
        let mut compressed_bytes = self.encoder.finish()?;
        let res = self.hasher.finalize();

        let hash = format!("{:x}", res);
        let object_path = get_object_path(&hash, true)?;
        let mut f = io::BufWriter::new(OpenOptions::new().write(true).create(true).open(object_path)?);
        f.write_all(compressed_bytes.by_ref())?;

        Ok(hash)
    }
}

impl Write for ObjectWriter {
    fn write(&mut self, chunk: &[u8]) -> io::Result<usize> {
        let size = self.encoder.write(chunk)?;
        self.hasher.update(chunk);
        Ok(size)
    }

    fn flush(&mut self) -> io::Result<()> {
        unimplemented!();
    }
}

pub fn read_object(hash: &str) -> io::Result<Vec<u8>> {
    let mut writer = Vec::new();
    let mut decoder = ZlibDecoder::new(writer);

    // current_dir: falseを渡しているのでErrが返ることはない
    let object_path = get_object_path(hash, false).unwrap();
    let mut f = io::BufReader::new(File::open(object_path)?);
    let mut buf = Vec::new();
    f.read_to_end(&mut buf)?;
    decoder.write(&buf)?;
    writer = decoder.finish()?;

    Ok(writer)
}
