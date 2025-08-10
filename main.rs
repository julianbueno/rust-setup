fn main() {
    let sentence = "the quickest brown fox jumps over the lazy dog".to_string();
    // Use slicing to get the first three characters of the sentence
    println!("{}", &sentence[0..=4]);

    // concatenate using format!
    let description = format!("Title: Quick story\n{}", sentence);
    //println!("{}", description);

    // Count the number of occurrences of each vowel
    let mut a_count = 0;
    let mut e_count = 0;
    let mut i_count = 0;
    let mut o_count = 0;
    let mut u_count = 0;

    for c in sentence.chars() {
        match c {
            'a' => a_count += 1,
            'e' => e_count += 1,
            'i' => i_count += 1,
            'o' => o_count += 1,
            'u' => u_count += 1,
            _ => {}
        }
    }

    println!("Number of 'a': {}", a_count);
    println!("Number of 'e': {}", e_count);
    println!("Number of 'i': {}", i_count);
    println!("Number of 'o': {}", o_count);
    println!("Number of 'u': {}", u_count);

    // Split and collect into a vector
    //let words: Vec<&str> = sentence.split_whitespace().collect();
    let words = sentence.split(' ').collect::<Vec<&str>>();
    //println!("{:?}", words);

    let reversed = sentence.chars().rev().collect::<String>();
    println!("{}", reversed);

    // Find and print the longest word
    let longest = longest_word(&sentence);
    println!("Longest word: {}", longest);
}

// Returns the longest word in the sentence
fn longest_word(sentence: &str) -> &str {
    sentence
        .split_whitespace()
        .max_by_key(|word| word.len())
        .unwrap_or("")
}