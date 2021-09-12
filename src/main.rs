use std::env::{args};
use std::process::{exit};
use std::fs::{create_dir, File};
use std::path::{Path};
use std::io::{BufWriter, Read, Write, stdin};

mod api;
use api::reflog::{RefLog, RefLogKind, append_reflog};
use api::objects::{ObjectWriter, read_object};

fn print_usage(args: &Vec<String>) {
    eprintln!("Usage: {:} subcommand", args[0])
}

fn do_init() {
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
}

fn do_write_object() {
    let mut content = Vec::new();
    stdin().read_to_end(&mut content).unwrap();
    let mut writer = ObjectWriter::new();
    writer.write(&content);
    let hash = writer.finalize();
    println!("{}", hash);
}

fn do_read_object(subcommand_args: Vec<String>) {
    if subcommand_args.len() != 1 {
        eprintln!("Usage: read-object OBJECT_HASH");
        return;
    }
    let hash = subcommand_args[0].to_string();

    let content = read_object(&hash);
    let res = String::from_utf8(content).unwrap();
    print!("{}", res);
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

    match subcommand.as_str() {
        "init" => {
            do_init();
        },
        "add" => {
            println!("add!!!");

            let mut writer = ObjectWriter::new();
            writer.write(b"hoge foo bar");
            let hash = writer.finalize();

            let content = read_object(&hash);
            let res = String::from_utf8(content).unwrap();
            println!("{}", res);

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
        },
        "write-object" => {
            do_write_object();
        },
        "read-object" => {
            do_read_object(subcommand_args);
        }
        _ => {
            eprintln!("unknown subcommand: {:}", subcommand);
            exit(1);
        }
    }
}
