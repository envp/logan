use regex::Regex;
use std::path::Path;
use structopt::StructOpt;

fn parse_regex(re: &str) -> Regex {
    Regex::new(re).unwrap()
}

fn parse_istream_path(path: &std::ffi::OsStr) -> Box<dyn std::io::Read> {
    if path.is_empty() || path == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(std::fs::File::open(Path::new(path)).unwrap())
    }
}

fn parse_ostream_path(path: &std::ffi::OsStr) -> Box<dyn std::io::Write> {
    if path.is_empty() || path == "-" {
        Box::new(std::io::stdout())
    } else {
        Box::new(std::fs::File::open(Path::new(path)).unwrap())
    }
}

// pub enum ErrorStream {
//     Standard(std::io::Stderr),
//     File(std::fs::File)
// }

#[derive(StructOpt)]
#[structopt(name = "logan", about = "Match, parse, and transform your logs")]
pub struct ProgramOptions {
    /// Run the tool in debug mode. Copious amounts of information will be
    /// dumped to stderr
    #[structopt(short)]
    pub debug: bool,

    /// Pattern to search for, syntax is similar to perl regexes
    #[structopt(
        short,
        name="REGEX",
        parse(from_str=parse_regex)
    )]
    pub pattern: Regex,

    /// Source of input lines, defaults to stdin
    #[structopt(
        name = "INFILE",
        default_value="-",
        parse(from_os_str=parse_istream_path)
    )]
    pub input: Box<dyn std::io::Read>,

    /// Sink for results, defaults to stdout
    #[structopt(
        short,
        name = "OUTFILE",
        default_value = "-",
        parse(from_os_str=parse_ostream_path)
    )]
    pub output: Box<dyn std::io::Write>,
}
