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
    minimum_length: usize,
    include_numbers: bool,
    include_special: bool,
    include_ucase: bool,
    use_words: bool,
}
impl GenerationData {
    pub fn new(minimum_length: usize, include_numbers: bool, include_special: bool, include_ucase: bool, use_words: bool) -> GenerationData{
        GenerationData{
            minimum_length,
            include_numbers,
            include_special,
            include_ucase,
            use_words,
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
