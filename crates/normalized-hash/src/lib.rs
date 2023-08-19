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
    use std::iter::zip;
    use std::ops::Add;

    use tempfile::NamedTempFile;

    use super::*;

    struct TestEnv {
        file_with_crlf: NamedTempFile,
        file_with_lf: NamedTempFile,

        normalized_file_with_crlf: NamedTempFile,
        normalized_file_with_lf: NamedTempFile,
    }

    impl TestEnv {
        fn new() -> Result<Self, std::io::Error> {
            let mut file_with_crlf = NamedTempFile::new()?;
            let mut file_with_lf = NamedTempFile::new()?;

            let normalized_file_with_crlf = NamedTempFile::new()?;
            let normalized_file_with_lf = NamedTempFile::new()?;

            let content = vec!["A B", "C D"];

            file_with_crlf.write_all(content.join("\r\n").add("\r\n").as_bytes())?;
            file_with_lf.write_all(content.join("\n").add("\n").as_bytes())?;

            Ok(TestEnv {
                file_with_crlf,
                file_with_lf,

                normalized_file_with_crlf,
                normalized_file_with_lf,
            })
        }

        fn get_input_files(&self) -> Vec<&NamedTempFile> {
            vec![
                &self.file_with_crlf,
                &self.file_with_lf,
            ]
        }

        fn get_output_files(&self) -> Vec<&NamedTempFile> {
            vec![
                &self.normalized_file_with_crlf,
                &self.normalized_file_with_lf,
            ]
        }

        fn hash_files(&self, hasher: Hasher) -> Result<(), Box<dyn Error>> {
            let mut hash_check = None;
            let mut content_check = None;

            for (file_in, file_out) in zip(self.get_input_files(), self.get_output_files()) {
                let hash = hasher.hash_file(file_in, Some(file_out));

                if hash_check.is_none() {
                    hash_check = Some(hash.clone());
                    content_check = Some(fs::read_to_string(file_out)?)
                }

                if let (Some(hash_check), Some(content_check)) = (&hash_check, &content_check) {
                    assert_eq!(&hash, hash_check, "Hashes don't match");
                    assert_eq!(
                        &fs::read_to_string(file_out)?,
                        content_check,
                        "Normalized files don't match"
                    );
                }
            }

            Ok(())
        }
    }

    #[test]
    fn check_empty_file() -> Result<(), Box<dyn Error>> {
        let file = NamedTempFile::new()?;

        // Sanity check between hasher versions
        let hash_expected = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";
        let hash_actual = Hasher::new().hash_file(file, None::<OsString>);

        assert_eq!(hash_actual, hash_expected);

        Ok(())
    }

    #[test]
    fn check_different_eols() -> Result<(), Box<dyn Error>> {
        let test_env = TestEnv::new()?;
        test_env.hash_files(Hasher::new())?;

        assert_eq!(
            fs::read_to_string(&test_env.file_with_lf)?,
            fs::read_to_string(&test_env.normalized_file_with_lf)?,
            "Normalized files do not have LF"
        );

        Ok(())
    }
}
