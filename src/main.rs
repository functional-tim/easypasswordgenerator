/*
    main.rs - Console program to create the password.

    (C) 2020 Tim Gravert <crazymind102@googlemail.com>

    License: MIT OR Apache-2.0
*/


use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::{Path, PathBuf};
use std::process::exit;
use structopt::StructOpt;

mod password;



/// Struct for the parameters of the app.
#[derive(Debug, StructOpt)]
#[structopt(name = "easypassword",
about = "A program to create passwords like in xkcd.com/936.
The file has to have one word per line.
One seperator should be a special character.
The other seperator should be a number.")]
struct Opt {
    /// Input file
    #[structopt(parse(from_os_str))]
    file: PathBuf,

    /// Seperator 1
    #[structopt()]
    seperator1: String,

    /// Seperator 2
    #[structopt()]
    seperator2: String,

    /// Number of words
    #[structopt(short = "n", long = "number", default_value = "4")]
    number: usize,
}


/// Auxiliary function to transform the input file into a Vector of single words.
/// Input file has to be formatted in such a way that every word is on a single line.
fn lines_from_file(filename: impl AsRef<Path>) -> Result<Vec<String>, (String, i32)> {
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => return Err((String::from("no such file"), 2)),
    };
    let buf = BufReader::new(file);
    match buf.lines().collect() {
        Ok(res) => return Ok(res),
        Err(_) => return Err((String::from("file contained invalid UTF-8"),101)),
    };
}


/// Main program logic.
fn main() {
    let opt = Opt::from_args();
    let wordlist = match lines_from_file(opt.file) {
        Ok(x) => x,
        Err((err, c)) => {
            eprintln!("Error: {}", err);
            exit(c);
        },
    };
    let password = password::create_password(wordlist, opt.seperator1, opt.seperator2, opt.number);
    println!("{}", password);
}
