use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::str;

#[inline]
fn parse_close_tag(tag: &mut bool, tag_name: &str, line: &mut String) {
    if *tag {
        *tag = false;
        line.push_str("</");
        line.push_str(tag_name);
        line.push_str(">\n");
    }
}

pub fn parse_markdown_file(_filename: &str) {
    print_short_banner();
    println!("[ INFO ] Trying to parse {}...", _filename);

    let input_filename = Path::new(_filename);

    // Opening the file
    let file = File::open(&input_filename).expect("[ ERROR ] Failed to open file!");

    let mut ptag: bool = false;
    let mut htag: bool = false;

    let mut tokens: Vec<String> = Vec::new();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line_contents = line.unwrap();

        // Takes first character
        let mut first_char: Vec<char> = line_contents.chars().take(1).collect();

        let mut output_line = String::new();
        match first_char.pop() {
            Some('#') => {
                parse_close_tag(&mut ptag, "p", &mut output_line);
                parse_close_tag(&mut htag, "h1", &mut output_line);

                htag = true;
                output_line.push_str("<h1>");
                output_line.push_str(&line_contents[2..]);
            }
            _ => {
                if !ptag {
                    ptag = true;
                    output_line.push_str("<p>");
                }
                output_line.push_str(&line_contents);
            }
        }
        parse_close_tag(&mut ptag, "p", &mut output_line);
        parse_close_tag(&mut htag, "h1", &mut output_line);

        if output_line != "<p></p>\n" {
            tokens.push(output_line);
        }
    }
    for t in &tokens {
        println!("{}", t);
    }

    let mut output_filename = String::from(&_filename[.._filename.len() - 3]);
    output_filename.push_str(".html");
    write_to_file(&output_filename, &tokens);
    println! {"[ INFO ] Parsing complete!"};
}

fn write_to_file(output_filename: &String, tokens: &Vec<String>) {
    let mut outfile = File::create(output_filename).expect("Couldn't create output file!");
    for line in tokens {
        outfile
            .write_all(line.as_bytes())
            .expect("[ ERROR ] Could not write to output file!");
    }
}

fn print_long_banner() {
    print_short_banner();
    println!(
        "Written by: {}\nHomepage: {}\nUsage: tinymd <somefile>.md\n",
        env!("CARGO_PKG_AUTHORS"),
        env!("CARGO_PKG_HOMEPAGE")
    );
}

fn print_short_banner() {
    println!("{}", get_title());
}

fn get_title() -> String {
    let mut title = String::from(env!("CARGO_PKG_NAME"));
    title.push_str(" (v");
    title.push_str(env!("CARGO_PKG_VERSION"));
    title.push_str("), ");
    title.push_str(env!("CARGO_PKG_DESCRIPTION"));
    title
}

pub fn usage() {
    print_long_banner();
}
