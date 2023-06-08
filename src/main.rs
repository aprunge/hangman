use rand::{thread_rng, Rng};
use std::io;

const MAX_LIVES: u8 = 6;

fn select_word(ai_word: &mut String, words: &[&str; 26]) {
    let mut rng = thread_rng();
    let n: usize = rng.gen_range(0..26);
    *ai_word = words[n].to_string();
}

fn check_input(input: char, ai_word: &str, result_array: &mut Vec<char>, lives: &mut u8) {
    let mut found = false;
    for (index, character) in ai_word.chars().enumerate() {
        if character == input {
            result_array[index] = character;
            found = true;
        }
    }
    if found {
        println!("Found character '{}'", input);
    } else {
        println!("Character '{}' not found", input);
        *lives -= 1;
        println!("Remaining Lives: {}", *lives);
        print_hangman(*lives);
    }
    println!("Word Formation: {}", result_array.iter().collect::<String>());
}

fn print_hangman(lives: u8) {
    let stages = [
        // Stage 5: 5 mistakes
        "
          |---
          |  o
          | /|\\
          | / \\
          |
          |
        ",
        // Stage 4: 4 mistakes
        "
          |---
          |  o
          | /|\\
          | /
          |
          |
        ",
        // Stage 3: 3 mistakes
        "
          |---
          |  o
          | /|\\
          |
          |
          |
        ",
        // Stage 2: 2 mistakes
        "
          |---
          |  o
          | /|
          |
          |
          |
        ",
        // Stage 1: 1 mistake
        "
          |---
          |  o
          |  |
          |
          |
          |
        ",
        // Stage 0: No mistakes
        "
          |---
          |  o
          |
          |
          |
          |
        ",
    ];

    if let Some(stage) = stages.get(lives as usize) {
        println!("{}", stage);
    }
}


fn main() {
    let mut gameloop = true;
    let mut line = String::new();
    let mut ai_word = String::new();
    let words: [&str; 26] = [
        "apple", "banana", "carrot", "dog", "elephant", "frog", "giraffe", "hamburger", "igloo",
        "jellyfish", "kangaroo", "lemon", "monkey", "noodle", "orange", "penguin", "quilt",
        "rabbit", "strawberry", "turtle", "umbrella", "violet", "watermelon", "xylophone",
        "yogurt", "zebra",
    ];

    select_word(&mut ai_word, &words);
    let mut result_array: Vec<char> = vec!['-'; ai_word.len()];
    let mut lives = MAX_LIVES;
    
    println!("Currently selected word: {}", ai_word);

    while gameloop {
        print!("Please enter a letter: ");
        io::stdin().read_line(&mut line).unwrap();
        let first_char = line.chars().next().unwrap();
        check_input(first_char, &ai_word, &mut result_array, &mut lives);
        line.clear();

        if ai_word.chars().eq(result_array.iter().cloned()) {
            gameloop = false;
        } else if lives == 0 {
            println!("Out of lives! The word was: {}", ai_word);
            gameloop = false;
        }
    }

    println!("Congratulations! You guessed the word correctly.");
}
