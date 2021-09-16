use std::convert::TryInto;
use std::io;
use std::io::prelude::*;
use std::fs::{File, OpenOptions, create_dir_all};
use std::path::{Path, PathBuf};
use flate2::Compression;
use flate2::write::{ZlibEncoder, ZlibDecoder};
use sha1::{Sha1, Digest};
use hex;

use super::base::ObjectBase;

const OBJECTS_DIR: &str = "git2/objects/";

type HashType = [u8; 20];

pub struct ObjectWriter {
    encoder: ZlibEncoder<Vec<u8>>,
    hasher: Sha1,
}

pub fn byte_hash_to_string(hash: &HashType) -> String {
    hex::encode(hash)
}

fn get_object_path(str_hash: &str, create_dir: bool) -> io::Result<PathBuf> {
    let str_hash1 = &str_hash[0..2];
    let str_hash2 = &str_hash[2..];

    let hash_dir = Path::new(OBJECTS_DIR).join(str_hash1);
    if create_dir {
        create_dir_all(&hash_dir)?;
    };

    let object_file = hash_dir.join(str_hash2);

    Ok(object_file)
}

impl ObjectWriter {
    pub fn new() -> Self {
        Self {
            encoder: ZlibEncoder::new(Vec::new(), Compression::default()),
            hasher: Sha1::new(),
        }
    }

    pub fn write_object<Base>(mut self, object: Base) -> std::io::Result<HashType> where Base: ObjectBase {
        object.write_to(&mut self)?;
        self.finalize()
    }

    pub fn finalize(self) -> io::Result<HashType> {
        let mut compressed_bytes = self.encoder.finish()?;
        let res = self.hasher.finalize();
        let hash: HashType = res.as_slice().try_into().unwrap(); // TODO

        let str_hash = byte_hash_to_string(&hash);
        let object_path = get_object_path(&str_hash, true)?;
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

const BUF_SIZE: usize = 2048;

pub fn read_object(str_hash: &str) -> io::Result<Vec<u8>> {
    let mut writer = Vec::new();
    let mut decoder = ZlibDecoder::new(writer);

    // current_dir: falseを渡しているのでErrが返ることはない
    let object_path = get_object_path(str_hash, false).unwrap();
    let mut f = io::BufReader::new(File::open(object_path)?);

    let mut buf = [0u8; BUF_SIZE];

    loop {
        if f.read(&mut buf)? <= 0 {
            break;
        }

        decoder.write(&buf)?;
    }

    writer = decoder.finish()?;

    Ok(writer)
}
