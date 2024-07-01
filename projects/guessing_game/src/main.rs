// rand::Rng package was linked in the 'Cargo.toml' file.
// Rng is a trait that comes from rand crate, it will be used to generate a random number.
use rand::Rng;
// std is a crate that is built into Rust that doesn't need to be linked with the Cargo.toml file.
// cmp is a method that compares two values, and can be called on anything that can be compared.
// Ordering is a type that is an enum, and the variants, Less, Greater, Equal.
use std::cmp::Ordering;
// std crate.
// io is a library that is from std. io stands for input/output.
use std::io;

fn main() {
    println!("Guess the number!");

    // let is used to create a variable.
    // 'secret_number' is the name of the variable.
    // '=' binds the left side of the expression with the right side.
    // The rand::thread_rng function gives us the particular random number generator we’re going to use: one that is local to the current thread of execution and is seeded by the operating system.
    // The gen_range on the random number generator with the argument passed will generate a random number from 1 to 100, including the lower and higher end.
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

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