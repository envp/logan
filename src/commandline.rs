use regex::Regex;
use std::fs::{create_dir_all, File, OpenOptions};
use std::path::Path;
use structopt::StructOpt;

/// Parse the user provided regex into its own struct. This will fully support
/// capture expressions that can later be used to filter out log lines
fn parse_regex(re: &str) -> Regex {
    match Regex::new(re) {
        Ok(parsed) => parsed,
        Err(e) => {
            eprintln!("Failed to initialize Regex. Reason: \n {}", e);
            std::process::exit(1);
        }
    }
}

/// Return the input stream to use, given a path string as represented by the OS
/// Notable special cases:
///     - If path is empty, it returns stdin
fn istream_from_path(path: &std::ffi::OsStr) -> Box<dyn std::io::Read> {
    if path.is_empty() || path == "/dev/stdin" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(Path::new(path)).expect("Failed to open input file"))
    }
}

/// FIXME: This should defer output file creation to the first write
///
/// Given an operating system represented path string return a handle to the
/// output stream that writes to the file pointed to by that path
/// Notable special cases:
///     - if path is empty it returns stdout
fn ostream_from_path(path: &std::ffi::OsStr) -> Box<dyn std::io::Write> {
    if path.is_empty() || path == "/dev/stdout" {
        Box::new(std::io::stderr())
    } else {
        let output_path = Path::new(path);
        create_dir_all(output_path.parent().unwrap()).expect(&format!(
            "Unable to create parent dirs for output path: {:?}",
            path
        ));
        Box::new(
            OpenOptions::new()
                .create(true)
                .append(true)
                .open(output_path)
                .expect("Failed to create, or open output file for truncated write"),
        )
    }
}

#[derive(StructOpt)]
#[structopt(
    name = "logan",
    about = "Match, parse, and transform your logs",
    verbatim_doc_comment
)]
pub struct ProgramOptions {
    /// Pattern to search for, syntax is similar to perl regexes
    #[structopt(short, name="REGEX", parse(from_str=parse_regex))]
    pub pattern: Regex,

    /// Query to run on the consumed log line
    #[structopt(name = "QUERY")]
    pub query: Option<String>,

    /// Path to output file, defaults to stdout.
    /// {n}Existing files will be truncated before write
    #[structopt(
        short,
        name = "OUTFILE",
        default_value = "/dev/stdout",
        parse(from_os_str=ostream_from_path)
    )]
    pub output: Box<dyn std::io::Write>,

    /// Path to input file, defaults to stdin
    #[structopt(
        name = "INFILE",
        default_value="/dev/stdin",
        parse(from_os_str=istream_from_path)
    )]
    pub input: Box<dyn std::io::Read>,
}

impl std::fmt::Debug for ProgramOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ProgramOptions {{ pattern: '{:?}', query: {:?} }}",
            self.pattern, self.query
        )
    }
}
