A simple version of grep written in Rust using Rust regular expressions.

I'm learning Rust. What else does one write for their first app? :)

    $ grepr --help

    grepr 1.0.0
    Steven Lalewicz
    A simple rgrep using Rust regular expressions
    https://docs.rs/regex/latest/regex/#syntax

    USAGE:
        grepr [FLAGS] <pattern> [file]...

    FLAGS:
        -d, --display    Only display filenames of files which match pattern
        -h, --help       Prints help information
        -i, --ignore     Ignore case for pattern
        -V, --version    Prints version information
        -v, --verbose    Show the pattern

    ARGS:
        <pattern>    Rust regular expression
        <file>...    List of file patterns. The pattern is a Rust regular expression.
                 
                     If no file pattern is specified then read from stdin.
                     Otherwise automatically scan the current and all sub directories for
                     the file pattern skipping any directory starting with a period.
                 
                     To search all Rust and Python files in current and subdirectories for purple:
                     grepr purple *\.(rs|py)$"
                 
                     To search all .txt regardless of case:
                     grepr purple "(?i)\.txt$"
