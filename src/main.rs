use std::env::{args};
use std::process::{exit};
use std::fs::{create_dir, File};
use std::path::{Path};
use std::io::{BufWriter, Write};

mod api;
use api::reflog::{RefLog, RefLogKind};

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
        },
        _ => {
            eprintln!("unknown subcommand: {:}", subcommand);
            exit(1);
        }
    }
}
