use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;

use sha2::{Digest, Sha256};

pub fn hash_file(file_in: impl AsRef<Path>, file_out: Option<impl AsRef<Path>>) -> String {
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
        let hash_actual = hash_file(file, None::<OsString>);

        assert_eq!(hash_actual, hash_expected);

        Ok(())
    }

    #[test]
    fn check_different_eols() -> Result<(), Box<dyn Error>> {
        let mut file_with_lf = tempfile::NamedTempFile::new()?;
        let mut file_with_crlf = tempfile::NamedTempFile::new()?;

        let file_with_lf_normalized = tempfile::NamedTempFile::new()?;
        let file_with_crlf_normalized = tempfile::NamedTempFile::new()?;

        file_with_lf.write_all("A\nb".as_ref())?;
        file_with_crlf.write_all("A\r\nb".as_ref())?;

        let hash_with_lf = hash_file(file_with_lf, Some(&file_with_lf_normalized));
        let hash_with_crlf = hash_file(file_with_crlf, Some(&file_with_crlf_normalized));

        assert_eq!(hash_with_lf, hash_with_crlf);
        assert_eq!(
            fs::read_to_string(file_with_lf_normalized)?,
            fs::read_to_string(file_with_crlf_normalized)?
        );

        Ok(())
    }
}
