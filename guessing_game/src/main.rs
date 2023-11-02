use std::io;

// Accompanying notes are available in ruyoga.io (look for the blogpost Learning Rust)

fn main() {
    println!("Guess a number between 1 and 10!");
    println!("Please input your guess");

    let mut guess : String = String::new();

    io::stdin()
        // read_line() appends inputs of stdin into a string (without overwriting its content)
        // guess needs to be mutable, since read line will change its value
        .read_line(&mut guess)
        .expect("failed to read line");

    println!("You guessed: {guess}");
}
