use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);
    // println!("The secret number is: {}", secret_number);
    // ^^^
    // Then we call the gen_range method on the random number generator. This method is defined by
    // the Rng trait that we brought into scope with the use rand::Rng; statement. The gen_range
    // method takes a range expression as an argument and generates a random number in the range.
    // The kind of range expression we're using here takes the form start..=end and is inclusive
    // on the lower and upper bounds, so we need to specify 1..=100 to request a number
    // between 1 and 100.

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
        // ^^^
        // We create a variable named guess. But wait, doesn't the program already have a variable
        // named guess? It does, but helpfully Rust allows us to shadow the previous value of guess
        // with a new one. Shadowing lets us reuse the guess variable name rather than forcing us to
        // create two unique variables, such as guess_str and guess, for example. We'll cover this
        // in more detail in Chapter 3, but for now, know that this feature is often used when you
        // want to convert a value from one type to another type.

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
