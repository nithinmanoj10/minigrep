use std::error::Error;
use std::fs;

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
    let contents = fs::read_to_string(arguments.file_name)?;
    println!("{}", contents);

    Ok(())
}
