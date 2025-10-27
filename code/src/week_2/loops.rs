// Loops
// Rust currently provides three approaches to performing some kind of iterative activity.
// They are: loop, while and for. Each approach has its own set of uses.

// loop
// The infinite loop is the simplest form of loop available in Rust.
// Using the keyword loop, Rust provides a way to loop indefinitely until some terminating statement is reached.
// Rust's infinite loops look like this:

pub fn run () {
    sometime();
    condition();
}

fn forever() {
    loop {
    println!("Loop forever!");
}
}

// while
// Rust also has a while loop. It looks like this:

fn sometime() {
    let mut x = 5; // mut x: i32
    let mut done = false; // mut done: bool

    while !done {
        x += x - 3;

        println!("{}", x);

        if x % 5 == 0 {
            done = true;
        }
    }
}

// while loops are the correct choice when you’re not sure how many times you need to loop.

// for
// The for loop is used to loop a particular number of times.
// Rust’s for loops work a bit differently than in other systems languages,
// however. Rust’s for loop doesn’t look like this “C-style” for loop:

fn condition() {
    for x in 0..10 {
    println!("{}", x); // x: i32
}
}

