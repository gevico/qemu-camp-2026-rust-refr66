pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().to_string() + c.as_str(),
    }
}

// Step 2.
// Apply the `capitalize_first` function to a slice of string slices.
pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    // 1. Create an iterator from the slice
    // 2. Map each word using the capitalize_first function
    // 3. Collect the results into a Vector
    words.iter().map(|word| capitalize_first(word)).collect()
}

// Step 3.
// Apply the `capitalize_first` function again to a slice of string slices.
pub fn capitalize_words_string(words: &[&str]) -> String {
    // 1. Create an iterator from the slice
    // 2. Map each word using the capitalize_first function
    // 3. Collect the results into a single String
    words.iter().map(|word| capitalize_first(word)).collect()
}
