mod generation_logic;
mod cli;
mod database;
use std::{env};
fn main() {
    println!("Hello, world!");
    println!("Enter a password");
    let mut args: Vec<String> = env::args().collect();
    // remember that args[0] will be the path to the executable
    for arg in &args {
        println!("{}", arg);
    }
    let conn = database::create_connection();
    match  database::initialize_db(&conn) {
        Ok(_) => {
            println!("database initialization successful" );
        }
        Err(x) => {
            println!("something went wrong when initializing the database:");
            println!("{}", x)
        }
    }
    if args.len() >= 2{ // the first arg is always the executable path
        args.remove(0);
        if args.contains(&String::from("--help")) {
            println!("help arg detected");
            //TODO: better way to handle detection of the help option
        }
        else{


        }
        let parsed_arguments = cli::parse_args(args);
        // TODO: tests (ex. cargo run --minlength 8)
        for arg in &parsed_arguments{
            println!("{:?}", arg);
        }
        let generation_features = cli::construct_features_from_arguments(parsed_arguments);
        let password = generation_features.generate_password();
        println!("printing the newly generated password: ");
        println!("{}", password);
    }
    else{
        println!("No options provided. Creating a password using default settings...");
    }
}
