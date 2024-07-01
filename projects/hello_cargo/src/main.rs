fn main() {
    println!("Hello, Cargo!");
}

// Terminal Code

// A Rust program can be built using Cargo, with the line, "cargo new 'program_name'", in this case 'cargo new hello_cargo'.
// 'cd program_name' can be used to change the directory to 'program_name'.
// Cargo expects all source files to be located in the 'src' folder.
// If a project that might use Cargo, it is much easier to start by building the project with Cargo.

// Cargo has a built-in compiler. It can be run using the line, 'cargo build'.
// This command create a 'program_name.exe' file.
// Cargo puts the binary in a directory name 'debug'.
// The executable code can be run with the line, './target/debug/program_name', or in this case, './target/debug/hello_cargo'.
// To run the executable code in one line, we can use the line, 'cargo run'.
// 'cargo check' can be used to check if the file can be compiled, but doesn't create an executable.
// If there hasn't been any changes to the code, Cargo won't rebuild, but will run the program.