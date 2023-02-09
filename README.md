# normalized-hasher

[![badge github]][url github]
[![badge crates.io]][url crates.io]
[![badge docs.rs]][url docs.rs]
[![badge license]][url license]

[badge github]: https://img.shields.io/badge/github-FloGa%2Fnormalized--hasher-green
[badge crates.io]: https://img.shields.io/crates/v/normalized-hasher
[badge docs.rs]: https://img.shields.io/docsrs/normalized-hasher
[badge license]: https://img.shields.io/crates/l/normalized-hasher

[url github]: https://github.com/FloGa/normalized-hasher
[url crates.io]: https://crates.io/crates/normalized-hasher
[url docs.rs]: https://docs.rs/normalized-hasher
[url license]: https://github.com/FloGa/normalized-hasher/blob/develop/LICENSE

Create cross-platform hashes of text files.

## Motivation

TBD

## Installation

`normalized-hasher` can be installed easily through Cargo via `crates.io`:

```shell script
cargo install --locked normalized-hasher
```

Please note that the `--locked` flag is necessary here to have the exact same
dependencies as when the application was tagged and tested. Without it, you
might get more up-to-date versions of dependencies, but you have the risk of
undefined and unexpected behavior if the dependencies changed some
functionalities. The application might even fail to build if the public API of
a dependency changed too much.

## Usage

<!--% !cargo --quiet run -- --help | tail -n+3 %-->

```text
Usage: normalized-hasher <FILE_IN> [FILE_OUT]

Arguments:
  <FILE_IN>   File to be hashed
  [FILE_OUT]  File to write normalized input into

Options:
  -h, --help     Print help
  -V, --version  Print version
```
