// The Rust language has a number of types that are considered ‘primitive’.
// This means that they’re built-in to the language.
// Rust is structured in such a way that the standard library also provides a number of useful types built on top of these ones,
// as well, but these are the most primitive.

// Boalean

// Rust has a built-in boolean type, named bool. It has two values, true and false:
#![allow(unused_variables)]
pub fn run() {
let x = true;
let y: bool = false;
}

// Char

// The char type represents a single Unicode scalar value. You can create chars with a single tick: (')

// fn main() {
// let x = 'x';
// let two_hearts = '💕';
// }

// Unlike some other languages, this means that Rust’s char is not a single byte, but four.


// Numeric types

// Rust has a variety of numeric types in a few categories: signed and unsigned, fixed and variable, floating-point and integer.
// These types consist of two parts: the category, and the size. For example, u16 is an unsigned type with sixteen bits of size. More bits lets you have bigger numbers.
// If a number literal has nothing to cause its type to be inferred, it defaults:

fn intro_to_signed_integer() {
let x = 42; // `x` has type `i32`.

let y = 1.0; // `y` has type `f64`.
}

// Fixed-sized types

// Rust also provides types whose particular size depends on the underlying machine architecture.
// Their range is sufficient to express the size of any collection, so these types have ‘size’ as the category.
// They come in signed and unsigned varieties which account for two types: isize and usize.

// Floating-point types
// Rust also has two floating point types: f32 and f64. These correspond to IEEE-754 single and double precision numbers.

// Arrays
// Like many programming languages, Rust has list types to represent a sequence of things.
// The most basic is the array, a fixed-size list of elements of the same type. By default, arrays are immutable.


fn array_container() {
    let a = [1, 2, 3]; // a: [i32; 3]
    let mut m = [1, 2, 3]; // m: [i32; 3]
// Arrays have type [T; N]. We’ll talk about this T notation in the generics section. The N is a compile-time constant, for the length of the array.
// There’s a shorthand for initializing each element of an array to the same value. In this example, each element of a will be initialized to 0:
    let a = [0; 20]; // a: [i32; 20]
// You can get the number of elements in an array a with a.len():

    let a = [1, 2, 3];

    println!("a has {} elements", a.len());
//You can access a particular element of an array with subscript notation:

    let names = ["Graydon", "Brian", "Niko"]; // names: [&str; 3]

    println!("The second name is: {}", names[1]);

}
