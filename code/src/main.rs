// import session_3 module
mod week_2;

// entry point to run all topics covered
fn main() {
    // call functions from session_3 modules
    println!("Task 1:");
    week_2::task_1::main();

    println!("\nTask 2:");
    week_2::task_2::main();

    println!("\nTask 3:");
    week_2::task_3::main();

    println!("\nOther topics:");
    week_2::function::run();
    week_2::loops::run();
    week_2::primitive_types::run();
}