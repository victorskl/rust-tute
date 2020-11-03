use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    // println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();

        io::stdin() // <-- stdin() function returns an instance of std::io::Stdin
            .read_line(&mut guess)
            .expect("Failed to read line"); // <-- anticipate exception, without it, compiler warn
        /*
        let guess: u32 = guess
            .trim()// <-- trim the string for new line character i.e. \n
            .parse()// <-- parse string to int
            .expect("Please type a number!"); // <-- raise exception with message
        */
        let guess: u32 = match guess.trim().parse() {
            // handle parse exception
            Ok(num) => num,
            Err(_) => continue
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
