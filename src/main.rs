use std::ffi::OsString;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

use clap::Parser;
use sha2::{Digest, Sha256};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// File to be hashed
    file_in: OsString,

    /// File to write normalized input into
    file_out: Option<OsString>,
}

fn main() {
    let cli = Cli::parse();

    let file_in = cli.file_in;
    let file_out = cli.file_out;

    let file_in = File::open(file_in).unwrap();
    let file_in = BufReader::new(file_in);

    let mut file_out = match file_out {
        Some(file_out) => {
            let file_out = File::create(file_out).unwrap();
            let file_out = BufWriter::new(file_out);
            Some(file_out)
        }
        None => None,
    };

    let mut hasher = Sha256::new();
    for line in file_in.lines() {
        let line = line.unwrap();
        let line = format!("{}\n", line);
        hasher.update(&line);

        if let Some(file_out) = &mut file_out {
            file_out.write(line.as_bytes()).unwrap();
        }
    }

    let hash = hasher.finalize();

    let hex_hash = base16ct::lower::encode_string(&hash);
    println!("{}", hex_hash);
}

#[test]
fn verify_app() {
    use clap::CommandFactory;
    Cli::command().debug_assert()
}
