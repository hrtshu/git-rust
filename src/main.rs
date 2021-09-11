use std::env::{args};
use std::process::{exit};
use std::fs::{create_dir, File};
use std::path::{Path};
use std::io::{BufWriter, Write};

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

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() < 2 {
        print_usage(&args);
        exit(1);
    }

    let subcommand = args[1].to_string();

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
        _ => {
            eprintln!("unknown subcommand: {:}", subcommand);
            exit(1);
        }
    }
}
