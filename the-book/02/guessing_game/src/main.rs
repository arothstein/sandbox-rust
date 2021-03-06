use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101); // inclusive on the lower bound but exclusive on the upper bound

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        // In Rust, variables are immutable by default. Use 'mut' to explicitly declare a variable mutable.
        // The :: syntax indicates that new is an associated function of the String type, aka a static method.

        io::stdin()
        .read_line(&mut guess) // The & indicates the argument is a reference. Also immutable by default, you need to use &mut guess rather than &guess to make it mutable.
        .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);
        // ou can print more than one value using curly brackets: the first set of curly brackets holds the first value listed after the format string,
        // the second set holds the second value, and so on.

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
