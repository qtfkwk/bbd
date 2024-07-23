#![doc = include_str!("../README.md")]

use bbd_lib::*;
use clap::Parser;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(
    version,
    about = "\
Binary Braille Dump

Encode/decode data to/from Braille Patterns Unicode Block characters
\
    ",
    after_help = "\
---

Notes:

1. Styles:
    * `bcd`: Binary Coded Decimal of byte values 0-99
    * `direct`: Direct encoding using the standard Braille dot values
    * `nlbb`: Most significant nibble (MSN) left column, most significant bit
      (MSB) bottom row. This is the default style.
    * `nlbt`: MSN left column, MSB top row
    * `nrbb`: MSN right column, MSB bottom row
    * `nrbt`: MSN right column, MSB top row
\
    "
)]
struct Cli {
    /// Decode Braille characters to bytes using the given style; ignores
    /// wrapping
    #[arg(short)]
    decode: bool,

    /// Style (1)
    #[arg(
        short,
        value_name = "STYLE",
        value_parser = ["bcd", "direct", "nlbb", "nlbt", "nrbb", "nrbt"],
        default_value = "nlbb")
    ]
    style: String,

    /// Wrap to N columns ("bytes") per line; 0: disable wrapping
    #[arg(short, value_name = "N", default_value = "64")]
    columns: usize,

    /// Markdown output
    #[arg(short, conflicts_with = "decode")]
    markdown: bool,

    /// Input file(s); [default: "-" (stdin)]
    #[arg(value_name = "PATH")]
    files: Vec<PathBuf>,
}

fn read_file(file: &Path) -> Vec<u8> {
    let mut f = File::open(file).expect("no file found");
    let metadata = std::fs::metadata(file).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read_exact(&mut buffer).expect("buffer overflow");
    buffer
}

fn main() {
    let cli = Cli::parse();

    let style = cli.style.as_str();
    let (encode_byte, decode_byte) = match style {
        "bcd" => (encode_bcd as EncodeFn, decode_bcd as DecodeFn),
        "direct" => (encode_direct as EncodeFn, decode_direct as DecodeFn),
        "nlbb" => (encode_nlbb as EncodeFn, decode_nlbb as DecodeFn),
        "nlbt" => (encode_nlbt as EncodeFn, decode_nlbt as DecodeFn),
        "nrbb" => (encode_nrbb as EncodeFn, decode_nrbb as DecodeFn),
        "nrbt" => (encode_nrbt as EncodeFn, decode_nrbt as DecodeFn),
        _ => unreachable!(),
    };

    let mut files = cli.files.clone();
    let stdin = PathBuf::from("-");
    if files.is_empty() {
        files.push(stdin.clone());
    }
    for i in &files {
        if *i != stdin {
            if !i.exists() {
                eprintln!("File path `{}` does not exist!", i.display());
                std::process::exit(1);
            } else if !i.is_file() {
                eprintln!("File path `{}` is not a file!", i.display());
                std::process::exit(2);
            }
        }
    }

    let mut prev_content_length = 0;
    for i in &files {
        if cli.decode {
            let content = if i.as_os_str() == "-" {
                let mut r = String::new();
                std::io::stdin().read_to_string(&mut r).unwrap();
                r
            } else {
                std::fs::read_to_string(i).unwrap()
            };
            let binary = decode(&content, decode_byte);
            std::io::stdout().write_all(&binary).unwrap();
        } else {
            let content = if i.as_os_str() == "-" {
                let mut r = vec![];
                std::io::stdin().read_to_end(&mut r).unwrap();
                r
            } else {
                read_file(i)
            };
            let binary = encode(&content, encode_byte, cli.columns, prev_content_length);
            if cli.markdown {
                println!("`{}`:\n\n```\n{binary}\n```\n", i.display());
            } else {
                println!("{binary}");
            }
            prev_content_length = content.len();
        }
    }
}
