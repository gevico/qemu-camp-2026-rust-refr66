#[macro_use] // This makes all macros inside the module visible to the rest of the file
mod macros {
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

// No 'use' statement is needed here!

fn main() {
    my_macro!();
}
