use minigrep::Arguments;
use std::env;
use std::process;

fn main() {
    // Accepting user inputed arguments from the command line
    let arg_list: Vec<String> = env::args().collect();
    let arguments = Arguments::new(&arg_list).unwrap_or_else(|err| {
        println!("Error: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(arguments) {
        println!("Error: {}", e);
        process::exit(1);
    }
}
