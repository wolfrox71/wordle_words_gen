use std::env;
use regex::Regex;
use std::fs;

// to use, type cargo run then the word to search using '.'s for missing letters
// [optionaly] enter all the characters that are disallowed (white) and to remove them from the outputs
// [optionaly] enter all the letters that are somewhere in the word (red)
// example use
// cargo run t..e. audiost de

fn get_words(_filename: &str) -> Vec<String> {
    // get the contents of the file as a single string
    let words = fs::read_to_string(_filename).expect(format!("Unable to read file {}", _filename).as_str());
    // get each line from the contents of the file as a vec of strings
    let _words: Vec<String> = words.lines().map(|s| s.to_string()).collect();
    // returns the string vector
    return _words;
}

fn main() {
    // get the expected length of word
    const WORD_LENGTH: usize = 5;
    // an array of characters fir the letters of the alphabet
    let letters = ['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'];
    // an array of the characters that mean that an unknown letter is being used
    let break_characters = ['.'];
    let given_characters;
    // assign these so that they can be in this scope for if they are needed depending on the number of args entered
    let mut disallowed_chars = String::default();
    let mut using_disallowed_chars: bool = false;
    let mut red_chars = String::default();
    let mut using_red_chars: bool = false;
    // set the maximum ammount of args that the user can entere without panicing 
    let max_args = 3;
    {
        // get the args passed in with the cargo run
        let mut args: Vec<String> = env::args().collect();
        // removes the first argument from the list of arguments
        // the first argument is the path of the output file
        args.remove(0);
        // this line is so that args becomes immutable again
        let args = args;
        // output the args passed in
        println!("Args: {:?}", args);
        if args.len() < 1 || args.len() > max_args {
            // if and incorrect number of args are passed in,
            // panic output that error and the number of args presented
            panic!("Expected 1 - {} argument(s) but {} arguments given", max_args, args.len())
        }
        // depending on the number of args passed in, use different things
        // if there are disallowed chars passed in
        // if there are 2+ args
        if args.len() >= 2 {
            disallowed_chars = args[1].clone();
            println!("Desallowed Chars: {:?}", disallowed_chars);
            using_disallowed_chars = true;
        }
        // pass in the red characters if there are 3+ args
        if args.len() >= 3 {
            red_chars = args[2].clone();
            println!("Red Chars: {}", red_chars);
            using_red_chars = true;
        }
        // get the characters from the first set of args given
        given_characters = args[0].clone();

    }
    // if the word is not the correct length
    if given_characters.len() != WORD_LENGTH {
        // panic with an error message
        panic!("Expected the entered argument to be {} characters long, got an argument of length {}", WORD_LENGTH, given_characters.len());
    }
    println!("Given characters: {}", given_characters);
    // get an array of characters that are given from the user's args
    let characters = given_characters.chars().collect::<Vec<char>>();
    // start a string from ^ -> start of line
    let mut regex_str = String::from("^");
    for character in characters {
        // if the character is an unknown letter
        if break_characters.contains(&character) {
            // add the symbol to the end of the regex
            if !using_disallowed_chars {
                regex_str.push_str(r"\w");
                continue;
            }
            regex_str.push_str(format!("[^{}]",disallowed_chars).as_str());
            // move to the next character
            continue;
        }
        // if the character is a letter
        if letters.contains(&character) {
                // add the character to the end of the regex
                regex_str.push(character);
                // move to the next character
                continue;
            }
        // if the character is not a letter or a * then panic displaying the eronius character
        panic!("Unexpected character found {}", character);
    }
    // push to the end of the regex a $ so that it only matches to the end of the line
    // ^ for start of line was in the original decleration
    regex_str.push('$');
    // create a new regex from the string and unwrap from the result
    let re = Regex::new(regex_str.as_str()).unwrap();
    println!("regex: {}", regex_str);
    let mut passed_words: Vec<String> = Vec::new();
    // get the words from the wordle folder
    let words = get_words("words.txt");
    'words: for word in words {
        let _word = word.as_str();
        // if red characters are being used and  if the word does not contain the red characters
        if using_red_chars {
            // go through each letter in the red chars 
            for letter in red_chars.chars() {
                // if the word does not contain the letter
                if !_word.contains(letter) {
                    // move on to the next word
                    continue 'words;
                }
            }
        }
        if re.is_match(&_word) {
            // add the word onto the vec of words that passed all the checks
            passed_words.push(word);
            // move on to the next word
            continue 'words;
        }

    }
    // output the avalable words
    println!("{:?}", passed_words);
}
