# normalized-hasher

[![badge github]][url github]
[![badge crates.io]][url crates.io]
[![badge license]][url license]

[badge github]: https://img.shields.io/badge/github-FloGa%2Fnormalized--hasher-green
[badge crates.io]: https://img.shields.io/crates/v/normalized-hasher
[badge license]: https://img.shields.io/crates/l/normalized-hasher

[url github]: https://github.com/FloGa/normalized-hasher
[url crates.io]: https://crates.io/crates/normalized-hasher
[url license]: https://github.com/FloGa/normalized-hasher/blob/develop/LICENSE

Create cross-platform hashes of text files.

*This is the binary crate. If you're looking for the library crate instead, go
to [`normalized-hash`].*

[`normalized-hash`]: https://github.com/FloGa/normalized-hasher/crates/normalized-hash

## Motivation

Hashes or checksums are a great means for validating the contents of files.
You record the hash of a file, distribute the file and the hash code, and
everyone can run the hasher again to verify that the file has not changed
since you created the hash the first time. Each small change will also change
the hash code. Even if it is a change you cannot even see.

In my job, we unfortunately had this situation a couple of times. The workflow
is as follows: We create code and generate a hash from this code. Both are
inserted into a specification document. Then we copy and paste the code to a
customer's system and run the hasher again to verify that the code is still
the same as in the specification. But from time to time, we got different
hashes. After some search for the reason, we stumbled across this one coworker
who did not save their files with UNIX line endings (a single LF) like the
rest of us, but with Windows line endings (CR followed by LF). Just by looking
at the files, they seemed identical, but after enabling control characters, we
could clearly see the differences in the end of every line. By copying the
code to the customer system, the line endings get automatically converted into
UNIX style, hence the hash would be different from what we generate on our
systems. This is an embarrassing situation, because this involves huge paper
work to request a change in the already finalized specification document.

To come over this problem, I created this program. A file hasher that would
convert file endings to UNIX style on the fly when generating the hash. So, no
matter how the file was created, the hash would be the same.

## Installation

`normalized-hasher` can be installed easily through Cargo via `crates.io`:

```shell
cargo install --locked normalized-hasher
```

Please note that the `--locked` flag is necessary here to have the exact same
dependencies as when the application was tagged and tested. Without it, you
might get more up-to-date versions of dependencies, but you have the risk of
undefined and unexpected behavior if the dependencies changed some
functionalities. The application might even fail to build if the public API of
a dependency changed too much.

Alternatively, pre-built binaries can be downloaded from the [GitHub
releases][gh-releases] page.

[gh-releases]: https://github.com/FloGa/normalized-hasher/releases

## Usage

<!--% !cargo --quiet run -- --help | tail -n+3 %-->

```text
Usage: normalized-hasher [OPTIONS] <FILE_IN> [FILE_OUT]

Arguments:
  <FILE_IN>
          File to be hashed

  [FILE_OUT]
          Optional file path to write normalized input into

Options:
      --eol <EOL>
          End-of-line sequence, will be appended to each normalized line for hashing
          
          [default: "\n"]

      --no-eof
          Skip last end-of-line on end-of-file.
          
          With this flag, no trailing EOL will be appended at the end of the file.

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

### Flags

-   `--eol`
    
    With the `--eol` flag you can change the end-of-line sequence that will be
    appended to each normalized line to generate the hash. This can be useful
    if you explicitly want CRLF endings, for example.
    
    Please note that you need to escape control characters properly in your
    shell. For Bash, you can type:
    
    ```shell
    normalized-hasher --eol $'\r\n' input.txt output.txt
    ```
    
-   `--no-eof`

    With the `--no-eof` flag you can avoid appending the EOL sequence at the
    end of the file. This is for use cases where such trailing EOL is not
    desireable, like in Windows files. In contrast to UNIX files which usually
    end with a final LF, Windows files do not usually end with an additional
    CRLF.

## Examples

Simple example with default options, without writing an output file:

```shell
normalized-hasher input.txt
```

More complex example, with writing output:

```shell
normalized-hasher --eol $'\r\n' --no-eof input.txt output.txt
```
