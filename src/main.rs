use std::env::args;
use std::process::exit;
use std::fs::{create_dir, File};
use std::path::Path;
use std::io::{BufReader, BufWriter, Read, Write, stdin};
use std::str::FromStr;

use chrono::{DateTime, Local, Utc};

mod api;
use api::objects::tree::{Mode, TreeEntry, TreeObject};
use api::reflog::{RefLog, RefLogKind, append_reflog};
use api::objects::io::{ObjectWriter, ObjectReader, Hash};
use api::objects::blob::BlobObject;
use api::objects::commit::CommitObject;
use api::tree;

use crate::api::common::datetime::Timestamp;
use crate::api::common::user::User;

fn print_usage(args: &Vec<String>) {
    eprintln!("Usage: {:} subcommand", args[0])
}

fn do_init() -> i32 {
    let path_to_init = Path::new(".");
    let git_dir = path_to_init.join(".git");

    // create .git/ directory
    create_dir(&git_dir).unwrap_or_else(|_| {
        println!("already initialized");
        return;
    });

    // create .git/HEAD file
    let head_file = git_dir.join("HEAD");
    let mut f = BufWriter::new(File::create(&head_file).unwrap());
    f.write(b"ref: refs/heads/master\n").unwrap();

    // create .git/objects/ directory
    let objects_dir = git_dir.join("objects");
    create_dir(&objects_dir).unwrap();

    // create .git/refs/ directory
    let refs_dir = git_dir.join("refs");
    create_dir(&refs_dir).unwrap();

    // create .git/refs/heads/ directory
    let refs_heads_dir = refs_dir.join("heads");
    create_dir(&refs_heads_dir).unwrap();

    // create .git/refs/tags/ directory
    let refs_tags_dir = refs_dir.join("tags");
    create_dir(&refs_tags_dir).unwrap();

    0
}

// TODO: src/api/objects/raw.rsのBUF_SIZEと共通化する
const BUF_SIZE: usize = 2048;

fn do_write_object() -> i32 {
    let mut buf = [0u8; BUF_SIZE];
    let mut writer = ObjectWriter::new();

    loop {
        match BufReader::new(stdin()).read(&mut buf) {
            Ok(size) => {
                if size <= 0 {
                    break;
                }
            }
            Err(_) => {
                eprintln!("error: failed to read stdin");
                return 1;
            },
        };

        if let Err(_) = writer.write(&buf) {
            eprintln!("error: failed to write to the object file");
            return 1;
        };
    }

    match writer.finalize() {
        Ok(hash) => {
            println!("{}", &hash);
            0
        },
        Err(_) => {
            eprintln!("error: failed to write to the object file");
            1
        },
    }
}

fn do_read_object(subcommand_args: Vec<String>) -> i32 {
    if subcommand_args.len() != 1 {
        eprintln!("Usage: read-object OBJECT_HASH");
        return 1;
    }
    let hash = subcommand_args[0].to_string();

    let content = match ObjectReader::read(&hash) {
        Ok(content) => content,
        Err(_) => {
            eprintln!("error: failed to read object file");
            return 1;
        },
    };

    match String::from_utf8(content) {
        Ok(res) => {
            print!("{}", res);
            0
        },
        Err(_) => {
            eprintln!("error: can't display the content read from object file");
            1
        },
    }
}

fn do_write_blob() -> i32 {
    let mut stream = BufReader::new(stdin());
    let mut content = Vec::new();

    if let Err(_) = stream.read_to_end(&mut content) {
        eprintln!("error: failed to read stdin");
        return 1;
    }

    let blob_object = BlobObject::new(content);

    match ObjectWriter::write(blob_object) {
        Ok(hash) => {
            println!("{}", hash);
            0
        },
        Err(_) => {
            eprintln!("error: failed to write blob object");
            1
        }
    }
}

fn do_write_tree() -> i32 {
    let mut tree = TreeObject::new();

    tree.add(TreeEntry {
        mode: Mode(0o100644),
        name: String::from("hoge.txt"),
        hash: Hash(*b"0123456789abcdef0123"),
    });
    tree.add(TreeEntry {
        mode: Mode(0o100644),
        name: String::from("foo.txt"),
        hash: Hash(*b"0123456789abcdef0123"),
    });

    match ObjectWriter::write(tree) {
        Ok(hash) => {
            println!("{}", hash);
            0
        },
        Err(_) => {
            eprintln!("error: failed to write tree object");
            1
        },
    }
}

fn do_tree_test() -> i32 {
    let mut root_tree = tree::Tree::new();

    let mut path1 = String::from("src/api/objects/blob.rs");

    root_tree.add_path(&mut path1, true);

    println!("{:?}", root_tree);

    let tree_hash = root_tree.write_recursively().unwrap();

    let user = User { name: String::from("Shuhei"), email: String::from("sh7916@gmail.com") };
    let timestamp = Timestamp::from_datetime::<Local>(DateTime::from_str("2021-09-16 00:00:00+09:00").unwrap());

    let commit = CommitObject {
        tree_hash,
        parent_hash: Hash(*b"00000000000000000000"),
        author: &user,
        author_timestamp: &timestamp,
        committer: &user,
        commit_timestamp: &timestamp,
        message: String::from("Initial commit"),
    };

    let hash = ObjectWriter::write(commit).unwrap();

    println!("{}", hash);

    0
}

fn do_commit_test() -> i32 {
    let ts = Timestamp::now();
    println!("{}", ts);
    let ts = Timestamp::from_datetime(Utc::now());
    println!("{}", ts);
    0
}

fn do_reflog_test() -> i32 {
    println!("add!!!");

    let ref_log = RefLog {
        prev_hash: String::from("0000000000000000000000000000000000000000"),
        hash: String::from("335584cfc68b36a5f2332c10b32a0cf6a441cad8"),
        author: String::from("Shuhei"),
        email: String::from("sh7916@gmail.com"),
        timestamp: 1631017871,
        timezone: 540,
        kind: RefLogKind::COMMIT,
        description: String::from("Initial commit"),
    };
    append_reflog("hoge", ref_log);

    0
}

fn main() {
    let mut args: Vec<String> = args().collect();

    if args.len() < 2 {
        print_usage(&args);
        exit(1);
    }

    let subcommand = args[1].to_string();
    args.remove(0);
    args.remove(0);
    let subcommand_args = args;

    let exit_code: i32 = match subcommand.as_str() {
        "init"         => do_init(),
        "write-object" => do_write_object(),
        "read-object"  => do_read_object(subcommand_args),
        "write-blob"   => do_write_blob(),
        "write-tree"   => do_write_tree(),
        "tree-test"    => do_tree_test(),
        "commit-test"  => do_commit_test(),
        "reflog-test"  => do_reflog_test(),
        _ => {
            eprintln!("unknown subcommand: {:}", subcommand);
            1
        }
    };

    exit(exit_code);
}
