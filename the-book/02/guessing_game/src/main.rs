use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101); // inclusive on the lower bound but exclusive on the upper bound

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    let mut guess = String::new();
    // In Rust, variables are immutable by default. Use 'mut' to explicitly declare a variable mutable.
    // The :: syntax indicates that new is an associated function of the String type, aka a static method.

    io::stdin()
    .read_line(&mut guess) // The & indicates the argument is a reference. Also immutable by default, you need to use &mut guess rather than &guess to make it mutable.
    .expect("Failed to read line");

    println!("You guessed: {}", guess);
    // ou can print more than one value using curly brackets: the first set of curly brackets holds the first value listed after the format string,
    // the second set holds the second value, and so on.
}
