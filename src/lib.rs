use std::error::Error;
use std::{fs, vec};
use colored::*;

pub struct Arguments {
    pub query: String,     // Word to be searched for in the file
    pub file_name: String, // Name of file to be searched
}

impl Arguments {
    // new() constructor for struct Arguments
    pub fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() != 3 {
            return Err(
                "Command doesn't have enough arguments\nExpected: cargo run <query> <file_name>\n",
            );
        }

        Ok(Arguments {
            query: args[1].clone(),
            file_name: args[2].clone(),
        })
    }
}

// Logic behind the whole program
pub fn run(arguments: Arguments) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&arguments.file_name)?;
    
    for line in search_case_insensitive(&arguments.query, &contents) {
        println!("{}:\t{}", format!("{}", line.0).cyan(), line.1);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<(usize, &'a str)> {

    let mut result: Vec<(usize, &str)> = vec![];

    // Iterating through each line of contents
    for (line_no, line) in contents.lines().into_iter().enumerate() {
        // checking if the line contains our query
        if line.contains(query) {
            result.push((line_no+1, line));
        }
    }

    result
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<(usize, &'a str)> {
    let query = query.to_lowercase();
    let mut result: Vec<(usize, &str)> = vec![];

    for (line_no, line) in contents.lines().into_iter().enumerate() {
        if line.to_lowercase().contains(&query) {
            result.push((line_no+1, line));
        }
    }
    
    result
}