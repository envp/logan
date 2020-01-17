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
        let text = line.unwrap();
        if pattern.find(&text).is_some() {
            matched_lines.push(text)
        }
    }
    matched_lines
}

/// This should probably return Result<usize>
fn put_matches<W: Write>(lines: Vec<String>, ostream: W) -> Result<(), std::io::Error> {
    let mut writer = BufWriter::new(ostream);

    for line in lines {
        writer.write(line.as_bytes())?;
        writer.write_all(b"\n")?;
    }
    Ok(())
}

fn main() {
    let opts = ProgramOptions::from_args();
    let matches = get_matches(opts.pattern, opts.input);
    put_matches(matches, opts.output).expect(&"Error during write!");
}
