//! # normalized-hash
//!
//! [![badge github]][url github]
//! [![badge crates.io]][url crates.io]
//! [![badge docs.rs]][url docs.rs]
//! [![badge license]][url license]
//!
//! [badge github]: https://img.shields.io/badge/github-FloGa%2Fnormalized--hasher-green
//! [badge crates.io]: https://img.shields.io/crates/v/normalized-hash
//! [badge docs.rs]: https://img.shields.io/docsrs/normalized-hash
//! [badge license]: https://img.shields.io/crates/l/normalized-hash
//!
//! [url github]: https://github.com/FloGa/normalized-hasher/crates/normalized-hash
//! [url crates.io]: https://crates.io/crates/normalized-hash
//! [url docs.rs]: https://docs.rs/normalized-hash
//! [url license]: https://github.com/FloGa/normalized-hasher/blob/develop/crates/normalized-hash/LICENSE
//!
//! Cross-platform hash algorithm.
//!
//! *This is the library crate. If you're looking for the binary crate instead, go
//! to [`normalized-hasher`].*
//!
//! [`normalized-hasher`]: https://github.com/FloGa/normalized-hasher
//!
//! ## Summary
//!
//! This hashing algorithm allows consistent hashes even if you accidentally
//! convert a file from using UNIX line endings (LF) to Windows line endings
//! (CRLF). For a longish motivational speech about how such a thing can happen
//! and why you should want to even care about such a case, head over to
//! [`normalized-hasher`].
//!
//! ## Code Example
//!
//! ```rust no_run
//! use std::path::PathBuf;
//!
//! use normalized_hash::Hasher;
//!
//! fn main() {
//!     let file_in = PathBuf::from("input.txt");
//!     let hash = Hasher::new().hash_file(file_in, None::<PathBuf>);
//!     println!("{}", hash);
//! }
//! ```

use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;

use sha2::{Digest, Sha256};

pub struct Hasher {}

impl Hasher {
    /// Create new Hasher instance with default options.
    ///
    /// # Example
    ///
    /// ```
    /// use normalized_hash::Hasher;
    /// let hasher = Hasher::new();
    /// ```
    pub fn new() -> Self {
        Hasher {}
    }

    /// Create hash from a text file, regardless of line endings.
    ///
    /// This function reads `file_in` linewise, replacing whatever line ending is present with a
    /// single line feed character (`\n`). From this, it generates a hash code.
    ///
    /// Optionally, it is possible to write the normalized input to `file_out`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use std::path::PathBuf;
    ///     use normalized_hash::Hasher;
    ///
    ///     let hash_without_output = Hasher::new().hash_file(PathBuf::from("input.txt"), None::<PathBuf>);
    ///
    ///     let hash_with_output = Hasher::new().hash_file(
    ///     PathBuf::from("input.txt"),
    ///     Some(PathBuf::from("output.txt"))
    ///     );
    /// ```
    pub fn hash_file(
        &self,
        file_in: impl AsRef<Path>,
        file_out: Option<impl AsRef<Path>>,
    ) -> String {
        let file_in = File::open(file_in).unwrap();
        let file_in = BufReader::new(file_in);

        let mut file_out = file_out.and_then(|file_out| {
            let file_out = File::create(file_out).unwrap();
            let file_out = BufWriter::new(file_out);
            Some(file_out)
        });

        let mut hasher = Sha256::new();
        for line in file_in.lines() {
            let line = line.unwrap();
            let line = format!("{}\n", line);
            hasher.update(&line);

            if let Some(file_out) = &mut file_out {
                file_out.write_all(line.as_bytes()).unwrap();
            }
        }

        let hash = hasher.finalize();

        base16ct::lower::encode_string(&hash)
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;
    use std::ffi::OsString;
    use std::fs;

    use tempfile;

    use super::*;

    #[test]
    fn check_empty_file() -> Result<(), Box<dyn Error>> {
        let file = tempfile::NamedTempFile::new()?;

        // Sanity check between hasher versions
        let hash_expected = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";
        let hash_actual = Hasher::new().hash_file(file, None::<OsString>);

        assert_eq!(hash_actual, hash_expected);

        Ok(())
    }

    #[test]
    fn check_different_eols() -> Result<(), Box<dyn Error>> {
        let mut file_with_lf = tempfile::NamedTempFile::new()?;
        let mut file_with_crlf = tempfile::NamedTempFile::new()?;

        let file_with_lf_normalized = tempfile::NamedTempFile::new()?;
        let file_with_crlf_normalized = tempfile::NamedTempFile::new()?;

        file_with_lf.write_all("A\nb\n".as_ref())?;
        file_with_crlf.write_all("A\r\nb".as_ref())?;

        let hash_with_lf = Hasher::new().hash_file(&file_with_lf, Some(&file_with_lf_normalized));
        let hash_with_crlf =
            Hasher::new().hash_file(&file_with_crlf, Some(&file_with_crlf_normalized));

        assert_eq!(hash_with_lf, hash_with_crlf, "Hashes don't match");
        assert_eq!(
            fs::read_to_string(&file_with_lf_normalized)?,
            fs::read_to_string(&file_with_crlf_normalized)?,
            "Normalized files don't match"
        );
        assert_eq!(
            fs::read_to_string(&file_with_lf)?,
            fs::read_to_string(&file_with_lf_normalized)?,
            "Normalized files do not have LF"
        );

        Ok(())
    }
}
