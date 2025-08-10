
fn get_item(vec: &[i32], index: usize) {
    // Retrieve a value at a specific index
    // Using `match` is a safe way to handle the Option returned by `get`
    match vec.get(index) {
        Some(value) => println!("The value at index {} is {}", index, value),
        None => println!("No value at index {}. It's out of bounds.", index),
    }
}

// Takes a slice of i32 and returns their sum.
fn sum_vector(v: &[i32]) -> i32 {
    // The `iter()` method creates an iterator, and `sum()` consumes the
    // iterator to calculate the sum of all items.
    v.iter().sum()
}

fn main() {
    let vec = vec![1, 2, 3, 4, 5];
    get_item(&vec, 3);
    get_item(&vec, 99); // Example with an out-of-bounds index

    // Retrieve a value at a specific index
    // Direct indexing will panic if the index is out of bounds.
    // It's often safer to use `vec.get()`.
    let third_value = vec[2];
    println!("The third value in the vector is: {}", third_value);

    // Retrieve the last value
    // `last()` returns an Option, so `unwrap()` is used here.
    // This will panic if the vector is empty.
    let last_value = vec.last().unwrap();
    println!("The last value in the vector is: {}", last_value);

    // Retrieve the first value using pattern matching
    match vec.first() {
        Some(first_value) => println!("The first value in the vector is: {}", first_value),
        None => println!("The vector is empty!"),
    }

    // Calculate and print the sum of the vector's elements
    let sum = sum_vector(&vec);
    println!("The sum of the vector's elements is: {}", sum);
}
