use std::io::{self, Write};

// `main` is our mandatory function, and is run when we run our application.
fn main() {
    println!("");
    println!("┌───────────────────────────┐");
    println!("│ ✨  RUST CAESAR CIPHER ✨ │");
    println!("└───────────────────────────┘");
    println!("");

    // All variables in Rust are immutable by default
    let phrase = prompt_phrase();
    let shift = prompt_shift();
    let direction = prompt_direction();

    /*
     * Rust has concepts of "Ownership" and "Borrowing". Assigning values to
     * variables creates Ownership of that value (a pointer in stack to heap).
     *
     * If the variable goes out-of-scope, the GC purges it from memory and it no
     * longer exists.
     *   let a = 5;
     *   println!("{}", a); // <- works fine
     *   let b = a;
     *   println!("{}", a); // <- error, a no longer exists in this scope
     *   do_something(b);
     *   println!("{}", b); // <- also error, b no longer exists in this scope
     *
     * Rust also has the concept of "Borrowing". This allows us to instead pass
     * a ref instead of transferring ownership of that variable.
     *   let a = 5;
     *   do_something(&a); // <- we pass a ref instead of transferring ownership
     *   println!("{}", a); // <- still works fine
     *
     * https://medium.com/better-programming/rust-ownership-and-borrowing-9cf7f081ade0
     */
    let processed = process_phrase(&phrase, &shift, &direction);

    println!("");
    println!("    {}", phrase);
    println!("    ⇵");
    println!("    {}", processed);
    println!("");
}

/**
 * All functions in Rust follow the signature:
 *
 * fn function_name(arg: Type, arg: Type) -> ReturnType {}
 *
 */
fn prompt_phrase() -> String {
    let phrase: String;
    /*
     * Using String::new allows us to allocate memory for a String of unknown
     * size. Typically, however, you want to try and limit the amount of memory
     * used by your app and would prefer to have fixed-length string by using
     * something like String::from or even &str (String literal).
     *
     * let a = String::from("My string");
     * let b = "My string";
     *
     * String::new is good for user input, however, since we can't know the
     * _actual_ value in advance.
     */
    // We can make variable mutable by using the `mut` keyword.
    let mut buf = String::new();

    print!("Enter your phrase [Hello, world!]: ");
    // We flush the stdout so the above print is shown in the console _before_
    // the user can type.
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut buf)
        .expect("Please enter a phrase to encrypt/decrypt");

    // We need to trim whitespace from the start/end of the stdin, otherwise we
    // wind up with newline chars from the buffer.
    let trimmed = String::from(buf.trim());

    if trimmed.is_empty() {
        phrase = String::from("Hello, world!");
    } else {
        phrase = trimmed;
    }

    /*
     * Rust has this funky way of returning values from functions.
     * We call `phrase` here on its own to signal it's the return value. _IF_ we
     * put a semicolon on the end, Rust will interpret that as a statement
     * rather than a return value and Err on us.
     *
     * We could also make use of the `return` keyword, but the below seems to be
     * the community standard.
     */
    phrase
}

/**
 * Prompts the user for the amount to shift the phrase by.
 *
 * We'll return an unsigned 8-bit integer because we'll be using this value to
 * work with characters, which will be represented by u8.
 */
fn prompt_shift() -> u8 {
    let mut shift: u8 = 13;

    /*
     * Infinite loop until `break` is called
     */
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

        /*
         * Match expressions are very similar to switch/case statements in other
         * languages.
         *
         * https://doc.rust-lang.org/rust-by-example/flow_control/match.html
         */
        match trimmed.parse::<u8>() {
            Ok(i) => {
                if i > 26 {
                    println!("Please enter a valid number.");
                    continue;
                }

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

fn prompt_direction() -> char {
    let mut direction = 'e';

    loop {
        let mut buf = String::new();

        print!("Are you encrypting or decrypting? [E/d]: ");
        // We flush the stdout so the above print is shown in the console _before_
        // the user can type.
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut buf)
            .expect("Unable to read input");

        let entered = buf.trim();

        if entered.is_empty() {
            break;
        }

        let first = buf.to_ascii_lowercase().chars().next().unwrap();
        if first == 'e' || first == 'd' {
            direction = first;
            break;
        }
    }

    direction
}

/**
 * This is how our functions can recieve borrowed values. In the signature, we
 * mark which ones are references with the ampersand.
 *
 * fn my_func(take_ownership: String, borrowed: &String) -> String {}
 */
fn process_phrase(phrase: &String, shift: &u8, direction: &char) -> String {
    let mut offset = *shift;

    if *direction == 'd' {
        offset = 26 - offset;
    };

    /*
     * Since the user has entered a phrase already, we can preallocate memory
     * for our processed phrase, since it will be the same length.
     *
     * String::with_capacity will create an empty space in memory for our string
     */
    let mut processed = String::with_capacity(phrase.len());

    for letter in phrase.chars() {
        if letter.is_alphabetic() {
            // Figure out which case this char is and cast the start of that
            // case to u8. We'll use this to calc the new ASCII address
            // http://www.asciitable.com/
            let case = if letter.is_uppercase() { 'A' } else { 'a' } as u8;
            // let total_shift = ((letter as u8 + offset - case) % 26) + case;
            let total_shift = (letter as u8 - case + offset) % 26 + case;
            // Cast the ASCII address to a character and push it onto the end of
            // our string.
            processed.push(total_shift as char);
        } else {
            // If it's not an alphabetic character, don't process it, just push
            // it into the processed string.
            processed.push(letter);
        }
    }

    processed
}
