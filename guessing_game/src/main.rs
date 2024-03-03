use std::io; // just like go, if this library not used, it will throw warning.

/*

Multiline comment also works!

*/

fn main() {
    println!("Guess number!"); // println need to add !
    println!("Please input:");

    // In Rust, variables are immutable by default, meaning once we give the variable a value, the value won’t change
    let mut guess = String::new(); // if this variable not mutated, it will throw variable does not need to be mutable

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line"); // in go, the tabbing for chain method goes automatically, rust is not.
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

    println!("Your guess: {guess}"); // in go, extra whitespace is removed automatically, rust is not. Linter?

    let x = 5; // tabbing would be disaster!
    let y = 10;

    println!("x = {x} and y + 2 = {}", y + 2); // formatting with template, fmt.Sprintf equivalent

    // tabbing issue solved with cargo fmt, added to readme
}
