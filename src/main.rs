use std::io::{self, Write};

// `main` is our mandatory function, and is run when we run our application.
fn main() {
    println!("");
    println!("┌───────────────────────────┐");
    println!("│ ✨  RUST CAESAR CIPHER ✨ │");
    println!("└───────────────────────────┘");
    println!("");

    let phrase = prompt_phrase();
    let shift = prompt_shift();
    let processed = process_phrase(&phrase, &shift);

    println!("");
    println!("    {}", phrase);
    println!("    ⇵");
    println!("    {}", processed);
    println!("");
}

fn prompt_phrase() -> String {
    let phrase: String;
    let mut buf = String::new();

    print!("Enter your phrase [Hello, world!]: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut buf)
        .expect("Please enter a phrase to encrypt/decrypt");

    let trimmed = buf.trim().to_string();

    if trimmed.is_empty() {
        phrase = String::from("Hello, world!");
    } else {
        phrase = trimmed;
    }

    phrase
}

fn prompt_shift() -> u8 {
    let mut shift: u8 = 13;

    loop {
        let mut buf = String::new();

        print!("Enter your shift [13]: ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut buf)
            .expect("Failed to get input.");

        let trimmed = buf.trim();
        if trimmed.is_empty() {
            break;
        }

        match trimmed.parse::<u8>() {
            Ok(i) => {
                shift = i;
                break;
            }
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        }
    }

    shift
}

fn process_phrase(phrase: &String, shift: &u8) -> String {
    let mut processed = String::with_capacity(phrase.len());

    for letter in phrase.chars() {
        if letter.is_alphabetic() {
            let case = if letter.is_uppercase() { 'A' } else { 'a' } as u8;
            let total_shift = ((letter as u8 + shift - case) % 26) + case;
            processed.push(total_shift as char);
        } else {
            processed.push(letter);
        }
    }

    processed
}
