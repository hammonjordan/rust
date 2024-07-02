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

    // The loop keyword creates an infinite loop, running the code inside of the {}.
    loop {
        println!("Please input your guess.");

        // 'mut' stands for mutable, a variable is unmutable by default, meaning you can't change the value somewhere else.
        // String::new() means to create a new instance of a String.
        // Here, we are setting guess, as a mutable variable to the instance of a new String.
        let mut guess = String::new();

        // io::stdin is a type that represents a handle to the standard input for the terminal.
        // 'read_line' is a method that is being called on the standard input handle to get input from the user.
        // We're passing '&mut guess' as the argument to 'read_line; to tell it what string to store the user input in.
        // The string argument also needs to be mutable so the method can change the string's content.
        // The expect method is an instance of a a Result type.
        // If the instance of Result is an 'Err' value, expect will cause the program to crash and display the message that you passed as an argument to the expect.
        // If the read_line method returns and Err, it would likely be the result of an error coming from the underlying operating system.
        // If the instance of Result is an Ok value, expect will take the return value that Ok is holding and return just that value.
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        // u32 is declaring a type to the variable. u32 is an unsigned 32 bit number.
        // Using a match expression decides what to do next based based on which variant of Ordering was return from the call to cmp with the values in guess and secret_number.
        // A match expression is made up of arms. An arm consists of a pattern to match against, and the code that should be run if the value given to match fits that arm's value.
        // Rust takes the value given to match and looks through each arm's pattern in turn.
        // Patterns and the match construct let you express a variety of situations your code might encounter, and they make sure they handle them all.
        // We bind he new variable to the expression 'guess.trim().parse()'.
        // The guess in this expression refers to the original guess variable that contained the input as a string.
        // The trim method on a String instance will eliminate any whitespace at the beginning and end, which we must do in order to compare the string to the u32, which can only contain numerical values.
        // The trim method also removes any '/r/n' which is carriage return and new line.
        // The parse mehtod on strings converts a string to another type.
        // We need to tell Rust the exact number type we want by uing u32. u32 is a good choose for a small positive number.
        // The ':' after the variable name tells rust that we'll annotate the variables type.
        // The u32 annotation adn the comparision with 'secret_number' means Rust will infer that 'secret_number' should be a u32 as well.
        // The parse method will only work on characters that can logically be converted into numbers, which is why trim is necessary.
        // The parse method also returns a Result.
        // If the parse method returns an Ok variant and contains a value of num, the expression evaluates to 'num'.
        // If the parse method returns an Err variant, the will execute the code 'continue' which tells the program to go to the next iteration of the loop.
        // The underscore is a catchall value. Which means no matter what information they have inside them, to match all Err values and ignores them.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // A variable value can be inserted into a string using '{}' with the variable_name storing the value.
        println!("You guessed: {guess}");

        // This function compares the value of the variable 'guess' with the value of 'secret_number' by referencing secret_number using a '&'.
        match guess.cmp(&secret_number) {
            // Ordering is another enum that has variants, Less, Greater, and Equal.
            //If guess is less than secret_number, println!("Too small!").
            Ordering::Less => println!("Too small!"),
            //If guess is greater than secret_number, println!("Too big!").
            Ordering::Greater => println!("Too big!"),
            //If guess is equal to than secret_number, println!("You win!").
            // Then 'break', which means to exit the loop when the user guesses correctly.
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}