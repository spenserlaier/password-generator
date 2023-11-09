use crate::generation_logic::GenerationData;
use dotenv;
use std::env;
#[derive(PartialEq, Debug)]
pub enum ArgType {
    MinimumLength,
    IncludeNumbers,
    IncludeSpecial,
    IncludeUcase,
    UseWords,
    Error,
}
#[derive(PartialEq, Debug)]
pub enum ArgValue {
    Bool(bool),
    Int(usize),
    Error,
}
#[derive(PartialEq, Debug)]
pub enum Argument {
    ParsedArgument(ArgType, ArgValue),
    Error
}
pub fn parse_single_arg(arg_type: &str, arg_value: &str) -> Argument{
    let parsed_arg_value = match arg_value {
        "true" => ArgValue::Bool(true),
        "false" => ArgValue::Bool(false),
        x => {
            if let Ok(parsed_int) = x.parse::<usize>() {
                ArgValue::Int(parsed_int)
            }
            else{
                ArgValue::Error
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
    while arg_idx + 1 < args.len() {
        let arg_type = args.get(arg_idx).unwrap();
        let arg_val = args.get(arg_idx + 1).unwrap();
        let parsed_argument = parse_single_arg(arg_type, arg_val);
        if parsed_argument == Argument::Error {
            println!("error argument propagated to higher level");
        }
        parsed_args.push(parsed_argument);
        arg_idx += 2;
    }
    parsed_args
}
pub fn construct_features_from_arguments(arguments: Vec<Argument>) -> GenerationData {
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
    for arg in arguments {
        match arg {
            Argument::ParsedArgument(arg_type, arg_val) => {
                if let ArgValue::Bool(boolean_arg) = arg_val {
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
    GenerationData::new(Some(minimum_length), 
                        Some(include_numbers), 
                        Some(include_special), 
                        Some(include_ucase), 
                        Some(use_words))
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

