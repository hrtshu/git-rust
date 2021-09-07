use std::env::{args};
use std::process::{exit};

fn print_usage(args: &Vec<String>) {
    eprintln!("Usage: {:} subcommand", args[0])
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
            println!("init!!!");
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
