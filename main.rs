use rand::Rng;
use std::env;

fn generate_random_word(length: usize) -> String {
    let charset: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let mut rng = rand::thread_rng();
    let word: String = (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset[idx] as char
        })
        .collect();
    word
}

fn main() {
    
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <number_of_words> <word_length>", args[0]);
        std::process::exit(1);
    }

    let number_of_words: usize = args[1].parse().expect("Please provide a valid number for the number of words.");
    let word_length: usize = args[2].parse().expect("Please provide a valid number for the word length.");

    
    for _ in 0..number_of_words {
        let word = generate_random_word(word_length);
        println!("{}", word);
    }
}
