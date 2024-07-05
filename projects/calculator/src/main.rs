use std::io;

fn main() {
    println!("Extended Calculator");
    println!("Enter 'quit' to exit.");

    loop {
        // Input
        println!("Enter your expression (e.g., 2+2):");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        // Trim whitespace and handle quitting
        let input = input.trim();
        if input == "quit" {
            break;
        }

        // Parse and evaluate the expression
        let result = evaluate_expression(input);
        println!("Result: {}", result);
    }
}

fn evaluate_expression(expression: &str) -> f64 {
    // Attempt to find the operator position
    let operator_pos = expression.chars().position(|c| "+-*/%^<=>".contains(c));

    if let Some(pos) = operator_pos {
        let operator = &expression[pos..pos + 1];
        let num1_str = &expression[..pos].trim();
        let num2_str = &expression[pos + 1..].trim();

        // Parse numbers
        let num1: f64 = num1_str.parse().expect("Invalid number");
        let num2: f64 = num2_str.parse().expect("Invalid number");

        // Perform calculation
        match operator {
            "+" => num1 + num2,
            "-" => num1 - num2,
            "*" => num1 * num2,
            "/" => num1 / num2,
            "%" => num1 % num2, // Modulus operator
            "^" => num1.powf(num2), // Power operator
            "==" => if num1 == num2 { 1.0 } else { 0.0 }, // Equality comparison
            "!=" => if num1 != num2 { 1.0 } else { 0.0 }, // Inequality comparison
            ">" => if num1 > num2 { 1.0 } else { 0.0 }, // Greater than comparison
            ">=" => if num1 >= num2 { 1.0 } else { 0.0 }, // Greater than or equal to comparison
            "<" => if num1 < num2 { 1.0 } else { 0.0 }, // Less than comparison
            "<=" => if num1 <= num2 { 1.0 } else { 0.0 }, // Less than or equal to comparison
            _ => panic!("Invalid operator"),
        }
    } else {
        panic!("Invalid expression format");
    }
}
