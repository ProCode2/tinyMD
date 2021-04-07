use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn parse_markdown_file(filename: &str) {
    print_banner();
    println!("[ INFO ] Trying to parse {}...", filename);
    let input_path = Path::new(filename);

    let file = File::open(&input_path).expect("[ ERROR ] Failed to open file!");

    let mut ptag: bool = false;
    let mut htag: bool = false;

    let mut tokens: Vec<String> = Vec::new();

    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line_contents = line.unwrap();
        let mut first_char: Vec<char> = line_contents.chars().take(1).collect();

        let mut output_line = String::new();

        match first_char.pop() {
            Some('#') => {
                if ptag {
                    ptag = false;
                    output_line.push_str("</p>\n");
                }

                if htag {
                    htag = false;
                    output_line.push_str("</h1>\n")
                }

                htag = true;
                output_line.push_str("\n\n<h1>");
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

        if ptag {
            ptag = false;
            output_line.push_str("</p>\n");
        }

        if htag {
            htag = false;
            output_line.push_str("</h1>\n");
        }

        if output_line != "<p></p>\n" {
            tokens.push(output_line);
        }
    }

    println!("{:?}", input_path)
}

fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

fn usage() -> String {
    String::from("Usage: tinymd <somefile>.md")
}

fn get_title() -> String {
    format!(
        "{} ({}), {}",
        env!("CARGO_PKG_NAME"),
        get_version(),
        env!("CARGO_PKG_DESCRIPTION")
    )
}

fn print_banner() {
    println!("{}", get_title());
    println!("{}", usage());
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        2 => parse_markdown_file(&args[1]),
        _ => {
            println!("[ ERROR ] Invalid invocation (you done goofed!)");
            println!("{}", usage());
        }
    }
}
