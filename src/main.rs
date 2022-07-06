use std::env;
use std::error::Error;
use std::fs;
use std::process;

fn main() {
    // Accepting user inputed arguments from the command line
    let arg_list: Vec<String> = env::args().collect();
    let arguments = Arguments::new(&arg_list).unwrap_or_else(|err| {
        println!("Error: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(arguments) {
        println!("Error: {}", e);
        process::exit(1);
    }
}

struct Arguments {
    query: String,
    file_name: String,
}

impl Arguments {
    fn new(args: &[String]) -> Result<Arguments, &'static str> {
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

fn run(arguments: Arguments) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(arguments.file_name)?;
    println!("{}", contents);

    Ok(())
}
