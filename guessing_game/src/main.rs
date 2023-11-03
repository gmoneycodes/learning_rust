use std::io;
use std::cmp::Ordering;
use rand::Rng;

// Accompanying notes are available in ruyoga.io (look for the blogpost Learning Rust Chapter 2)

fn main() {
    println!("Guess a number between 1 and 100!");
    let random_number = rand::thread_rng().gen_range(1..=100);

    loop{
        println!("Please input your guess");

        let mut guess : String = String::new();

        io::stdin()
            // read_line() appends inputs of stdin into a string (without overwriting its content)
            // guess needs to be mutable, since read line will change its value
            .read_line(&mut guess)
            .expect("failed to read line");


        let guess: u32 = guess.trim().parse().expect("please type a number");
        match guess.cmp(&random_number){
            Ordering::Greater => println!("Too big bro \n"),
            Ordering::Less => println!("Too small buddy \n"),
            Ordering::Equal => {
                println!("Congratulations!! you just won a new iPhone 13");
                break;
            },
        }

        println!("You guessed: {guess}");
    }
}