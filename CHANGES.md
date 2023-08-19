# Changes since latest release

# Changes in 0.2.0

-   Move library part to separate crate

-   Add eol flag

    With the `--eol` flag you can change the end-of-line sequence that will
    be appended to each normalized line to generate the hash. This can be
    useful if you explicitly want CRLF endings, for example.

    Please note that you need to escape control characters properly in your
    shell. For Bash, you can type:

    ```shell
    normalized-hasher --eol $'\r\n' input.txt output.txt
    ```

-   Add no-eof flag

    With the `--no-eof` flag you can avoid appending the EOL sequence at the
    end of the file. This is for use cases where such trailing EOL is not
    desireable, like in Windows files. In contrast to UNIX files which
    usually end with a final LF, Windows files do not usually end with an
    additional CRLF.

-   Add ignore-whitespaces flag

    In some extreme cases, you might want to ignore all whitespaces in a
    file. With the `--ignore-whitespaces` flag, all whitespaces are removed
    prior to generate the hash.

# Changes in 0.1.0

Initial release.
