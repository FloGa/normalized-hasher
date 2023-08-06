//! # normalized-hasher
//!
//! [![badge github]][url github]
//! [![badge crates.io]][url crates.io]
//! [![badge docs.rs]][url docs.rs]
//! [![badge license]][url license]
//!
//! [badge github]: https://img.shields.io/badge/github-FloGa%2Fnormalized--hasher-green
//! [badge crates.io]: https://img.shields.io/crates/v/normalized-hasher
//! [badge docs.rs]: https://img.shields.io/docsrs/normalized-hasher
//! [badge license]: https://img.shields.io/crates/l/normalized-hasher
//!
//! [url github]: https://github.com/FloGa/normalized-hasher
//! [url crates.io]: https://crates.io/crates/normalized-hasher
//! [url docs.rs]: https://docs.rs/normalized-hasher
//! [url license]: https://github.com/FloGa/normalized-hasher/blob/develop/LICENSE
//!
//! Create cross-platform hashes of text files.
//!
//! ## Motivation
//!
//! TBD
//!
//! ## Installation
//!
//! `normalized-hasher` can be installed easily through Cargo via `crates.io`:
//!
//! ```shell script
//! cargo install --locked normalized-hasher
//! ```
//!
//! Please note that the `--locked` flag is necessary here to have the exact same
//! dependencies as when the application was tagged and tested. Without it, you
//! might get more up-to-date versions of dependencies, but you have the risk of
//! undefined and unexpected behavior if the dependencies changed some
//! functionalities. The application might even fail to build if the public API of
//! a dependency changed too much.
//!
//! ## Usage
//!
//! ```text
//! Usage: normalized-hasher <FILE_IN> [FILE_OUT]
//!
//! Arguments:
//!   <FILE_IN>   File to be hashed
//!   [FILE_OUT]  File to write normalized input into
//!
//! Options:
//!   -h, --help     Print help
//!   -V, --version  Print version
//! ```

use std::ffi::OsString;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;

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

fn hash_file(file_in: impl AsRef<Path>, file_out: Option<impl AsRef<Path>>) -> String {
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

    base16ct::lower::encode_string(&hash)
}

fn main() {
    let cli = Cli::parse();

    println!("{}", hash_file(cli.file_in, cli.file_out));
}

#[test]
fn verify_app() {
    use clap::CommandFactory;
    Cli::command().debug_assert()
}
