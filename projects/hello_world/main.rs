// 'fn' = function.
// 'main' is the name of the function
// No arguments are currently being passed through the function.
// The 'main' function is always the first code that runs in every executable Rust program.
// The code inside the function are called parameters.

fn main() {

    // This line is printing the text, "Hello, World!" in this case, onto the screen.
    // Rust style is to indent with four spaces.
    // 'println!' calls a Rust 'macro'.
    // If the line was calling a function, there wouldn't be an '!', it would read, 'println'.
    // 'Hello, World!' is a string being passed as an argument, and the string is being printed on the screen.
    // Most lines end with a ';'. A semicolon indicates that the expression is over. 
    println!("Hello, World!");
}

// Terminal Code

// In Rust, compiling and running are a seperate step.
// To compile in Rust, we use the line 'rustc program_name.rs', in this case, 'rustc main.rs'.
// 'ls' stands for 'list'.
// There are three files after the program_name.rs file is compiled.
// The files are 'program_name.rs', 'program_name.exe', and 'program_name.pdb'.
// The 'program_name.rs' is the src code file.
// The 'program_name.exe' is the executable code file.
// The 'program_name.pdb' is a file that contains debugging information.