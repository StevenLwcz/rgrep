/* TODO
 * add -c count option
 * add -f option to be able to specify files rather than patterns
 */

use clap::{App, Arg, ArgMatches};
use regex::Regex;
use regex::RegexBuilder;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::PathBuf;
use std::env;
use walkdir::{DirEntry, WalkDir, Error};


const PATTERN_NOT_FOUND: i32 = 1;
const BAD_PATTERN: i32 = 2;
const BAD_FILE_PATTERN: i32 = 3;
const OPEN_FILE_ERROR: i32 = 4;

struct GrepOptions {
    regex: Regex,
    files: Vec<Regex>,
    display_pattern: bool,
    display_filename: bool,
}

impl GrepOptions {
    fn new(matches: ArgMatches) -> GrepOptions {
        let pattern_closure = | file | {
            match Regex::new(file) {
                Ok(r) => r,
                Err(err) => {
                    eprintln!("grepr: Error in file pattern: {}", err);
                    std::process::exit(BAD_FILE_PATTERN);
                }
            }
        };
        
        let pattern = matches.value_of("pattern").unwrap();
        GrepOptions {
            files: match matches.values_of("file") {
                Some(v) => v.map(pattern_closure).collect(),
                None => vec![],
            },
            regex: match RegexBuilder::new(pattern)
                .case_insensitive(matches.is_present("ignore"))
                .build() {
                    Ok(r) => r,
                    Err(err) => {
                        eprintln!("grepr: Error in pattern: {}", err);
                        std::process::exit(BAD_PATTERN);
                    }
            },

            display_pattern: matches.is_present("verbose"),
            display_filename: matches.is_present("display"),
        }
    }
}

/* -------------------------------------------------------------- */

fn main() {
    let options = parse_command_line();


    if options.display_pattern {
        eprintln!(
            "grepr: Regex is {:?} File count is {}",
            &options.regex,
            options.files.len()
        );
    }

    let mut found = false;
    if options.files.is_empty() {
        found = search_file(&options.regex, io::stdin().lock(), false, PathBuf::new() , true);
    } else {
        let files = find_files(options.files);
        let single_file = files.len() == 1;
        for file_name in files {
            let file = match File::open(&file_name) {
                Ok(r) => r,
                Err(err) => {
                    eprintln!("grepr: Can't open file {} - {}", file_name.to_string_lossy(), err);
                    std::process::exit(OPEN_FILE_ERROR);
                }
           };
           found = search_file(&options.regex, BufReader::new(file), options.display_filename, 
                        file_name, single_file);
        }
    }
    if !found {
        std::process::exit(PATTERN_NOT_FOUND);
    }
}

fn parse_command_line() -> GrepOptions {
    let matches = App::new("grepr")
        .version("1.0.0")
        .author("Steven Lalewicz")
        .about("A simple grep using Rust regular expressions\n\
               https://docs.rs/regex/latest/regex/#syntax")
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
            .help("List of file patterns. The pattern is a Rust regular expression.\n\
                  If no pattern is specified then read from stdin.\n\
                  To search all Rust and Python files in current and subdirectories for purple:\n\
                  grepr purple \".*.rs$ .*.py$\"\n")
            .multiple(true)
            .index(2)
         )
        .get_matches();

    GrepOptions::new(matches)
}

fn search_file<R>(
    reg: &Regex,
    reader: R,
    display_filename: bool,
    filename: PathBuf,
    single_file: bool,
) -> bool
where
    R: BufRead,
{
    let mut found = false;
    for line_result in reader.lines() {
        let line = match line_result {
            Ok(r) => r,
            Err(err) => {
                eprintln!("grepr: Problem reading from {} - {}", filename.to_string_lossy(), err);
                break;
            }
        };
        if reg.is_match(&line) {
            found = true;
            if display_filename {
                println!("{}", filename.to_string_lossy());
                break;
            } else if single_file {
                println!("{}", line);
            } else {
                println!("{}: {}", filename.to_string_lossy(), line);
            }
        }
    }
    found
}

fn find_files(res: Vec<Regex>) -> Vec<PathBuf>
{
    let name_filter = |entry: &DirEntry| {
        if entry.file_type().is_file() {
            res.iter().any(|re| re.is_match(&entry.file_name().to_string_lossy()))
        } else {
            if entry.file_type().is_dir(){
                !entry.file_name().to_string_lossy().starts_with(".")
            } else {
                true
            }
        }
    };

    let current_dir = env::current_dir().unwrap();

    let dir_filter = | entry: Result<DirEntry,Error>| {
        match entry {
            Ok(entry) => {
                if entry.file_type().is_file() {
                    Some(entry.path().strip_prefix(&current_dir).unwrap()
                                     .to_path_buf())
                } else {
                    None
                }
            },
            Err(_err) => None,
       }
    };
    
    WalkDir::new(&current_dir)
        .into_iter()
        .filter_entry(name_filter)
        .filter_map(dir_filter)
        .collect()
}
