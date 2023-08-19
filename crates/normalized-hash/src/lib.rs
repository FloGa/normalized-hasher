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
//!     let file_out = PathBuf::from("output.txt");
//!
//!     // Simple example with default options, without writing an output file
//!     let hash = Hasher::new().hash_file(&file_in, None::<PathBuf>);
//!     println!("{}", hash);
//!
//!     // More complex example, with writing output
//!     let hash = Hasher::new()
//!         .eol("\r\n")
//!         .no_eof(true)
//!         .hash_file(&file_in, Some(file_out));
//!     println!("{}", hash);
//! }
//! ```

use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;

use sha2::{Digest, Sha256};

pub struct Hasher {
    eol: String,
    ignore_whitespaces: bool,
    no_eof: bool,
}

impl Default for Hasher {
    fn default() -> Self {
        Self {
            eol: "\n".to_string(),
            ignore_whitespaces: false,
            no_eof: false,
        }
    }
}

impl Hasher {
    /// Create new Hasher instance with default options.
    ///
    /// # Defaults
    ///
    /// If not overwritten by the fluent API, the following defaults are valid:
    ///
    /// -   `eol`: `"\n"`
    ///
    ///     End-of-line sequence, will be appended to each normalized line for hashing.
    ///
    /// -   `ignore_whitespaces`: `false`
    ///
    ///     Ignore all whitespaces. This will remove all whitespaces from the input file when
    ///     generating the hash.
    ///
    /// -   `no_eof`: `false`
    ///
    ///     Skip last end-of-line on end-of-file. If this is set to true, no trailing EOL will be
    ///     appended at the end of the file.
    ///
    /// # Example
    ///
    /// ```
    /// use normalized_hash::Hasher;
    /// let hasher = Hasher::new();
    /// ```
    pub fn new() -> Self {
        Default::default()
    }

    /// Change the eol sequence.
    ///
    /// This string will be appended to each normalized line for hashing.
    ///
    /// Defaults to `"\n"`.
    ///
    /// # Example
    ///
    /// ```
    /// use normalized_hash::Hasher;
    /// let hasher = Hasher::new().eol("\r\n");
    /// ```
    pub fn eol(mut self, eol: impl Into<String>) -> Self {
        self.eol = eol.into();
        self
    }

    /// Ignore all whitespaces.
    ///
    /// This will remove all whitespaces from the input file when generating the hash.
    pub fn ignore_whitespaces(mut self, ignore_whitespaces: bool) -> Self {
        self.ignore_whitespaces = ignore_whitespaces;
        self
    }

    /// Skip last end-of-line on end-of-file.
    ///
    /// If this is set to true, no trailing EOL will be appended at the end of the file.
    ///
    /// Defaults to `false`.
    ///
    /// # Example
    ///
    /// ```
    /// use normalized_hash::Hasher;
    /// let hasher = Hasher::new().no_eof(true);
    /// ```
    pub fn no_eof(mut self, no_eof: bool) -> Self {
        self.no_eof = no_eof;
        self
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

        let mut is_first_line = true;
        for line in file_in.lines() {
            let line = line.unwrap();

            let line = if self.ignore_whitespaces {
                line.replace(|c: char| c.is_whitespace(), "")
            } else {
                line
            };

            let line = if !is_first_line {
                format!("{}{}", &self.eol, line)
            } else {
                line
            };

            hasher.update(&line);

            if let Some(file_out) = &mut file_out {
                file_out.write_all(line.as_bytes()).unwrap();
            }

            is_first_line = false;
        }

        if !self.no_eof {
            hasher.update(&self.eol);

            if let Some(file_out) = &mut file_out {
                file_out.write_all(&self.eol.as_bytes()).unwrap();
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
        file_with_crlf_noeof: NamedTempFile,
        file_with_lf: NamedTempFile,
        file_with_lf_noeof: NamedTempFile,

        normalized_file_with_crlf: NamedTempFile,
        normalized_file_with_crlf_noeof: NamedTempFile,
        normalized_file_with_lf: NamedTempFile,
        normalized_file_with_lf_noeof: NamedTempFile,
    }

    impl TestEnv {
        fn new() -> Result<Self, std::io::Error> {
            let mut file_with_crlf = NamedTempFile::new()?;
            let mut file_with_crlf_noeof = NamedTempFile::new()?;
            let mut file_with_lf = NamedTempFile::new()?;
            let mut file_with_lf_noeof = NamedTempFile::new()?;

            let normalized_file_with_crlf_noeof = NamedTempFile::new()?;
            let normalized_file_with_crlf = NamedTempFile::new()?;
            let normalized_file_with_lf_noeof = NamedTempFile::new()?;
            let normalized_file_with_lf = NamedTempFile::new()?;

            let content = vec!["A B", "C D"];

            file_with_crlf.write_all(content.join("\r\n").add("\r\n").as_bytes())?;
            file_with_crlf_noeof.write_all(content.join("\r\n").as_bytes())?;
            file_with_lf.write_all(content.join("\n").add("\n").as_bytes())?;
            file_with_lf_noeof.write_all(content.join("\n").as_bytes())?;

            Ok(TestEnv {
                file_with_crlf,
                file_with_crlf_noeof,
                file_with_lf,
                file_with_lf_noeof,

                normalized_file_with_crlf,
                normalized_file_with_crlf_noeof,
                normalized_file_with_lf,
                normalized_file_with_lf_noeof,
            })
        }

        fn get_input_files(&self) -> Vec<&NamedTempFile> {
            vec![
                &self.file_with_crlf,
                &self.file_with_crlf_noeof,
                &self.file_with_lf,
                &self.file_with_lf_noeof,
            ]
        }

        fn get_output_files(&self) -> Vec<&NamedTempFile> {
            vec![
                &self.normalized_file_with_crlf,
                &self.normalized_file_with_crlf_noeof,
                &self.normalized_file_with_lf,
                &self.normalized_file_with_lf_noeof,
            ]
        }

        fn hash_files(&self, hasher: &Hasher) -> Result<(String, String), Box<dyn Error>> {
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

            let (Some(hash_check), Some(content_check)) = (hash_check, content_check) else {
                unreachable!()
            };

            Ok((hash_check, content_check))
        }
    }

    #[test]
    fn check_empty_file() -> Result<(), Box<dyn Error>> {
        let file = NamedTempFile::new()?;

        // Sanity check between hasher versions

        // Completely empty file
        let hash_expected = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";
        let hash_actual = Hasher::new().eol("").hash_file(&file, None::<OsString>);
        assert_eq!(hash_actual, hash_expected);

        // Empty file ending in LF
        let hash_expected = "01ba4719c80b6fe911b091a7c05124b64eeece964e09c058ef8f9805daca546b";
        let hash_actual = Hasher::new().hash_file(&file, None::<OsString>);
        assert_eq!(hash_actual, hash_expected);

        Ok(())
    }

    #[test]
    fn check_default_options() -> Result<(), Box<dyn Error>> {
        let test_env = TestEnv::new()?;
        let (_, normalized_content) = test_env.hash_files(&Hasher::new())?;

        assert_eq!(
            fs::read_to_string(&test_env.file_with_lf)?,
            normalized_content,
            "Normalized files do not have LF"
        );

        Ok(())
    }

    #[test]
    fn check_with_custom_eol() -> Result<(), Box<dyn Error>> {
        let test_env = TestEnv::new()?;
        let (_, normalized_content) = test_env.hash_files(&Hasher::new().eol("\r\n"))?;

        assert_eq!(
            fs::read_to_string(&test_env.file_with_crlf)?,
            normalized_content,
            "Normalized files do not have CRLF"
        );

        Ok(())
    }

    #[test]
    fn check_without_eof() -> Result<(), Box<dyn Error>> {
        let test_env = TestEnv::new()?;
        let (_, normalized_content) = test_env.hash_files(&Hasher::new().no_eof(true))?;

        assert_eq!(
            fs::read_to_string(&test_env.file_with_lf_noeof)?,
            normalized_content,
            "Normalized files do not have LF without EOF"
        );

        Ok(())
    }

    #[test]
    fn check_ignore_spaces() -> Result<(), Box<dyn Error>> {
        let test_env = TestEnv::new()?;
        let hasher = Hasher::new().eol("").ignore_whitespaces(true).no_eof(true);
        let (normalized_hash, normalized_content) = test_env.hash_files(&hasher)?;

        let mut file_with_lf_without_spaces = NamedTempFile::new()?;
        let normalized_file_with_lf_without_spaces = NamedTempFile::new()?;

        file_with_lf_without_spaces.write_all("ABCD".as_bytes())?;

        let hash = hasher.hash_file(
            &file_with_lf_without_spaces,
            Some(normalized_file_with_lf_without_spaces),
        );

        assert_eq!(hash, normalized_hash, "Hashes don't match");
        assert_eq!(
            fs::read_to_string(&file_with_lf_without_spaces)?,
            normalized_content,
            "Normalized files do not ignore white spaces"
        );

        Ok(())
    }
}
