mod lib;

use lib::{parse_markdown_file, usage};
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::str;

fn read_file() -> io::Result<String> {
    let mut f = File::open("test.md")?;
    let mut buffer = Vec::new();

    f.read_to_end(&mut buffer)?;

    let s = match str::from_utf8(&buffer) {
        Ok(v) => v,
        Err(e) => panic!("Invaild UTF-8 sequence: {}", e),
    };
    Ok(s.to_string())
}

fn main() {
    usage();
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        2 => parse_markdown_file(&args[1]),
        _ => {
            println! {"[ERROR] Invaild invocation (you done goofed!)"};
            usage();
        }
    }
    let content = read_file().unwrap();
    println!("{:?}", &content);
}
