// Variable binding
// Virtually every non-'Hello World’ Rust program uses variable bindings.
// They bind some value to a name, so it can be used later. let is used to introduce a binding, like this:
fn run() {
    specify_type();
}
fn bind_value() {
    let x = 5;
}

// Patterns
// in many languages, a variable binding would be called a variable,
// but Rust’s variable bindings have a few tricks up their sleeves.
// For example the left-hand side of a let statement is a ‘pattern’,
// not a variable name. This means we can do things like:
fn pattern_function() {
let (x, y) = (1, 2);
}

// Type annotations
// Rust is a statically typed language,
// which means that we specify our types up front, and they’re checked at compile time.
fn specify_type() {
let x: i32 = 5;
}


// Mutability
// By default, bindings are immutable. This code will not compile:

// fn change_value() {
//     let x = 5;
//     x = 10;
// }

// if you want it to be mutable, you can use mut:
// fn change_value() {
// let mut x = 5; // mut x: i32
// x = 10;
// }

// Scope and shadowing

// Let’s get back to bindings. Variable bindings have a scope - they are constrained to live in the block they were defined in.
// A block is a collection of statements enclosed by { and }.
// Function definitions are also blocks! In the following example we define two variable bindings, x and y, which live in different blocks.
// x can be accessed from inside the fn main() {} block, while y can be accessed only from inside the inner block:

// fn function_range() {
//     let x: i32 = 17;
//     {
//         let y: i32 = 3;
//         println!("The value of x is {} and value of y is {}", x, y);
//     }
//     println!("The value of x is {} and value of y is {}", x, y); // This won't work.
// }