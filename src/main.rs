/*
 * grepr: Search for files matching 'file patterns' and then search these files for 'pattern'.
 * Kind of like find ... -exec grep on UNIX and findstr /s on Windows
 */

/* 
 * TODO (maybe)
 * add -f --file option to be able to specify files rather than patterns
 * add -c --colour colour option to display the matched text in green
 */

use clap::{App, Arg, ArgMatches};
use regex::{Regex, RegexBuilder};
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
    display_count: bool,
}


fn ext_to_vec(i: clap::Values) -> Vec<Regex> {
    let pattern = String::from("\\.(");
    let pattern = pattern + &(i.collect::<Vec<&str>>().join("|")) + ")$";
    match Regex::new(&pattern) {
        Ok(r) => vec![r],
        Err(err) => {
             eprintln!("grepr: Error in ext pattern: {}", err);
             std::process::exit(BAD_FILE_PATTERN);
        }
    }
}

/* Handle command line arguments and create all Regex objects */
        
impl GrepOptions {
    fn new(matches: ArgMatches) -> Self {
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
        Self {
            files: match matches.values_of("file") {
                       Some(v) => v.map(pattern_closure).collect(),
                       None => match matches.values_of("ext") {
                               Some(e) => ext_to_vec(e),
                               None => vec![],
                           }
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
            display_count: matches.is_present("count"),
        }
    }
}

/*
 * 
 */

fn main() {
    let options = parse_command_line();

    if options.display_pattern {
        eprintln!("grepr: Regex is {:?}", &options.regex);
    }

    let mut count = 0;
    if options.files.is_empty() {
        count = search_file(&options, io::stdin().lock(), PathBuf::new(), true);
    } else {

        let files = find_files(&options.files);
        if options.display_pattern {
            eprintln!("grepr: Number of files to search is {}", files.len());
        }
        let single_file = files.len() == 1;

        for file_name in files {
            let file = match File::open(&file_name) {
                Ok(r) => r,
                Err(err) => {
                    eprintln!("grepr: Can't open file {} - {}", file_name.to_string_lossy(), err);
                    std::process::exit(OPEN_FILE_ERROR);
                }
           };
           count += search_file(&options, BufReader::new(file), file_name, single_file);
        }
    }
    if options.display_count {
        println!("{}", count);
    }
    if count == 0 {
        std::process::exit(PATTERN_NOT_FOUND);
    }
}

/*
 * Parse command line arguments and return GrepOptions
 */

fn parse_command_line() -> GrepOptions {
    let matches = App::new("grepr")
        .version("1.0.0")
        .author("Steven Lalewicz 02-2023")
        .about("A simple rgrep using Rust regular expressions\n\
               https://docs.rs/regex/latest/regex/#syntax")
        .arg(
            Arg::with_name("count")
                .help("Only display the number of files which match the pattern")
                .short("n")
                .long("number")
        )
        .arg(
            Arg::with_name("display")
                .help("Only display filenames of files which match pattern")
                .short("d")
                .long("display")
        )
        .arg(
            Arg::with_name("ignore")
                .help("Ignore case for pattern")
                .short("i")
                .long("ignore")
        )
        .arg(
            Arg::with_name("verbose")
                .help("Show the pattern and file count")
                .short("v")
                .long("verbose")
        )
        .arg(
            Arg::with_name("ext")
            .help("file extension: -e rs py")
            .value_name("EXT")
            .short("e")
            .takes_value(true)
            .multiple(true)
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
                  \nIf no file pattern is specified then read from stdin.\n\
                  Otherwise automatically scan the current and all sub directories for\n\
                  the file pattern skipping any directory starting with a period.\n\
                  \nTo search all Rust and Python files in current and subdirectories for purple:\n\
                  grepr purple \"\\.(rs|py)$\"\n\
                  \nTo search all .txt regardless of case:\n\
                  grepr purple \"(?i)\\.txt$\"\n")
            .multiple(true)
            .index(2)
         )
        .get_matches();

    GrepOptions::new(matches)
}

/*
 * Search a file/stdin for the regex pattern and return a count of number of matches
 */

fn search_file<R>(
    options: &GrepOptions,
    reader: R,
    filename: PathBuf,
    single_file: bool,
) -> u32
where
    R: BufRead,
{
    let mut count = 0;
    for line_result in reader.lines() {
        let line = match line_result {
            Ok(r) => r,
            Err(err) => {
                eprintln!("grepr: Problem reading from {} - {}", filename.to_string_lossy(), err);
                break;
            }
        };
        if options.regex.is_match(&line) {
            count+=1;
            if options.display_count {
                break;
            } else if options.display_filename {
                println!("{}", filename.to_string_lossy());
                break;
            } else if single_file {
                println!("{}", line);
            } else {
                println!("{}: {}", filename.to_string_lossy(), line);
            }
        }
    }
    count
}

/* 
 * Return a vector of PathBuf for all the files we want to search
 * Uses walkdir crate to iterate directories
 */

fn find_files(res: &Vec<Regex>) -> Vec<PathBuf>
{
    /* 
     * Used for walkdir filter_entry 
     * Skip directories which start with a period
     * skip files which don't match one of the list of regular expressions
     */

    let name_filter = |entry: &DirEntry| {
        if entry.file_type().is_file() {
            res.iter().any(|re| re.is_match(&entry.file_name().to_string_lossy()))
         } else if entry.file_type().is_dir(){
                !entry.file_name().to_string_lossy().starts_with('.')
            } else {
                true
            }
        
    };

    let current_dir = env::current_dir().unwrap();

    /*
     * Used for filter_map
     * Skip entries which gave an error (If I can't access it I'm not worried)
     * Don't add directories to the final vector of PathBuf
     * Return the relative path of the file to the current directory
     */

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
