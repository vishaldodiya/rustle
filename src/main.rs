use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::Write;
#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(pub md);

fn get_title() -> String {
    let mut the_title = String::from(env!("CARGO_PKG_NAME"));
    the_title.push_str(" (v");
    the_title.push_str(env!("CARGO_PKG_VERSION"));
    the_title.push_str("), ");
    the_title.push_str(env!("CARGO_PKG_DESCRIPTION"));

    return the_title;
}

fn print_short_banner() {
    println!("{}", get_title());
}

fn print_long_banner() {
    print_short_banner();
    println!("Written by: {}\nGithub: {}\nUsage: rustle <example>.md\n", env!("CARGO_PKG_AUTHORS"), env!("CARGO_PKG_HOMEPAGE"));
}

fn usage() {
    print_long_banner();
}

fn parse_markdown_file(_filename: &str) {
    print_short_banner();
    println!("[ INFO ] {} Parsing Starting!", _filename);

    // Create path variable for the input file.
    let input_filename = Path::new(_filename);

    // Open file.
    let file = File::open(&input_filename).expect("[ ERROR ] Failed to open file!");

    let mut _htag: bool = false;
    let mut _ptag: bool = false;

    // Store all tokens.
    let mut tokens: Vec<String> = Vec::new();

    // Read file line by line.
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line_contents = line.unwrap();
        let output_line = md::MarkDownParser::new().parse(&line_contents).unwrap();

        tokens.push(output_line);
    }

    let mut output_filename = String::from(&_filename[.._filename.len()-3]);
    output_filename.push_str(".html");

    let mut outfile = File::create(output_filename).expect("[ ERROR ] Could not create output file!");

    for line in &tokens {
        outfile.write(line.as_bytes()).expect("[ ERROR ] Could not write output file!");
        outfile.write("\n".as_bytes()).expect("[ ERROR ] Could not write output file!");
    }

    println!("[ INFO ] Parsing Complete!")
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        2 => parse_markdown_file(&args[1]),
        _ => {
            println!("[ ERROR ] You forgot to specify the markdown file to parse!");
            usage();
        }
    }
}

#[test]
fn test_headings() {
    // Test H1 Tag.
    assert_eq!(md::MarkDownParser::new().parse("# Hello9").unwrap(), "<h1>Hello9</h1>");

    // Test H2 Tag.
    assert_eq!(md::MarkDownParser::new().parse("## Hello").unwrap(), "<h2>Hello</h2>");

    // Test H3 Tag.
    assert_eq!(md::MarkDownParser::new().parse("### Hello").unwrap(), "<h3>Hello</h3>");

    // Test H4 Tag.
    assert_eq!(md::MarkDownParser::new().parse("#### Hello").unwrap(), "<h4>Hello</h4>");

    // Test H5 Tag.
    assert_eq!(md::MarkDownParser::new().parse("##### Hello").unwrap(), "<h5>Hello</h5>");

    // Test H6 Tag.
    assert_eq!(md::MarkDownParser::new().parse("###### Hello").unwrap(), "<h6>Hello</h6>");
}

#[test]
fn test_paragraphs() {
    // Test P tags.
    assert_eq!(md::MarkDownParser::new().parse("Hello World").unwrap(), "<p>Hello World</p>");
}

#[test]
fn test_bold() {
    // Test only Strong.
    assert_eq!(md::MarkDownParser::new().parse("**Hello World**").unwrap(), "<strong>Hello World</strong>");

    // Test Strong with initial text.
    assert_eq!(md::MarkDownParser::new().parse("Hey There **Hello World**").unwrap(), "<p>Hey There <strong>Hello World</strong></p>");

    // Test Strong with text at end.
    assert_eq!(md::MarkDownParser::new().parse("**Hello World** Howdy").unwrap(), "<p><strong>Hello World</strong> Howdy</p>");

    // Test Strong with text on both sides.
    assert_eq!(md::MarkDownParser::new().parse("Hey There **Hello World** Howdy").unwrap(), "<p>Hey There <strong>Hello World</strong> Howdy</p>");
}