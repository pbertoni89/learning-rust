use std::env;
use std::fs;
use std::collections::HashMap;


fn get_all_words() -> String {
    let contents = match env::args().nth(1) {
        Some(f) => match fs::read_to_string(f) {
            Ok(s) => s.to_lowercase(),
            Err(e) => {
                eprintln!("Could not read file: {}", e);
                std::process::exit(1);
            }
        },
        None => {
            eprintln!("Program requires an argument: <file path>");
            std::process::exit(2);
        }
    };
    contents
}


fn get_count_words(all_words: Vec<&str>) -> HashMap<&str, u32> {
    let mut word_counts: HashMap<&str, u32> = HashMap::new();
    for word in all_words.iter() {
        *word_counts.entry(word).or_insert(0) += 1;
    };
    word_counts
}


fn get_top_words(word_counts: HashMap<&str, u32>) -> (u32, Vec<&str>) {
    let mut top_count = 0u32;
    let mut top_words: Vec<&str> = Vec::new();
    for (&key, &val) in word_counts.iter() {
        if val > top_count {
            top_count = val;
            top_words.clear();
            top_words.push(key);
        } else if val == top_count {
            top_words.push(key);
        }
    }
    (top_count, top_words)
}


fn main() {
    // read file and build vector of individual words
    let contents = get_all_words();
    let all_words = contents.split_whitespace().collect::<Vec<&str>>();

    // count how many times each unique word occurs
    let word_counts = get_count_words(all_words);

    // determine the most commonly used word(s)
    let (top_count, top_words) = get_top_words(word_counts);

    // display results
    println!("Top word(s) occurred {} times:", top_count);
    for (i, word) in top_words.iter().enumerate() {
        println!("{}) {}", i+1, word);
    }
}