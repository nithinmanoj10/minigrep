use colored::*;
use std::error::Error;
use std::{fs, vec};
use std::collections::HashMap;

pub struct Arguments {
    pub query: String,      // Word to be searched for in the file
    pub file_name: String,  // Name of file to be searched
    pub help_option: bool,  // Option set to see the help menu
    pub view_version: bool, // Option set to view minigrep version
    pub case_ignore: bool,  // Option set to ignore cases
    pub line_number: bool,  // Option to view line numbers
    pub query_count: bool,  // Option to view count of query occurences
    pub non_match: bool,    // Option to view non matching lines
    pub line_count: bool,   // Option to view total line count containing query
    pub view_stats: bool,   // Option to view file stats
}

pub struct LineInfo<'a> {
    pub line_number: usize,
    pub line_content: &'a str,
}

pub struct SearchResult<'a> {
    pub line_info: Vec<LineInfo<'a>>,
    pub count: usize,
    pub line_count: usize,
}

impl Default for Arguments {
    fn default() -> Self {
        Arguments {
            query: Default::default(),
            file_name: Default::default(),
            help_option: false,
            view_version: false,
            case_ignore: false,
            line_number: false,
            query_count: false,
            non_match: false,
            line_count: false,
            view_stats: false,
        }
    }
}

impl Arguments {
    // new() constructor for struct Arguments
    pub fn new(args: &[String]) -> Result<Arguments, &'static str> {
        // if it contains only two arguments

        let error_message = Err("Unknown command, run 'cargo new minigrep_help' to learn more\n");

        if args.len() == 2 {
            // Possible command: cargo run minigrep_help
            if args[1].eq("minigrep_help") {
                return Ok(Arguments {
                    help_option: true,
                    ..Default::default()
                });
            }
        } else if args.len() == 3 {
            if args[1].eq("minigrep") {
                // possible command cargo run minigrep -v or
                // cargo run minigrep --version
                if args[2].eq("-v") || args[2].eq("--version") {
                    return Ok(Arguments {
                        view_version: true,
                        ..Default::default()
                    });
                }
            } 
            else if args[2].eq("-S") || args[2].eq("--stats") {
                // possible command:
                // cargo run war_and_peace.txt -S
                return Ok(Arguments {
                    file_name: args[1].clone(),
                    view_stats: true,
                    ..Default::default()  
                });
            }
            else {
                //possible command:
                // cargo run hello hello_world.txt
                return Ok(Arguments {
                    query: args[1].clone(),
                    file_name: args[2].clone(),
                    ..Default::default()
                });
            }
        } else if args.len() >= 4 {
            // possible command:
            // cargo run hello hello_world.txt -i -n

            // get the list of options
            let option_list = &args[3..];

            let mut argument = Arguments {
                query: args[1].clone(),
                file_name: args[2].clone(),
                ..Default::default()
            };

            if option_list.contains(&"-i".to_string())
                || option_list.contains(&"--ignore-case".to_string())
            {
                argument.case_ignore = true;
            }

            if option_list.contains(&"-n".to_string())
                || option_list.contains(&"--line-number".to_string())
            {
                argument.line_number = true;
            }

            if option_list.contains(&"-c".to_string())
                || option_list.contains(&"--query-count".to_string())
            {
                argument.query_count = true;
            }

            if option_list.contains(&"-lc".to_string())
                || option_list.contains(&"--line-count".to_string())
            {
                argument.line_count = true;
            }

            if option_list.contains(&"-I".to_string())
                || option_list.contains(&"--invert-match".to_string())
            {
                argument.non_match = true;
            }

            return Ok(argument);
        }

        return error_message;
    }
}

// Logic behind the whole program
pub fn run(arguments: Arguments) -> Result<(), Box<dyn Error>> {
    // if the user wants to view the help menu
    if arguments.help_option {
        view_help_menu();
        return Ok(());
    }

    // if the user wants to view the version
    if arguments.view_version {
        view_version();
        return Ok(());
    }

    // if the user wants to view file stats
    if arguments.view_stats {
        view_file_stats(arguments);
        return Ok(());
    }

    let contents = fs::read_to_string(&arguments.file_name)?;
    println!("\n{}", format!("{}:", &arguments.file_name).yellow());

    let search_result = if arguments.case_ignore {
        search_case_insensitive(&arguments.query, &contents)
    } 
    else if arguments.non_match {
        search_invert_match(&arguments.query, &contents)
    }
    else {
        search(&arguments.query, &contents)
    };

    let lines = search_result.line_info;

    if lines.len() == 0 {
        println!("No results");
    }

    // Printing the output with line numbers
    if arguments.line_number {
        for line in lines {
            println!("{}:  {}", format!("{}", line.line_number).cyan(), line.line_content);
        }
    } else {
        for line in lines {
            println!("{}", line.line_content);
        }
    }

    // Printing query occurence count
    if arguments.query_count {
        println!("{}", format!("\nCount: {}", search_result.count).green().bold());
    }
    else {
        println!("");
    }

    // Printing line count
    if arguments.line_count {
        println!("{}", format!("Line count: {}\n", search_result.line_count).green().bold());
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> SearchResult<'a> {
    let mut line_info: Vec<LineInfo> = vec![];
    let mut count = 0;
    let mut line_count = 0;

    // Iterating through each line of contents
    for (line_no, line) in contents.lines().into_iter().enumerate() {
        // checking if the line contains our query
        if line.contains(query) {
            line_info.push(LineInfo {
                line_number: line_no,
                line_content: line,
            });
            count += line.matches(query).count();
            line_count += 1;
        }
    }

    SearchResult { line_info, count, line_count}
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> SearchResult<'a> {
    let query = query.to_lowercase();
    let mut line_info: Vec<LineInfo> = vec![];
    let mut count = 0;
    let mut line_count = 0;

    // Iterating through each line of contents
    for (line_no, line) in contents.lines().into_iter().enumerate() {
        // checking if the line contains our query
        if line.to_lowercase().contains(&query) {
            line_info.push(LineInfo {
                line_number: line_no,
                line_content: line,
            });
            count += line.to_ascii_lowercase().matches(&query).count();
            line_count += 1;
        }
    }

    SearchResult { line_info, count, line_count}
}

pub fn search_invert_match<'a>(query: &str, contents: &'a str) -> SearchResult<'a> {
    let mut line_info: Vec<LineInfo> = vec![];
    let mut line_count = 0;

    // Iterating through each line of contents
    for (line_no, line) in contents.lines().into_iter().enumerate() {
        // checking if the line contains our query
        if !line.contains(query) {
            line_info.push(LineInfo {
                line_number: line_no,
                line_content: line,
            });
            line_count += 1;
        }
    }

    SearchResult { line_info, count: 0, line_count}
}

pub fn view_file_stats(arguments: Arguments) {
    let contents = fs::read_to_string(&arguments.file_name).expect("Unknown File");
    let contents = contents.to_lowercase();
    let words_list: Vec<&str> = contents.split_whitespace().collect();
    let mut word_count = HashMap::new();
    let mut char_count = 0;
    let mut line_count = 0;

    for _line in contents.lines().into_iter() {
        line_count += 1;
    }

    println!("\n{}\n", format!("{}", arguments.file_name).yellow().bold());

    for word in words_list {
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{}", format!("{0: <10} | {1: <10}", "Sequence", "Frequency").bold());
    println!("{}", format!("----------------------").bold());

    for (word, frequency) in word_count {
        println!("{0: <10} {1} {2: <10}", word, format!("|").bold(), frequency);
        char_count += word.len();
    }

    println!("\n{}", format!("{}: {}", "Character count", char_count).bold());
    println!("{}\n", format!("{}: {}", "Line count", line_count).bold());
}

pub fn view_help_menu() {
    println!("\nRust {} help menu\n", format!("minigrep").yellow().bold());

    println!(
        "{}: cargo run [QUERY] [FILE_NAME] [OPTION]",
        format!("Usage").cyan().bold()
    );
    println!(
        "{}: Search for QUERY i.e a word in the FILE_NAME provided",
        format!("Description").cyan().bold()
    );
    println!(
        "{}: 'cargo run hello hello_world.txt -i'\n",
        format!("Example").cyan().bold()
    );

    // ---- PATTERN SELECTION ----

    println!("{}", format!("Pattern Selection:").yellow().bold());
    println!(
        "{}\tignore case distinctions\t\t'cargo run hello hello_world.txt -i'",
        format!("   -i, --ignore-case").blue().bold()
    );

    // ---- OUTPUT CONTROL ----

    println!("{}", format!("\nOutput Control:").yellow().bold());

    // Line Number
    println!(
        "{}\tprint line numbers with output lines \t'cargo run hello hello_world.txt -n'",
        format!("   -n, --line-number").blue().bold()
    );

    // Query Occurence Count
    println!(
        "{}\toutput total occurences of query \t'cargo run butter recipe.txt -c'",
        format!("   -c, --query-count").blue().bold()
    );

    // Line Occurence Count
    println!(
        "{}\toutput total lines containing query \t'cargo run phone devices.txt -lc'",
        format!("   -lc, --line-count").blue().bold()
    );

    // Invert Match
    println!(
        "{}\toutput non-matching lines \t\t'cargo run puppy pets.txt -I'",
        format!("   -I, --invert-match").blue().bold()
    );

    // ---- MISCELLANEOUS ----

    println!("{}", format!("\nMiscellaneous:").yellow().bold());

    // Display file stats
    println!(
        "{}\t\tdisplays file statistics\t\t'cargo run war_and_peace.txt -S'",
        format!("   -S, --stats").blue().bold()
    );


    // Display version
    println!(
        "{}\tdisplays version information\t\t'cargo run minigrep -v'",
        format!("   -v, --version").blue().bold()
    );

    // Display help menu
    println!(
        "{}\tdisplays help menu\t\t\t'cargo run minigrep_help'",
        format!("   minigrep_help").blue().bold()
    );
}

pub fn view_version() {
    println!("\n{}", format!("Minigrep v1.0.1").yellow().bold());
    println!("Made by Nithin for learning purposes only");
    println!("https://github.com/nithinmanoj10/minigrep")
}
