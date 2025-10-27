// Slices
// A ‘slice’ is a reference to (or “view” into) another data structure.
// They are useful for allowing safe, efficient access to a portion of an array without copying. //
// For example, you might want to reference only one line of a file read into memory.
// By nature, a slice is not created directly, but from an existing variable binding.
// Slices have a defined length, and can be mutable or immutable.

// Internally, slices are represented as a pointer to the beginning of the data and a length.

// Slicing syntax
// You can use a combo of & and [] to create a slice from various things.
// The & indicates that slices are similar to references, which we will cover in detail later in this section.
// The []s, with a range, let you define the length of the slice:

fn view_data() {
    let a = [0, 1, 2, 3, 4];
    let complete = &a[..]; // A slice containing all of the elements in `a`.
    let middle = &a[1..4]; // A slice of `a`: only the elements `1`, `2`, and `3`.
// Slices have type &[T]. We’ll talk about that T when we cover generics.
}

// Tuples
// A tuple is an ordered list of fixed size. Like this:

fn collection(){
    let x = (1, "hello");
// The parentheses and commas form this two-length tuple. Here’s the same code, but with the type annotated:
let x: (i32, &str) = (1, "hello");
}

// Tuple Indexing
// You can also access fields of a tuple with indexing syntax:

fn tuple_index() {
    let tuple = (1, 2, 3);

    let x = tuple.0;
    let y = tuple.1;
    let z = tuple.2;

    println!("x is {}", x);
// Like array indexing, it starts at zero, but unlike array indexing, it uses a ., rather than []s.

}

