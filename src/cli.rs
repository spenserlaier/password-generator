use crate::database;
use crate::generation_logic::GenerationData;
use dotenv;
use std::env;
use std::process::exit;
#[derive(PartialEq, Debug)]
pub enum ArgType {
    MinimumLength,
    IncludeNumbers,
    IncludeSpecial,
    IncludeUcase,
    UseWords,
    Overwrite,
    Profile,
    Error,
    ListProfiles,
    ProfileInfo,
    Help,
    NewProfile,
}
#[derive(PartialEq, Debug)]
pub enum ArgValue {
    Bool(bool),
    Int(usize),
    String(String),
    NoValue,
    Error,
}
#[derive(PartialEq, Debug)]
pub enum Argument {
    ParsedArgument(ArgType, ArgValue),
    Error
}
pub fn is_arg(input: &str) -> bool{
    match input {
        "--minlength" => {
            true
        },
        "--include_nums" => {
            true
        },
        "--include_spec" => {
            true
        },
        "--include_ucase" => {
            true
        },
        "--use_dict_words" => {
            true
        },
        "--overwrite" => {
            true
        }
        "--profile" => {
            true
        }
        "--help" => {
            true
        }
        "--new_profile" => {
            true
        }
        "--list_profiles" => {
            true
        }
        "--profile_info" => {
            true
        }
        _ => {
            false
        },
    }
}
pub fn parse_single_arg(arg_type: &str, arg_value: &str) -> Argument{
    let parsed_arg_value = match arg_value {
        "true" => ArgValue::Bool(true),
        "false" => ArgValue::Bool(false),
        "" => ArgValue::NoValue,
        x => {
            if let Ok(parsed_usize) = x.parse::<usize>() {
                ArgValue::Int(parsed_usize)
            }
            else{
                ArgValue::String(String::from(x))
            }
        }
    };
    let parsed_arg_type = match arg_type {
    //TODO: check for "--help" arg, and set up some documentation for options available
        "--minlength" => {
            ArgType::MinimumLength
        },
        "--include_nums" => {
            ArgType::IncludeNumbers
        },
        "--include_spec" => {
            ArgType::IncludeSpecial
        },
        "--include_ucase" => {
            ArgType::IncludeUcase
        },
        "--use_dict_words" => {
            ArgType::UseWords
        },
        "--overwrite" => {
            ArgType::Overwrite
        }
        "--profile" => {
            //TODO: check if we actually use this anywhere;
            ArgType::Profile
        }
        "--help" => {
            ArgType::Help
        }
        "--new_profile" => {
            ArgType::NewProfile
        }
        "--list_profiles" => {
            ArgType::ListProfiles
        }
        "--profile_info" => {
            ArgType::ProfileInfo
        }
        _ => {
            ArgType::Error
        },
    };
    if parsed_arg_type != ArgType::Error && parsed_arg_value != ArgValue::Error {
        Argument::ParsedArgument(parsed_arg_type, parsed_arg_value)
    }
    else{
        Argument::Error
    }
}
pub fn parse_args(args: Vec<String>) -> Vec<Argument>{
    let mut arg_idx = 0;
    let mut parsed_args: Vec<Argument> = Vec::new();
    while arg_idx < args.len() {
        let mut inc = 1;
        let arg_type = args.get(arg_idx).unwrap();
        if arg_idx < args.len() -1 {
            if !is_arg(args.get(arg_idx + 1).unwrap()){
                //if it isn't an argument type like --help, then try to parse a value from it
                inc = 2;
                let arg_val = args.get(arg_idx + 1).unwrap();
                let parsed_argument = parse_single_arg(arg_type, arg_val);
                if parsed_argument == Argument::Error {
                    println!("error argument propagated to higher level");
                }
                parsed_args.push(parsed_argument);
            }
            else{
                let parsed_argument = parse_single_arg(arg_type, "");
                parsed_args.push(parsed_argument);
            }
        }
        else{
            let parsed_argument = parse_single_arg(arg_type, "");
            parsed_args.push(parsed_argument);
        }
        arg_idx += inc;
    }
    parsed_args
}
pub fn construct_features(input_arguments: Option<Vec<Argument>>) -> GenerationData {
    dotenv::dotenv().ok();
    let mut minimum_length = 8;
    let mut include_numbers = false;
    let mut include_special = false;
    let mut include_ucase = false;
    let mut use_words = true;
    if let Ok(env_min_length) = env::var("MIN_LENGTH"){
        if let Ok(parsed_min_length) = env_min_length.parse::<usize>(){
            minimum_length = parsed_min_length;
        }
    }
    if let Ok(env_include_nums) = env::var("INCLUDE_NUMBERS"){
        if let Ok(parsed_include_nums) = env_include_nums.parse::<bool>(){
            include_numbers = parsed_include_nums;
        }
    }
    if let Ok(env_include_spec) = env::var("INCLUDE_SPECIAL"){
        if let Ok(parsed_include_spec) = env_include_spec.parse::<bool>(){
            include_special = parsed_include_spec;
        }
    }
    if let Ok(env_include_ucase) = env::var("INCLUDE_UCASE"){
        if let Ok(parsed_include_ucase) = env_include_ucase.parse::<bool>(){
            include_ucase = parsed_include_ucase;
        }
    }
    if let Ok(env_use_words) = env::var("USE_WORDS"){
        if let Ok(parsed_use_words) = env_use_words.parse::<bool>(){
            use_words = parsed_use_words;
        }
    }
    let mut save_data = false;
    let mut new_profile: Option<String> = None;
    let mut existing_profile: Option<String> = None;
    let mut use_existing_profile = false;
    if let Some(arguments) = input_arguments{
        for arg in arguments {
            match arg {
                Argument::ParsedArgument(arg_type, arg_val) => {
                    if let ArgValue::String(string_arg) = arg_val {
                        match arg_type {
                            ArgType::NewProfile => {
                                save_data = true;
                                new_profile = Some(string_arg);
                            }
                            ArgType::Profile => {
                                use_existing_profile = true;
                                existing_profile = Some(string_arg);
                            }
                            _ => {}
                        }
                    }
                    else if let ArgValue::Bool(boolean_arg) = arg_val {
                        match arg_type {
                            ArgType::IncludeNumbers => {
                                include_numbers = boolean_arg;
                            },
                            ArgType::IncludeSpecial => {
                                include_special = boolean_arg;
                            },
                            ArgType::IncludeUcase => {
                                include_ucase = boolean_arg;
                            },
                            ArgType::UseWords => {
                                use_words = boolean_arg;
                            },
                            _ => {
                                println!("invalid arg value for given arg type");
                            }
                        }
                    }
                    else if let ArgValue::Int(int_arg) = arg_val {
                        match arg_type {
                            ArgType::MinimumLength => {
                                minimum_length = int_arg;
                            },
                            _ => {
                                println!("invalid arg value for given arg type");
                            }
                        }
                    }
                }
                Argument::Error => { continue }
            }
        }
    }
    let mut generation_data = GenerationData::new(Some(minimum_length), 
                        Some(include_numbers), 
                        Some(include_special), 
                        Some(include_ucase), 
                        Some(use_words),
                        None,
                        Some(false));
    if save_data == true {
        println!("Saving current settings to user profile with the name: {}", new_profile.as_ref().unwrap());
        //TODO: take into account the --override option. if the user already exists and the
        //override option isn't set, then don't overwrite the profile with that particular name
        generation_data.profile = new_profile;
        let conn = database::create_connection();
        let result = database::insert_user_profile(&conn, &generation_data);
        match result {
            Ok(_) => {
                println!("Successfully saved profile");
            }
            Err(_) => {
                println!("Error saving profile.");
            }
        }
        conn.close().unwrap();
    }
    else if use_existing_profile == true {
        let conn = database::create_connection();
        let profile_settings = database::retrieve_profile_settings(&conn, &existing_profile.unwrap());
        conn.close().unwrap();
        return profile_settings.unwrap()
    }
    generation_data
}
/// Processes an argument vector and allows for early exit in the case of certain arguments,
/// like '--help'; avoids the need to process these arguments ahead of time in the 'main' module
pub fn process_and_execute_args(input_args: Option<Vec<Argument>>) -> GenerationData {
    match input_args {
        Some(ref parsed_args) => {
            for arg in parsed_args {
                match arg {
                    Argument::ParsedArgument(ArgType::Help, _) => {
                        println!("A command line tool to generate a random password with given parameters");
                        println!("Options:");
                        println!("--help: print the available command line options");
                        println!("--minlength <int> : min password length");
                        println!("--include_nums <bool> : include numbers in password");
                        println!("--include_spec <bool> : include special characters in password");
                        println!("--include_ucase <bool> : include uppercase characters in password");
                        println!("--use_dict_words <bool> : use dictionary words instead of random lowercase alphabetic characters");
                        println!("--profile <string> : name of the profile to use. Will pull from local database if such a profile exists");
                        println!("--new_profile <string> store a new profile with the provided settings using the given name");
                        println!("--overwrite <bool> : if using a profile, overwrite its current settings with the other command line options");
                        println!("--list_profiles : prints a list of available profiles");
                        println!("--profile_info <string> : prints the provided profile's settings, if the profile exists.");
                        exit(1);
                    }
                    Argument::ParsedArgument(ArgType::ListProfiles, _) => {
                        let conn = database::create_connection();
                        database::print_profiles(&conn);
                        conn.close().unwrap();
                        exit(1);
                    }
                    Argument::ParsedArgument(ArgType::ProfileInfo, ArgValue::String(profile_name)) => {
                        let conn = database::create_connection();
                        database::print_single_profile(&conn, &profile_name);
                        conn.close().unwrap();
                        exit(1);
                    }
                    /*
                    Argument::ParsedArgument(ArgType::NewProfile, ArgValue::String(profile_name)) => {
                        let conn = database::create_connection();
                        database::print_single_profile(&conn, &profile_name);
                        conn.close().unwrap();
                        exit(1);
                    }
                    */
                    _ => {
                        continue;
                    }
                }
            }
            construct_features(input_args)
        }
        None => {
            construct_features(None)
        }
    }
}
#[cfg(test)]
mod tests {
    use super::{
        ArgType,
        ArgValue,
        Argument,
        parse_single_arg,
        parse_args
    };
    //TODO: include a test for options that don't take arguments, like --help: make sure it's
    //properly recognized. 
    #[test]
    fn detect_help_argument() {
        let help_arg = parse_single_arg("--help", "");
        assert_eq!(help_arg, Argument::ParsedArgument(ArgType::Help, ArgValue::NoValue));
        let args_vec = vec![String::from("--help")];
        let parsed_help_arg_vec = parse_args(args_vec);
        assert_eq!(vec![help_arg], parsed_help_arg_vec);
    }
    //TODO: include a test for combinations of non-argument-taking and argument-taking options

    #[test]
    fn parse_single_argument() {
        let min_length_arg = parse_single_arg("--minlength", "8");
        assert_eq!(min_length_arg, Argument::ParsedArgument(ArgType::MinimumLength, ArgValue::Int(8)));
        let dict_words_arg = parse_single_arg("--use_dict_words", "true");
        assert_eq!(dict_words_arg, Argument::ParsedArgument(ArgType::UseWords, ArgValue::Bool(true)));
    }
    #[test]
    fn parse_args_from_vec() {
        let args_vec = vec![String::from("--minlength"), String::from("8")];
        let parsed_args = parse_args(args_vec);
        let correct_parsed_args = vec![Argument::ParsedArgument(ArgType::MinimumLength, ArgValue::Int(8))];
        assert_eq!(parsed_args, correct_parsed_args);
    }
}

