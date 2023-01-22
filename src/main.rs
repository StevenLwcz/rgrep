/* TODO
 * filename have wild cards
 * directory search
 * ignore case
 */
use clap::{App, Arg, ArgMatches};
use regex::Regex;
use std::io::{self, BufReader, BufRead};
use std::fs::File;

struct GrepOptions {
    pattern          : String,
    files            : Vec<String>,
    search_subdir    : bool,
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
             
             search_subdir    : matches.is_present("subdir"),
             ignore_case      : matches.is_present("ignore"),
             display_pattern  : matches.is_present("verbose"),
             display_filename : matches.is_present("display"),
         }
    }
}

fn main() {

    let options = parse_command_line();

    let reg = match Regex::new(&options.pattern) {
        Ok(r) => r,
        Err(err) => {
            println!("rgrep: Error in pattern: {}", err);
            std::process::exit(1);
        }
    };
 
    if options.display_pattern {
        println!("rgrep: Regex is {:?}", reg);
    }

    if options.files.is_empty() {
        search_file(&reg, io::stdin().lock(), false, &"stdin".to_string(), true);
    } else {
        let single_file = options.files.len() == 1;
        for name in &options.files {
            let f = match File::open(name) {
                Ok(r) => r,
                Err(err) => {
                    println!("rgrep: Can't open file {} {}", name, err);
                    std::process::exit(1);
                }
            };
            search_file(&reg, BufReader::new(f), options.display_filename, name, single_file);
        };
    };
}

fn parse_command_line() -> GrepOptions
{
    let matches = App::new("rgrep")
        .version("0.1.0")
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
            Arg::with_name("subdir")
                .help("Search sub directories")
                .short("s")
                .long("subdir")
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
            .help("List of files. Wildcards allowed\nIf no file specified, then read from stdin")
            .multiple(true)
            .index(2)
         )
        .get_matches();

    GrepOptions::new(matches)
}

fn search_file<R>(reg: &Regex, reader: R, display_filename: bool, filename: &String, single_file: bool) where R: BufRead
{
    for line_result in reader.lines() {
        let line = match line_result {
            Ok(r) => r,
            Err(err) => {
                println!("rgrep: Problem reading from {} {}", filename, err);
                break;
            }
        };
        if reg.is_match(&line) {
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
}
