# Changes in 0.1.0

-   Create crate from library part of normalized-hasher

-   Create struct for hashing

    This is a preparation action for better configurability.

-   Make eol configurable

    Some people might prefer Windows line endings (CRLF) instead of UNIX
    (LF). Maybe some might even prefer to have no line ending at all.
    Whatever the requirements might be, the line ending is something that
    should be configurable.

-   Make eof configurable

    On UNIX system, there is usually an LF as the last character of the
    file. On Windows systems, this is usually not the case. To handle use
    cases where this EOF character might be important, it is now
    configurable wether to include such a last EOL on EOF.

-   Support ignoring whitespaces

    There might be extreme use cases where you don't just want to have
    consistent line endings, but you don't even want to consider any other
    whitespaces in a file. With this new configuration option, all
    whitespaces are removed from the normalized content when generating the
    hash.
