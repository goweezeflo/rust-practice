fn main() {
    // Basic function invocation - no return type
    let greeting = "Hello, Rust!";
    print_msg(greeting);

    // Invoking a function that returns a value
    let sum = add(1, 2);
    println!("Sum: {}", sum);

    // Invoking a function written the idiomatic way
    let result = multiply(3, 5);
    println!("Result: {}", result);
}

// Basic function definition - no return type
fn print_msg(msg: &str) {
    println!("{}", msg);
}

// Defining a function that returns a value of type [i32]
fn add(num1: i32, num2: i32) -> i32 {
    return num1 + num2;
}

// An idiomatic way of returning a value - No [return] keyword, and no semicolon
fn multiply(num1: i32, num2: i32) -> i32 {
    // The return value of the function is the value of the
    // last expression evaluated before the function returns to its caller
    num1 * num2
}
