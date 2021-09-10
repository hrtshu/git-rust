use std::io::{BufWriter};
use std::io::prelude::*;
use std::fs::{OpenOptions, create_dir};
use std::path::{Path};
use flate2::Compression;
use flate2::write::ZlibEncoder;
use sha1::{Sha1, Digest};

const OBJECTS_DIR: &str = "git2/objects/";

pub fn write_object(content: &[u8]) {
    // compress the data         
    let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
    e.write_all(content).unwrap();      
    let mut compressed_bytes = e.finish().unwrap();

    // calculate the hash
    let mut hasher = Sha1::new();
    hasher.update(content);       
    let res = hasher.finalize();
    let hex_hash = format!("{:x}", res);
    let hash1 = &hex_hash[0..2];
    let hash2 = &hex_hash[2..];

    let hash_dir = Path::new(OBJECTS_DIR).join(Path::new(hash1));
    create_dir(&hash_dir).unwrap();
    let object_file = hash_dir.join(Path::new(hash2));
    let mut f = BufWriter::new(OpenOptions::new().write(true).create(true).open(object_file).unwrap());
    f.write_all(compressed_bytes.by_ref()).unwrap();
}
