mod commandline;
use commandline::ProgramOptions;

use regex::Regex;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::string::String;
use std::vec::Vec;
use structopt::StructOpt;

/// Return the lines that match our regex
fn get_matches<R: Read>(pattern: Regex, istream: R) -> Vec<String> {
    let mut matched_lines: Vec<String> = Vec::new();
    let reader = BufReader::new(istream);

    for line in reader.lines() {
        match line {
            Ok(text) => 
                match pattern.find(&text) {
                    Some(_) => matched_lines.push(text),
                    None => (),
                }
,
            Err(_) => panic!("Error during read"),
        }
    }
    matched_lines
}

/// This should probably return Result<usize>
fn put_matches<W: Write>(lines: Vec<String>, ostream: W) -> usize {
    let mut writer = BufWriter::new(ostream);

    // What do I even use this for
    let mut num_bytes = 0;

    for line in lines {
        num_bytes += match writer.write(line.as_bytes()) {
            Ok(n) => n,
            Err(err) => panic!(format!("Error occured writing to output file\n{:?}", err)),
        };
        let _ = writer.write_all(b"\n");
    }
    num_bytes
}

fn main() {
    let opts = ProgramOptions::from_args();
    let matches = get_matches(opts.pattern, opts.input);
    put_matches(matches, opts.output);
}
