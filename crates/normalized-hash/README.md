# normalized-hash

[![badge github]][url github]
[![badge crates.io]][url crates.io]
[![badge docs.rs]][url docs.rs]
[![badge license]][url license]

[badge github]: https://img.shields.io/badge/github-FloGa%2Fnormalized--hasher-green
[badge crates.io]: https://img.shields.io/crates/v/normalized-hash
[badge docs.rs]: https://img.shields.io/docsrs/normalized-hash
[badge license]: https://img.shields.io/crates/l/normalized-hash

[url github]: https://github.com/FloGa/normalized-hasher/crates/normalized-hash
[url crates.io]: https://crates.io/crates/normalized-hash
[url docs.rs]: https://docs.rs/normalized-hash
[url license]: https://github.com/FloGa/normalized-hasher/blob/develop/crates/normalized-hash/LICENSE

Cross-platform hash algorithm.

*This is the library crate. If you're looking for the binary crate instead, go
to [`normalized-hasher`].*

[`normalized-hasher`]: https://github.com/FloGa/normalized-hasher

## Summary

This hashing algorithm allows consistent hashes even if you accidentally
convert a file from using UNIX line endings (LF) to Windows line endings
(CRLF). For a longish motivational speech about how such a thing can happen
and why you should want to even care about such a case, head over to
[`normalized-hasher`].

## Code Example

```rust no_run
use std::path::PathBuf;

use normalized_hash::Hasher;

fn main() {
    let file_in = PathBuf::from("input.txt");
    let file_out = PathBuf::from("output.txt");

    // Simple example with default options, without writing an output file
    let hash = Hasher::new().hash_file(&file_in, None::<PathBuf>);
    println!("{}", hash);

    // More complex example, with writing output
    let hash = Hasher::new()
        .eol("\r\n")
        .no_eof(true)
        .hash_file(&file_in, Some(file_out));
    println!("{}", hash);
}
```
