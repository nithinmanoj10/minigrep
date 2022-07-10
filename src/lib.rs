use colored::*;
use std::error::Error;
use std::{fs, vec};

pub struct Arguments {
    pub query: String,     // Word to be searched for in the file
    pub file_name: String, // Name of file to be searched
    pub help_option: bool, // Option set to see the help menu
    pub view_version: bool, // Option set to view minigrep version
}

impl Arguments {
    // new() constructor for struct Arguments
    pub fn new(args: &[String]) -> Result<Arguments, &'static str> {
        // if it contains only two arguments
        
        if args.len() == 2 {
            // Possible command: cargo run minigrep_help
            if args[1].eq("minigrep_help") {
            return Ok(Arguments {
                query: Default::default(),
                file_name: Default::default(),
                help_option: true,
                view_version: false,
            });}
        }

        if args.len() == 3 {
            if args[1].eq("minigrep") {
                // possible command cargo run minigrep -v or
                // cargo run minigrep --version
                if args[2].eq("-v") || args[2].eq("--version") {
                    return Ok(Arguments {
                      query: Default::default(),
                      file_name: Default::default(),
                      help_option: false,
                      view_version: true,  
                    })
                }
            }
        }

        if args.len() < 2 {
            return Err(
                "Unknown command, run 'cargo new minigrep_help' to learn more\n",
            );
        }

        Ok(Arguments {
            query: args[1].clone(),
            file_name: args[2].clone(),
            help_option: false,
            view_version: false,
        })
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

    let contents = fs::read_to_string(&arguments.file_name)?;
    println!("\n{}", format!("{}:", &arguments.file_name).yellow());
    for line in search_case_insensitive(&arguments.query, &contents) {
        println!("{}:\t{}", format!("{}", line.0).cyan(), line.1);
    }
    println!("");

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<(usize, &'a str)> {
    let mut result: Vec<(usize, &str)> = vec![];

    // Iterating through each line of contents
    for (line_no, line) in contents.lines().into_iter().enumerate() {
        // checking if the line contains our query
        if line.contains(query) {
            result.push((line_no + 1, line));
        }
    }

    result
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<(usize, &'a str)> {
    let query = query.to_lowercase();
    let mut result: Vec<(usize, &str)> = vec![];

    for (line_no, line) in contents.lines().into_iter().enumerate() {
        if line.to_lowercase().contains(&query) {
            result.push((line_no + 1, line));
        }
    }

    result
}

pub fn view_help_menu() {
    println!("\nRust {} help menu\n", format!("minigrep").yellow().bold());

    println!("{}: cargo run [OPTION] [QUERY] [FILE_NAME]", format!("Usage").cyan().bold());
    println!("{}: Search for QUERY i.e a word in the FILE_NAME provided", format!("Description").cyan().bold());
    println!("{}: cargo run -i hello hello_world.txt\n", format!("Example").cyan().bold());

    println!("{}", format!("Miscellaneous:").yellow().bold());
    println!("{}\tdisplays version information", format!("   -v, --version").blue().bold());
}

pub fn view_version() {
    println!("\n{}", format!("Minigrep v1.0.1").yellow().bold());
    println!("Made by Nithin for learning purposes only");
    println!("https://github.com/nithinmanoj10/minigrep")
}