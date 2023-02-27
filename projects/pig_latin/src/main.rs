// Convert strings to pig latin. The first consonant of each word is moved 
// to the end of the word and “ay” is added, so “first” becomes “irst-fay.” 
// Words that start with a vowel have “hay” added to the end instead 
// (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!

use std::io;

fn main() {
    println!("We will start Pig Latin game!");
    println!("Please input your word");

    let line = read_line();
    let words: Vec<String> = parse_string_to_words(line);
    for word in words {
        let vowels = ['a', 'e'];
        let first_char = word.chars().next().unwrap();
        if vowels.contains(&first_char) {
            print!("{word}-hay ");
        } else {
            let word = &word[1..];
            print!("{word}-{first_char}ay ");
        }
    }

    fn read_line() -> String {
        // read line
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        return line;
    }

    fn parse_string_to_words(line: String) -> Vec<String> {
        // parse string of integers to vector
        line.split_whitespace()
            .map(|s| s.parse().expect("parse error"))
            .collect()
    }

}
