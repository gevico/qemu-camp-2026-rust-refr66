fn trim_me(input: &str) -> String {
    // TODO: Remove whitespace from both ends of a string!
    String::from(input.trim())
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There's multiple ways to do this!
    input.to_string() + " world!"
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons"!
    input.to_string().replace("cars", "balloons")
}
