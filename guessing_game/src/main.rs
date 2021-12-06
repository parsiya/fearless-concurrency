use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    // Generate a "random" number between 1 and 100.
    // Alternative param to "1..101" is "1..=100".
    let secret_number = rand::thread_rng().gen_range(1..101);
    // println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        // Create a mutable string.
        let mut guess = String::new();

        let _bytes = io::stdin()
            // Read from input and append to guess.
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,     // If parse worked correctly, store the number in guess.
            Err(_) => continue, // If parse returned an error, go back to the beginning of the loop.
        };
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

        // println!("Read {} bytes", _bytes);
        println!("You guessed: {}", guess);
    }
}
