use clap::{App, Arg, ArgMatches};
use regex::Regex;
use std::io::{self, BufReader, BufRead};
use std::fs::File;
use std::path::Path;
use glob::glob;

const PATTERN_NOT_FOUND: i32 = 1;
const BAD_PATTERN: i32 = 2;
const BAD_GLOB_PATTERN: i32 = 3;
const OPEN_FILE_ERROR: i32 = 4;

struct GrepOptions {
    pattern          : String,
    files            : Vec<String>,
    ignore_case      : bool,
    display_pattern  : bool,
    display_filename : bool,
}

impl GrepOptions {

    fn new(matches: ArgMatches) -> GrepOptions {
        GrepOptions {
            pattern : match matches.value_of("pattern") {
                Some(v) => v.to_string(),
                None => "".to_string()
            },

            files : match matches.values_of("file") {
                Some(v) => v.map(String::from).collect(),
                None => vec![],
             },
             
             ignore_case      : matches.is_present("ignore"),
             display_pattern  : matches.is_present("verbose"),
             display_filename : matches.is_present("display"),
         }
    }
}

/* -------------------------------------------------------------- */

fn main() {

    let options = parse_command_line();

    let pattern;
    if options.ignore_case {
        pattern = "(?i)".to_string() + &options.pattern;
    } else {
        pattern = options.pattern;
    }
    let reg = match Regex::new(&pattern) {
        Ok(r) => r,
        Err(err) => {
            eprintln!("grepr: Error in pattern: {}", err);
            std::process::exit(BAD_PATTERN);
        }
    };
 
    if options.display_pattern {
        println!("grepr: Regex is {:?} File count is {}", reg, options.files.len());
    }

    let mut found = false;
    if options.files.is_empty() {
        found = search_file(&reg, io::stdin().lock(), false, "stdin", true);
    } else {
        let mut single_file = options.files.len() == 1;
        for name in &options.files {
            let gfiles = match glob(name) {
                Err(err) => {
                    eprintln!("grepr: Pattern Error {:?}", err);
                    std::process::exit(BAD_GLOB_PATTERN);
                },
                Ok(g) => g
            };
            let mut count = 0;
            for entry in gfiles {
                let file_name = entry.unwrap();
                let file_name = file_name.to_str().unwrap();
                   /* if file_name != name then a pattern got expanded so multi files */
                if count == 0 && file_name != name {
                    single_file = false;
                }
                let path = Path::new(&file_name);
                if !single_file {
                    if path.is_dir() {
                        continue;
                    }
                }
                count+=1;
                // let f = match File::open(file_name) {
                let f = match File::open(path) {
                    Ok(r) => r,
                    Err(err) => {
                        eprintln!("grepr: Can't open file {} - {}", file_name, err);
                        std::process::exit(OPEN_FILE_ERROR);
                    }
                };
                found = search_file(&reg, BufReader::new(f), options.display_filename, file_name , single_file);
            };
            if count == 0 {
                eprintln!("grepr: {} not found", name);
            }
        };
    };
    if !found {
        std::process::exit(PATTERN_NOT_FOUND);
    }
}

fn parse_command_line() -> GrepOptions
{
    let matches = App::new("grepr")
        .version("1.0.0")
        .author("Steven Lalewicz")
        .about("A simple grep using Rust regular expressions\nhttps://docs.rs/regex/latest/regex/#syntax")
        .arg(
            Arg::with_name("ignore")
                .help("Ignore case")
                .short("i")
                .long("ignore")
        )
        .arg(
            Arg::with_name("display")
                .help("Only display filenames of files which match pattern")
                .short("d")
                .long("display")
        )
        .arg(
            Arg::with_name("verbose")
                .help("Show the pattern")
                .short("v")
                .long("verbose")
        )
        .arg(
            Arg::with_name("pattern")
            .help("Rust regular expression")
            .required(true)
            .index(1)
         )
        .arg(
            Arg::with_name("file")
            .help("List of files. Glob pattens allowed\nIf no file specified \
             then read from stdin\nSearch all Rust files in current and subdirectories for \
             purple:\ngrepr purple \"**/*.rs\"\n\
             https://docs.rs/glob/0.3.1/glob/struct.Pattern.html")
            .multiple(true)
            .index(2)
         )
        .get_matches();

    GrepOptions::new(matches)
}

fn search_file <R>(reg: &Regex, reader: R, display_filename: bool, filename: &str, single_file: bool) -> bool where R: BufRead
{
    let mut found = false;
    for line_result in reader.lines() {
        let line = match line_result {
            Ok(r) => r,
            Err(err) => {
                eprintln!("grepr: Problem reading from {} - {}", filename, err);
                break;
            }
        };
        if reg.is_match(&line) {
            found = true;
            if display_filename {
                println!("{}", filename);
                break;
            } else if single_file {
                println!("{}", line);
            } else {
                println!("{}: {}", filename, line);
            }
        }
    }
    found
}
