use rand::Rng;
use std::cmp::Ordering;
use std::io; // just like go, if this library not used, it will throw warning.

/*

Multiline comment also works!

*/

fn main() {
    println!("Guess number!"); // println need to add !

    // random secret, generated between 1 - 100
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        // In Rust, variables are immutable by default, meaning once we give the variable a value, the value won’t change
        let mut guess = String::new(); // if this variable not mutated, it will throw variable does not need to be mutable

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line"); // in go, the tabbing for chain method goes automatically, rust is too! The formatter doing it on save.
                                            /*
                                               since read_line return 2 value
                                               .expect need to be added. The error/warning message is surprisingly helpful:

                                                 --> src/main.rs:16:5
                                                       |
                                                   16  | /     io::stdin()
                                                   17  | |      .read_line(&mut guess);
                                                       | |___________________________^
                                                       |
                                                       = note: this `Result` may be an `Err` variant, which should be handled
                                                       = note: `#[warn(unused_must_use)]` on by default
                                                   help: use `let _ = ...` to ignore the resulting value
                                                       |
                                                   16  |     let _ = io::stdin()
                                                       |     +++++++

                                            */

        // attempt convert guess string to a uint32
        // Rust allows us to shadow the previous value of guess with a new one. Instead of making similar variable with another name: guess_str & guess_int
        // but simply trying to parse will cause panic,
        // so we add match, like type assertion in go
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // do not cause panic if err
        };
        println!("Your guess: {guess}"); // in go, extra whitespace is removed automatically, rust is too! The formatter doing it on save.

        // i think this match cmp works like switch case
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

    // tabbing issue solved with cargo fmt, added to readme
}
