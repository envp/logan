use regex::Regex;
use std::path::PathBuf;
use structopt::StructOpt;

fn parse_regex(re: &str) -> Regex {
    Regex::new(re).unwrap()
}

fn parse_istream_path(path: &std::ffi::OsStr) -> InputStream {
    if path.is_empty() || path == "-" {
        InputStream::stdin()
    } else {
        InputStream::file(PathBuf::from(path))
    }
}

fn parse_ostream_path(path: &std::ffi::OsStr) -> OutputStream {
    if path.is_empty() || path == "-" {
        OutputStream::stdout()
    } else {
        OutputStream::file(PathBuf::from(path))
    }
}

#[derive(Debug)]
pub enum InputStream {
    Standard(std::io::Stdin),
    File(std::fs::File),
}

impl InputStream {
    fn stdin() -> InputStream {
        InputStream::Standard(std::io::stdin())
    }

    fn file(path: std::path::PathBuf) -> InputStream {
        let in_ = std::fs::File::open(path).unwrap();
        InputStream::File(in_)
    }
}

impl std::io::Read for InputStream {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self {
            InputStream::Standard(ref mut f) => f.read(buf),
            InputStream::File(ref mut f) => f.read(buf)
        }
    }
}

#[derive(Debug)]
pub enum OutputStream {
    Standard(std::io::Stdout),
    File(std::fs::File),
}

impl OutputStream {
    fn stdout() -> OutputStream {
        OutputStream::Standard(std::io::stdout())
    }

    fn file(path: std::path::PathBuf) -> OutputStream {
        let out = std::fs::File::open(path).unwrap();
        OutputStream::File(out)
    }
}

impl std::io::Write for OutputStream {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match *self {
            OutputStream::Standard(ref mut f) => f.write(buf),
            OutputStream::File(ref mut f) => f.write(buf)
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        match *self {
            OutputStream::Standard(ref mut f) => f.flush(),
            OutputStream::File(ref mut f) => f.flush()
        }
    }
}

// pub enum ErrorStream {
//     Standard(std::io::Stderr),
//     File(std::fs::File)
// }

#[derive(Debug, StructOpt)]
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
    pub input: InputStream,

    /// Sink for results, defaults to stdout
    #[structopt(
        short,
        name = "OUTFILE",
        default_value = "-",
        parse(from_os_str=parse_ostream_path)
    )]
    pub output: OutputStream,
}
