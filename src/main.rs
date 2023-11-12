mod generation_logic;
mod cli;
mod database;
use std::{env, process::exit};
fn main() {
    let mut args: Vec<String> = env::args().collect();
    let conn = database::create_connection();
    println!("Command line tool for password generation with additional customization and features. Use --help option for more information.");
    match  database::initialize_db(&conn) {
        Ok(_) => {
            println!("database check successful" );
        }
        Err(x) => {
            println!("something went wrong when initializing the database:");
            println!("{}", x)
        }
    }
    if args.len() >= 2{ // the first arg is always the executable path
        args.remove(0);
        let parsed_arguments = cli::parse_args(args);
        for arg in &parsed_arguments{
            println!("{:?}", arg);
        }
        let mut generation_features = cli::process_and_execute_args(Some(parsed_arguments));
        let password = generation_features.generate_password();
        println!("printing the newly generated password: ");
        println!("{}", password);
    }
    else{
        println!("No options provided. Creating a password using default settings...");
        let mut generation_features = cli::construct_features(None);
        let password = generation_features.generate_password();
        println!("printing the newly generated password: ");
        println!("{}", password);
    }
}
