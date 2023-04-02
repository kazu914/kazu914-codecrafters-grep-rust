use std::env;
use std::io;
use std::process;

fn is_word_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}

fn is_char_in_set(c: char, set: &str) -> bool {
    set.chars().any(|x| x == c)
}

fn is_char_not_in_set(c: char, set: &str) -> bool {
    set.chars().all(|x| x != c)
}

fn match_pattern(input_line: &str, pattern: &str) -> bool {
    if pattern.chars().count() == 1 {
        input_line.contains(pattern)
    } else if pattern == r"\d" {
        input_line.chars().any(|c| c.is_ascii_digit())
    } else if pattern == r"\w" {
        input_line.chars().any(is_word_char)
    } else if pattern.starts_with('[') && pattern.ends_with(']') {
        let pattern_set = &pattern[1..pattern.len() - 1];
        if !pattern_set.starts_with('^') {
            input_line.chars().any(|c| is_char_in_set(c, pattern_set))
        } else {
            let inverted_set = &pattern_set[1..];
            input_line
                .chars()
                .all(|c| is_char_not_in_set(c, inverted_set))
        }
    } else {
        panic!("Unhandled pattern: {}", pattern)
    }
}

// Usage: echo <input_text> | your_grep.sh -E <pattern>
fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    if env::args().nth(1).unwrap() != "-E" {
        println!("Expected first argument to be '-E'");
        process::exit(1);
    }

    let pattern = env::args().nth(2).unwrap();
    let mut input_line = String::new();

    io::stdin().read_line(&mut input_line).unwrap();

    if match_pattern(&input_line, &pattern) {
        process::exit(0)
    } else {
        process::exit(1)
    }
}
