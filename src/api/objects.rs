use std::io::{BufWriter};
use std::io::prelude::*;
use std::fs::{OpenOptions, create_dir};
use std::path::{Path, PathBuf};
use flate2::Compression;
use flate2::write::ZlibEncoder;
use sha1::{Sha1, Digest};

const OBJECTS_DIR: &str = "git2/objects/";

pub struct ObjectWriter {
    encoder: ZlibEncoder<Vec<u8>>,
    hasher: Sha1,
}

fn get_object_path(hash: &str) -> PathBuf {
    let hash1 = &hash[0..2];
    let hash2 = &hash[2..];

    let hash_dir = Path::new(OBJECTS_DIR).join(Path::new(hash1));
    create_dir(&hash_dir).unwrap();
    let object_file = hash_dir.join(Path::new(hash2));

    object_file
}

impl ObjectWriter {
    pub fn new() -> Self {
        Self {
            encoder: ZlibEncoder::new(Vec::new(), Compression::default()),
            hasher: Sha1::new(),
        }
    }

    pub fn write(&mut self, chunk: &[u8]) {
        self.encoder.write_all(chunk).unwrap();
        self.hasher.update(chunk);
    }

    pub fn finalize(self) -> String {
        let mut compressed_bytes = self.encoder.finish().unwrap();
        let res = self.hasher.finalize();

        let hash = format!("{:x}", res);
        let object_path = get_object_path(&hash);
        let mut f = BufWriter::new(OpenOptions::new().write(true).create(true).open(object_path).unwrap());
        f.write_all(compressed_bytes.by_ref()).unwrap();

        hash
    }
    }
}
