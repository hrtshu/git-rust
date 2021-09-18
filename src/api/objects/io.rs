use std::convert::TryInto;
use std::fmt::Display;
use std::io::{self, BufReader, prelude::*};
use std::fs::{File, OpenOptions, create_dir_all};
use std::path::{Path, PathBuf};
use flate2::Compression;
use flate2::write::{ZlibEncoder, ZlibDecoder};
use sha1::{Sha1, Digest};
use hex;

use super::base::ObjectBase;

const OBJECTS_DIR: &str = "git2/objects/";

pub const HASH_SIZE: usize = 20;
pub const STR_HASH_LEN: usize = 40;

pub struct Hash(pub [u8; HASH_SIZE]);

impl Hash {
    // TODO: バイトに変換する適切なトレイトがあればそれに置換する
    pub fn as_bytes(&self) -> &[u8; HASH_SIZE] {
        &self.0
    }
}

impl Display for Hash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = hex::encode(self.0);
        write!(f, "{}", s)
    }
}

pub struct ObjectWriter {
    encoder: ZlibEncoder<Vec<u8>>,
    hasher: Sha1,
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
    pub fn write<Base>(object: Base) -> std::io::Result<Hash> where Base: ObjectBase {
        let mut writer = Self::new();
        object.write_to(&mut writer)?;
        writer.finalize()
    }

    pub fn new() -> Self {
        Self {
            encoder: ZlibEncoder::new(Vec::new(), Compression::default()),
            hasher: Sha1::new(),
        }
    }

    pub fn finalize(self) -> io::Result<Hash> {
        let mut compressed_bytes = self.encoder.finish()?;
        let res = self.hasher.finalize();
        let hash = Hash(res.as_slice().try_into().unwrap()); // TODO

        let object_path = get_object_path(&hash.to_string(), true)?;
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

pub struct ObjectReader {
    reader: BufReader<File>,
    decoder: ZlibDecoder<Vec<u8>>,
}

impl ObjectReader {
    pub fn read(str_hash: &str) -> std::io::Result<Vec<u8>> {
        let mut reader = Self::new(str_hash)?;

        reader.read_to_end()?;
        let buf = reader.finalize()?;

        Ok(buf)
    }

    pub fn new(str_hash: &str) -> std::io::Result<Self> {
        // current_dir: falseを渡しているのでErrが返ることはない
        let object_path = get_object_path(str_hash, false).unwrap();

        Ok(Self {
            reader: io::BufReader::new(File::open(object_path)?),
            decoder: ZlibDecoder::new(Vec::new()),
        })
    }

    fn read_to_end(&mut self) -> std::io::Result<usize> {
        let mut buf = Vec::new();
        let size = self.reader.read_to_end(&mut buf)?;
        self.decoder.write(&buf)?;
        Ok(size)
    }

    pub fn finalize(self) -> io::Result<Vec<u8>> {
        let res = self.decoder.finish()?;
        Ok(res)
    }
}
