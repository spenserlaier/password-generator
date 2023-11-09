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
        if args.contains(&String::from("--help")) {
            println!("A command line tool to generate a random password with given parameters");
            println!("Options:");
            println!("--minlength <int> : min password length");
            println!("--include_nums <bool> : include numbers in password");
            println!("--include_spec <bool> : include special characters in password");
            println!("--include_ucase <bool> : include uppercase characters in password");
            println!("--use_dict_words <bool> : use dictionary words instead of random lowercase alphabetic characters");
            println!("--profile <string> : name of the profile to use. Will pull from local database if such a profile exists");
            println!("--overwrite <bool> : if using a profile, overwrite its current settings with the other command line options");
            std::process::exit(1)
        }
        let parsed_arguments = cli::parse_args(args);
        for arg in &parsed_arguments{
            println!("{:?}", arg);
        }
        let generation_features = cli::construct_features(Some(parsed_arguments));
        let password = generation_features.generate_password();
        println!("printing the newly generated password: ");
        println!("{}", password);
    }
    else{
        println!("No options provided. Creating a password using default settings...");
        let generation_features = cli::construct_features(None);
        let password = generation_features.generate_password();
        println!("printing the newly generated password: ");
        println!("{}", password);
    }
}
