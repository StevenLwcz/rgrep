A simple version of grep written in Rust using Rust regular expressions.

I'm learning Rust. What else does one write for their first app? :)

rgrep --help

  rgrep 0.1.0
  Steven Lalewicz
  A simple grep using Rust regular expressions
  https://docs.rs/regex/latest/regex/#syntax
  
  USAGE:
      rgrep [FLAGS] <pattern> [file]...
   
  FLAGS:
      -d, --display    Only display filenames of files which match pattern
      -h, --help       Prints help information
      -i, --ignore     Ignore case
      -V, --version    Prints version information
      -v, --verbose    Show the pattern
  
  ARGS:
      <pattern>    Rust regular expression
      <file>...    List of files. Glob pattens allowed
                   If no file specified then read from stdin
                   Search all Rust files in current and subdirectories for purple:
                   rgrep purple "**/*.rs"
                   https://docs.rs/glob/0.3.1/glob/struct.Pattern.html
