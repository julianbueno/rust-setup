// Inserts a value at the beginning and end of a vector.
fn insert_at_both_ends(vec: &mut Vec<i32>, value: i32) {
    vec.insert(0, value);
    vec.push(value);
}

// Appends all elements from the second vector to the first.
// Note: This leaves the `second` vector empty.
fn append_vectors(first: &mut Vec<i32>, second: &mut Vec<i32>) {
    first.append(second);
}

fn main() {
    let mut v = vec![1, 2, 3];
    v.push(4);
    println!("{:?}", v); // Output: [1, 2, 3, 4]

    // extend adds each element of the given slice to the vector
    let more_numbers = vec![5, 6];
    v.extend(more_numbers);
    println!("{:?}", v);

    // append adds the given vector to the vector, requires the vector to be mutable
    let mut other_numbers = vec![7, 8];
    v.append(&mut other_numbers);
    println!("{:?}", v);

    // insert items at a given index
    v.insert(0, 0);
    println!("{:?}", v); // Output: [0, 1, 2, 3, 4, 5, 6, 7, 8]

    // Insert a value at the beginning and end of the vector
    insert_at_both_ends(&mut v, 99);
    println!("{:?}", v); // Output: [99, 0, 1, 2, 3, 4, 5, 6, 7, 8, 99]

    // Append another vector using the new function
    let mut even_more_numbers = vec![100, 101, 102];
    append_vectors(&mut v, &mut even_more_numbers);
    println!("After appending with function: {:?}", v);
}
