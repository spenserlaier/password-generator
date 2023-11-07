use crate::generation_logic::GenerationData;
#[derive(PartialEq, Debug)]
pub enum ArgType {
    MinimumLength,
    IncludeNumbers,
    IncludeSpecial,
    IncludeUcase,
    UseWords,
    Error
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
    let mut minimum_length = 8;
    let mut include_numbers = false;
    let mut include_special = false;
    let mut include_ucase = false;
    let mut use_words = true;
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
    GenerationData::new(minimum_length, include_numbers, include_special, include_ucase, use_words)
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

