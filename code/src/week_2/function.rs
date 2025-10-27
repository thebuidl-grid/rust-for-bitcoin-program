// Every Rust program has at least one function, the main function:
// fn is a function followed by the name, some parentheses because this function takes no arguments,
// and then some curly braces to indicate the body.

// Here’s a function named foo:
fn foo() {
}

// So, what about taking arguments? Here’s a function that prints a number:
fn print_number(x: i32) {
    println!("x is: {}", x);
}


// Here’s a complete program that uses print_number:
pub fn run() {
    print_number(5);
}


// Here’s a complete program that adds two numbers together and prints them:
fn print_sum(x: i32, y: i32) {
    println!("sum is: {}", x + y);
}

// Unlike let, you must declare the types of function arguments. This does not work:


// What about returning a value? Here’s a function that adds one to an integer:

fn add_one(x: i32) -> i32 {
    x + 1
}

// Statements vs expression

// Rust is primarily an expression-based language. There are only two kinds of statements, and everything else is an expression.

// So what's the difference? Expressions return a value, and statements do not.
// That’s why we end up with ‘not all control paths return a value’ here: the statement x + 1; doesn’t return a value.
// There are two kinds of statements in Rust: ‘declaration statements’ and ‘expression statements’.
// Everything else is an expression. Let’s talk about declaration statements first.