// bring the io lib into scope, which comes from the
// standard library, known as std
use std::io;
use rand::Rng;
use std::cmp::Ordering;

// Rust brings only a few types into the scope of every prog
// in the "prelude". If it's not in the "prelude" it must be
// imported explicitly with a "use" statement.

fn main() {
  println!("Guess a number!");

  // Loop is an infinite loop
  loop {
    println!("Please input your guess");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    // variables are immutable by default, so they must
    // be explicitly set otherwise.
    let mut guess = String::new();
    // ^ Here, guess is inferred to be a string which is pretty cool.

    // The :: is for calling an "associated function" of the
    // string type. Estn: a "static method" of the string "class"

    // If we didn't have "use std:io", we could have written
    // "std::io::Stdin"
    io::stdin()
      .read_line(&mut guess)
      .expect("Failed to read line");

    let guess: u32 = match guess.trim().parse() { 
      Ok(num) => num, // If okay, return the number
      Err(_) => continue, // _ (catch-all variable. like haskell remember lol)
      // continue means -> go to the next iteration of the loop
    };
    // variable "shadow" -> re-declaration

    // `read_line` puts user input into the string we pass it,
    // but it also returns a value: of type io::Result.
    // Result types are enums, for Result, options are Ok or Err
    // if the Result read_line returns is Err, it'll be caught by the
    // `.expect` method.

    println!("You guessed: {}", guess);

    // `Ordering` is an enum, with three "variant" 
    // This is then asserted in a "Match" expression
    match guess.cmp(&secret_number) {
      // a "Match" expression is made up of "arms"
      Ordering::Less => println!("Too small!"),
      // An "arm" has a pattern and the code that should be run if the value given
      // to the beginning of the match expression fits that arm's pattern.
      Ordering::Greater => println!("Too big!"),
      Ordering::Equal => {
        println!("You win!");
        break; // breaks the inf loop
      }
    }
  }
}

// Crate: a crate is a col of rust source code files. This project is a 
// "binary crate", which is an executable. the "rand" crate is a "library
// crate" which contains code intended to be used in other programs.

// 