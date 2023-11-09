use random_word;
use std::collections::HashSet;
use rand;
use rand::Rng;
pub fn generate_random_char(start: char, end:char) -> char {
    let mut rng = rand::thread_rng();
    let random_char : char = rng.gen_range(start..=end);
    random_char
}
pub fn generate_lcase_char() -> char {
    generate_random_char('a', 'z')
}
pub fn generate_ucase_char() -> char {
    generate_random_char('A', 'Z')
}
pub fn generate_numeric_char() -> char {
    generate_random_char('0', '9')
}
pub fn generate_special_char() -> char {
    generate_random_char('!', '~')
}
pub struct GenerationData {
    pub minimum_length: usize,
    pub include_numbers: bool,
    pub include_special: bool,
    pub include_ucase: bool,
    pub use_words: bool,
    pub profile: Option<String>,
    pub overwrite: bool,

}
impl GenerationData {
    pub fn new(minimum_length: Option<usize>, 
               include_numbers: Option<bool>, 
               include_special: Option<bool>, 
               include_ucase: Option<bool>, 
               use_words: Option<bool>,
               profile: Option<String>,
               overwrite: Option<bool>
               )-> GenerationData{
        let minimum_length = if let Some(min_length) = minimum_length{min_length} else {8};
        let include_numbers = if let Some(include_nums) = include_numbers{include_nums} else {false};
        let include_special = if let Some(include_spec) = include_special{include_spec} else {false};
        let include_ucase = if let Some(include_u) = include_ucase{include_u} else {false};
        let use_words = if let Some(use_wrds) = use_words{use_wrds} else {true};
        // let profile = if let Some(prof) = profile{} else None;
        let overwrite = if let Some(overw) = overwrite{overw} else {false};
        GenerationData{
            minimum_length,
            include_numbers,
            include_special,
            include_ucase,
            use_words,
            profile,
            overwrite
        }
    }
    pub fn generate_password(&self) -> String{
        let mut password = String::new();
        let mut used_words = HashSet::new();
        if self.use_words {
            while password.len() < self.minimum_length {
                let word = random_word::gen(random_word::Lang::En);
                if !used_words.contains(&word) {
                    for ch in word.chars() {
                        password.push(ch);
                    }
                    used_words.insert(word);
                }
            }
        }
        else{
            while password.len() < self.minimum_length {
                password.push(generate_lcase_char());
            }
        }
        if self.include_numbers{
            password.push(generate_numeric_char());
        }
        if self.include_special {
            password.push(generate_special_char());
        }
        if self.include_ucase {
            password.push(generate_ucase_char());
        }
        password
    }
}
